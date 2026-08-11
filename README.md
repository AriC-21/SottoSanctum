# SottoSanctum

> *From the Latin sub rosa (under the rose) and sanctum (sacred place) - an air-gapped, local-first AI journaling app.*

> ⚠️ **Project Status: Under Active Build**
> SottoSanctum is currently under active development. Core features, model integrations, and IPC interfaces are continuously being updated and refined.

---

## Key Features

* **Zero-Cloud Privacy:** All journal entries, metrics, and AI inference run 100% locally on your machine.
* **On-Device SLM Inference:** Integrates directly with `llama.cpp` via native Rust bindings (`llama_cpp_2`) to execute quantized GGUF models (`Qwen3-0.6B`).
* **Real-time Token Streaming:** Uses Tauri v2 IPC channels (`Channel<String>`) offloaded onto background threads (`tokio::task::spawn_blocking`) for smooth, non-blocking UI rendering.
* **Dual-Panel Workspace:**
* **Left:** Minimalist Markdown editor with word count tracking and live rendered preview toggles.
* **Right:** Structured Daily Dashboard tracking Mood, Energy levels, dynamic Wins, Frictions, and Intentions.


* **Markdown File Persistence:** Journal sessions are stored as clean `.md` files with YAML/JSON frontmatter inside `~/Documents/JournalData/`.
* **History Drawer:** Slide-out access to past entries for quick review and reload.

---

## Tech Stack

* **Frontend:** [Svelte](https://svelte.dev/), TypeScript, Vite
* **Desktop Shell & IPC:** [Tauri v2](https://v2.tauri.app/), Rust
* **AI Engine:** [`llama_cpp_2`](https://www.google.com/search?q=https://github.com/eagletai/llama-cpp-rs) (C++ bindings for GGUF model execution)
* **Async Runtime:** [Tokio](https://tokio.rs/)

---

## Prerequisites

1. **Rust Toolchain:** Installed via [rustup](https://rustup.rs/).
2. **Node.js:** v18+ and `npm`.
3. **C++ Compiler:** `clang` / `gcc` / MSVC required by `llama.cpp` to build C++ native bindings.
4. **Local SLM Weights:** Download `Qwen3-0.6B-Q4_K_M.gguf` (or any compatible GGUF model).

---

## Setup & Installation

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/SottoSanctum.git
cd SottoSanctum

```

### 2. Install Frontend Dependencies

```bash
npm install

```

### 3. Setup Model Storage Directory

Create the target model directory in your local user documents and place the GGUF model file inside it:

```bash
mkdir -p ~/Documents/JournalData/models

```

Download `Qwen3-0.6B-Q4_K_M.gguf` and place it at:

```text
~/Documents/JournalData/models/Qwen3-0.6B-Q4_K_M.gguf

```

---

## Development

Start the Tauri development environment with live reloading:

```bash
npm run tauri dev

```

---

## Project Structure

```text
SottoSanctum/
├── src/                      # Svelte Frontend
│   ├── App.svelte            # Main App layout & state management
│   └── main.ts               # Svelte entrypoint
└── src-tauri/                # Tauri Rust Backend
    ├── Cargo.toml            # Rust dependencies & lib configuration
    ├── tauri.conf.json       # Tauri v2 application configuration
    └── src/
        ├── main.rs           # Binary entry point
        ├── lib.rs            # Core Tauri initialization & AppState
        ├── ai.rs             # LocalBrain struct & llama_cpp_2 stream generator
        ├── commands.rs       # Tauri IPC commands (save, load, stream_analyze)
        └── models.rs         # JournalPayload struct & serialization logic

```

---

## Building for Production

To create a release binary with optimized native C++ inference:

```bash
npm run tauri build

```

---

## License

MIT

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
