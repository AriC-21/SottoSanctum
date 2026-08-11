use encoding_rs::UTF_8;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::token::data_array::LlamaTokenDataArray;
use std::path::PathBuf;

pub struct LocalBrain {
    model_path: PathBuf,
}

impl Default for LocalBrain {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalBrain {
    pub fn new() -> Self {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let model_path = PathBuf::from(home_dir)
            .join("Documents")
            .join("JournalData")
            .join("models")
            .join("Qwen3-0.6B-Q4_K_M.gguf");

        Self { model_path }
    }

    pub fn generate_response_stream<F>(&self, prompt: &str, mut on_token: F) -> Result<String, String>
    where
        F: FnMut(String) + Send + 'static,
    {
        if !self.model_path.exists() {
            return Err(format!(
                "Model file not found at {}. Download Qwen3-0.6B-Q4_K_M.gguf into that path.",
                self.model_path.display()
            ));
        }

        let backend = LlamaBackend::init().map_err(|e| e.to_string())?;
        let model_params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(&backend, &self.model_path, &model_params)
            .map_err(|e| format!("Failed to load model weights: {}", e))?;

        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(Some(std::num::NonZeroU32::new(2048).unwrap()));
        let mut ctx = model
            .new_context(&backend, ctx_params)
            .map_err(|e| format!("Failed to create context: {}", e))?;

        let formatted_prompt = format!(
            "<|im_start|>system\nYou are an empathetic personal journaling assistant. Provide concise, constructive insights and 1 actionable step based on the entry.<|im_end|>\n<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n",
            prompt
        );

        let tokens = model
            .str_to_token(&formatted_prompt, llama_cpp_2::model::AddBos::Always)
            .map_err(|e| e.to_string())?;

        let mut batch = LlamaBatch::new(2048, 1);
        let last_idx = (tokens.len() - 1) as i32;

        for (i, &token) in tokens.iter().enumerate() {
            let is_last = i as i32 == last_idx;
            batch
                .add(token, i as i32, &[0], is_last)
                .map_err(|e| e.to_string())?;
        }

        ctx.decode(&mut batch).map_err(|e| e.to_string())?;

        let mut generated_text = String::new();
        let mut n_cur = tokens.len() as i32;
        let max_tokens = n_cur + 256;

        let mut decoder = UTF_8.new_decoder();

        while n_cur < max_tokens {
            let candidates = ctx.candidates();
            let mut candidates_p = LlamaTokenDataArray::from_iter(candidates, false);
            let new_token_id = candidates_p.sample_token_greedy();

            if model.is_eog_token(new_token_id) {
                break;
            }

            let token_str = model
                .token_to_piece(new_token_id, &mut decoder, false, None)
                .map_err(|e| e.to_string())?;

            on_token(token_str.clone());
            generated_text.push_str(&token_str);

            batch.clear();
            batch
                .add(new_token_id, n_cur, &[0], true)
                .map_err(|e| e.to_string())?;

            ctx.decode(&mut batch).map_err(|e| e.to_string())?;
            n_cur += 1;
        }

        Ok(generated_text)
    }
}