# Folder Structure

## Current Foundation

```text
super-foundation/
  lapce-app/          desktop UI, workbench, panels, settings, themes
  lapce-core/         editor core, syntax, movement, buffer primitives
  lapce-proxy/        terminal, LSP, filesystem, plugin, and remote bridge
  lapce-rpc/          protocol types shared by app and proxy
  superide-sdk/       SUPER public SDK contracts
  defaults/           default settings, themes, keymaps, run config
  docs/superide/      SUPER architecture and roadmap docs
  extra/images/       product and platform assets
  icons/              codicons and Lapce/SUPER UI icon sources
```

## Target Additions

```text
lapce-foundation/
  superide-ai/
    src/provider_service.rs
    src/providers/
      openai.rs
      openrouter.rs
      anthropic.rs
      grok.rs
      deepseek.rs
      qwen.rs
      glm.rs
      kimi.rs
      minimax.rs
      ollama.rs
      openai_compatible.rs
    src/model_catalog.rs
    src/token_usage.rs

  superide-agent/
    src/agent_service.rs
    src/context.rs
    src/tool_registry.rs
    src/tools/
      file.rs
      terminal.rs
      search.rs
      git.rs
    src/adapters/
      claude_code.rs
      codex.rs
      opencode.rs
      blackbox.rs

  superide-mcp/
    src/client.rs
    src/server_registry.rs
    src/tool_bridge.rs
    src/permissions.rs

  superide-extension-host/
    src/activation.rs
    src/contributions.rs
    src/host.rs
    src/manifest.rs
    src/permissions.rs
```

## Migration Rule

Do not rename every `Lapce*` internal type during MVP work. Rename visible UI,
packaging, docs, default settings, and new SUPER crates first. Internal protocol
renames should happen after the MVP compiles and after downstream paths, config
folders, plugin IDs, and update channels are intentionally migrated.
