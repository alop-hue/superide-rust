# Roadmap

## MVP Roadmap

Goal: SUPER IDE is usable as a normal IDE with a branded shell and no AI cost on
startup.

1. Foundation and brand
   - Keep latest Lapce checkout as the base.
   - Add SUPER theme and product strings.
   - Generate platform icons and update package metadata.

2. Core workbench
   - Validate editor, explorer, search, terminal, source control, settings, and
     workspace flows under SUPER defaults.
   - Establish startup and memory benchmarks against VS Code on the same sample
     projects.
   - Keep AI services disabled and uninitialized.

3. SDKs and service shells
   - Land `superide-sdk`.
   - Add runtime event bus, provider service shell, and agent service shell.
   - Wire `AiRequested` without loading a provider by default.

4. First AI surface
   - Add chat panel.
   - Implement OpenAI-compatible provider path.
   - Add OpenRouter and Ollama after the provider abstraction is stable.
   - Persist provider settings with secrets outside plain settings when possible.

5. First agent
   - Implement native read/search/edit tool loop.
   - Add workspace trust gates and approval UI.
   - Add Codex and Claude Code adapters after native tool contracts settle.

MVP exit criteria:

- App opens a workspace and edits files normally.
- Default theme and visible branding say SUPER IDE.
- Startup path initializes no AI clients.
- Chat works with at least one provider after explicit user action.
- Agent can propose and apply file edits behind approval.

## Production Roadmap

1. Packaging and distribution
   - Complete Linux, macOS, and Windows package metadata.
   - Add signing, notarization, update channel, and release automation.
   - Decide config directory migration from Lapce-compatible paths to SUPER paths.

2. Performance hardening
   - Add startup and resident-memory CI benchmarks.
   - Profile large workspaces.
   - Move indexing, model catalogs, and extension scans behind budgets.
   - Track memory per service.

3. Provider depth
   - Add Anthropic, Grok, DeepSeek, Qwen, GLM, Kimi, MiniMax, and custom
     OpenAI-compatible provider support.
   - Add streaming, tool calls, structured outputs where providers support them,
     and usage accounting.

4. Agent ecosystem
   - Add Claude Code, Codex, OpenCode, and Blackbox adapters.
   - Add MCP client with per-server permissions.
   - Add reproducible agent transcripts and run export.

5. Extension ecosystem
   - Stabilize manifest schema.
   - Publish Plugin SDK, Provider SDK, and Agent SDK docs.
   - Add extension sandbox permissions, marketplace policy, and compatibility
     tests.

6. Enterprise readiness
   - Workspace trust policy.
   - Provider allowlists and local-only mode.
   - Secret storage integrations.
   - Audit logs for agent actions.
   - Offline extension and model configuration.
