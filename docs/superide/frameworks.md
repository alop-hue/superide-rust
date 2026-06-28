# Frameworks

## Agent Framework

The agent framework has three layers:

- Agent UI: chat panel, agent mode controls, progress timeline, and approval UI.
- Agent service: run lifecycle, context packing, tool approval, cancellation.
- Agent backend adapters: native SUPER agent, Claude Code, Codex, OpenCode,
  Blackbox, and custom process adapters.

Agent runs are event-driven:

1. User opens chat or agent mode.
2. UI emits `AiRequested`.
3. Agent service lazy-loads and resolves the configured provider/backend.
4. Context manager gathers the active file, selection, diagnostics, workspace
   roots, and explicit user attachments.
5. Agent backend emits steps.
6. Tool registry validates each tool call against workspace trust and user
   approval policy.
7. File, terminal, search, and git tools execute through existing Lapce service
   boundaries.

Initial SDK contracts live in `superide-sdk/src/agent.rs`.

## Provider Framework

Provider support is unified behind a small `AiProvider` contract. Every provider
is loaded through a registry only after AI is requested.

Initial providers:

- OpenAI
- OpenRouter
- Anthropic
- Grok
- DeepSeek
- Qwen
- GLM
- Kimi
- MiniMax
- Ollama
- Custom OpenAI-compatible APIs

Provider responsibilities:

- Validate settings.
- Initialize network or local clients lazily.
- Expose chat/tool/streaming capabilities.
- Return token usage when available.
- Convert provider-specific errors into uniform `ProviderError` values.

Initial SDK contracts live in `superide-sdk/src/provider.rs`.

## Extension Framework

SUPER inherits Lapce's WASI-oriented plugin foundation, then layers a stricter
manifest and contribution model on top.

Extension activation events:

- `OnStartupFinished`
- `OnLanguage`
- `OnCommand`
- `OnView`
- `OnAiRequested`
- `WorkspaceContains`

Contribution points:

- Commands
- Themes
- Icon themes
- Sidebar views
- Settings schemas
- Providers
- Agents

Initial SDK contracts live in `superide-sdk/src/extension.rs`.

## Event Framework

The event framework uses typed workbench events rather than stringly coupled
UI callbacks. It is intentionally small at SDK level so the runtime can choose
the actual channel, scheduler, and tracing implementation.

Initial SDK contracts live in `superide-sdk/src/event.rs`.

## Lazy AI Contract

No provider, agent backend, MCP client, model catalog, or external agent process
can initialize during default startup. The only permitted cold-start AI state is
static metadata needed to render a disabled or hidden entry point.

Activation begins from `AiRequested` and should be measurable in profiling.
