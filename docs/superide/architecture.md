# SUPER IDE Architecture Plan

## Product Shape

SUPER IDE is a Rust-first desktop IDE that starts as a normal editor and adds AI
only when requested. The default startup path must initialize the shell, editor,
workspace metadata, file explorer, settings, command registry, and theme system.
It must not initialize AI providers, agent backends, model catalogs, MCP clients,
or remote API clients on cold start.

The workflow should feel familiar to VS Code users: activity bar, explorer,
search, source control, terminal, extensions, settings, command palette, tabs,
 and split editor panes. The implementation should preserve Lapce's performance
advantages instead of recreating an Electron-style architecture.

## Lapce Foundation Mapping

| SUPER Capability | Lapce Foundation |
| --- | --- |
| App shell and windows | `lapce-app/src/app.rs`, `window.rs`, `window_tab.rs` |
| Editor and buffers | `lapce-app/src/editor*`, `doc.rs`, `lapce-core` |
| File explorer | `lapce-app/src/file_explorer` |
| Search | `lapce-app/src/global_search.rs`, `panel/global_search_view.rs` |
| Terminal | `lapce-app/src/terminal`, `panel/terminal_view.rs` |
| Git/source control | `lapce-app/src/source_control.rs`, `panel/source_control_view.rs` |
| Settings | `lapce-app/src/settings.rs`, `config/*`, `defaults/settings.toml` |
| Themes | `defaults/*theme.toml`, `config/color_theme.rs` |
| Plugins | `lapce-app/src/plugin.rs`, `lapce-proxy/src/plugin`, `lapce-rpc` |
| RPC/proxy | `lapce-rpc`, `lapce-proxy` |

## Target Workspace

SUPER stays compatible with Lapce internals while adding product-specific crates.
The migration path is additive first, rename-heavy second.

Core crates:

- `lapce-core`: text, movement, syntax, and low-level editor primitives.
- `lapce-app`: desktop UI and workbench composition.
- `lapce-proxy`: filesystem, LSP, terminal, plugin, and remote-process bridge.
- `lapce-rpc`: shared protocol types.
- `superide-sdk`: public contracts for events, providers, agents, and extensions.
- `superide-ai`: lazy AI service runtime. Planned.
- `superide-agent`: native agent loop and external agent adapters. Planned.
- `superide-mcp`: MCP client runtime. Planned.

## Event-Driven Design

SUPER uses an event bus to decouple UI actions, core services, providers,
agents, and extensions. User actions enter through the command registry, produce
typed events, and update focused stores. Services subscribe only to the topics
they need.

Primary event domains:

- `workspace.*`: open, close, roots changed, trust changed.
- `file.*`: open, save, rename, delete, dirty state.
- `editor.*`: cursor, selection, diagnostics, active document.
- `terminal.*`: session created, output, exit, task finished.
- `git.*`: status changed, branch changed, diff requested.
- `extension.*`: manifest loaded, activation requested, command contributed.
- `provider.*`: provider selected, initialized, failed, usage reported.
- `agent.*`: run started, step produced, tool requested, run finished.

Cold startup subscribes core services only. AI subscriptions are registered when
`AiRequested` fires from chat, agent mode, inline completion, or provider
settings.

## Service Boundaries

Startup services:

- Workspace service
- Settings service
- Theme service
- Command registry
- Event bus
- State store
- Editor service
- File explorer service
- Search service
- Terminal service
- Source control service

Lazy services:

- Provider service
- Agent service
- MCP service
- External agent adapters
- Model catalog refresh
- Embedding/indexing service
- Usage and cost tracker

## Performance Policy

- AI code paths are feature-gated and service-lazy.
- Provider clients are constructed only after provider selection.
- Model catalogs refresh in the background after AI UI opens.
- Workspace indexing starts as plain text search, then becomes incremental.
- Extension activation follows explicit activation events, never eager global
  activation.
- Startup measurement must track time to first editable buffer and resident RAM
  after idle stabilization.

## Security Policy

- Secrets live in OS credential storage when available.
- Workspace trust gates terminal tools, file writes, MCP tools, and external
  agents.
- Agent tools are explicit capabilities, not arbitrary host access.
- MCP servers run behind per-workspace approval and timeout limits.
- Extensions run through the existing WASI-oriented Lapce plugin boundary first;
  native extensions require a separate signing and permission model.
