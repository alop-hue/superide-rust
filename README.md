<h1 align="center">
  <img src="extra/images/superide/super_logo.svg" width=120 height=120/><br>
  SUPER IDE
</h1>

<h4 align="center">Lightning-fast AI-Powered Code Editor</h4>

<div align="center">
  <img src="https://img.shields.io/badge/built%20on-Lapce-orange" alt="Built on Lapce"/>
  <img src="https://img.shields.io/badge/language-Rust-orange" alt="Language"/>
</div>
<br/>

**SUPER IDE** is a lightning-fast, AI-powered code editor built on the foundation of [Lapce](https://github.com/lapce/lapce). It combines Lapce's high-performance Rust architecture with deep AI integration for an intelligent coding experience.

Built in pure Rust with a UI in [Floem](https://github.com/lapce/floem). Designed with [Rope Science](https://xi-editor.io/docs/rope_science_00.html) from the [Xi-Editor](https://github.com/xi-editor/xi-editor), enabling lightning-fast computation, and leverages [wgpu](https://github.com/gfx-rs/wgpu) for rendering.

## Features

- **🧠 6 AI Modes** — Chat, Edit, Plan, Review, Debug, and Research — each a specialized AI agent that understands your entire project
- **🔌 Multi-Provider** — OpenAI, Anthropic, OpenRouter, Ollama (local), DeepSeek, Qwen, and Grok (xAI) — all configurable from one settings panel
- **📋 Agent Tool System** — AI can read files, search code, edit text, run terminal commands, and browse the web autonomously
- **⚡ Blazing Rust Performance** — Built on [Floem](https://github.com/lapce/floem) UI framework with [Rope Science](https://xi-editor.io/docs/rope_science_00.html) and [wgpu](https://github.com/gfx-rs/wgpu) rendering
- **🎨 Beautiful Custom Themes** — Dynamic gradient and neon glow themes included. Full TOML theming engine for endless customization
- **🔌 LSP Support** — Language Server Protocol for completion, diagnostics, code actions, and more
- **⌨️ Modal Editing** — Vim-like modal editing as a first-class citizen (toggleable)
- **🌐 Remote Development** — Built-in remote development support
- **🧩 Plugin System** — Plugins in WASI-compilable languages (C, Rust, AssemblyScript)
- **💻 Built-in Terminal** — Execute commands in your workspace without leaving the IDE

## Building from Source

### Prerequisites

- Rust toolchain (latest stable)
- Required system libraries (Linux): `libssl-dev`, `libfontconfig-dev`

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## Configuration

Settings are available via `Open Settings` in the command palette. AI providers can be configured in the AI Chat panel settings.

## Contributing

SUPER IDE is built on Lapce and welcomes contributions. Guidelines can be found in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

Apache License Version 2. See [`LICENSE`](LICENSE).
