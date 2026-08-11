use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalPayload {
    pub markdown_content: String,
    pub mood_score: u8,
    pub energy_level: String,
    pub wins: Vec<String>,
    pub frictions: String,
    pub intentions: String,
    pub timestamp: String,
}

impl JournalPayload {
    pub fn to_file_string(&self) -> Result<String, serde_json::Error> {
        let json_metadata = serde_json::to_string_pretty(self)?;
        Ok(format!("---\n{}\n---\n\n{}", json_metadata, self.markdown_content))
    }

    pub fn from_file_string(content: &str) -> Result<Self, String> {
        if let Some(strip_front) = content.strip_prefix("---\n") {
            if let Some((json_str, markdown)) = strip_front.split_once("\n---\n\n") {
                let mut payload: JournalPayload =
                    serde_json::from_str(json_str).map_err(|e| e.to_string())?;
                payload.markdown_content = markdown.to_string();
                return Ok(payload);
            }
        }
        serde_json::from_str(content).map_err(|e| e.to_string())
    }
}