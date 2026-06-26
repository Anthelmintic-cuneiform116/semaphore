# Semaphore

Floating traffic light for AI coding agents. Know at a glance when your agent is idle, thinking, or writing files.

Green = ready for a new task  
Yellow = thinking  
Red = writing / editing files

## One app for end users

Install Semaphore, open it once, connect your tools from Settings — no terminal required.

1. Download a release for your OS (or build from source)
2. Launch Semaphore (stays in the system tray)
3. Open Settings → **Connect tools**
4. Use your AI tools normally

Hooks call `semctl` in the background. You never run it manually.

## Supported tools (v0.1)

| Tool | Status | Install |
|------|--------|---------|
| Cursor | Supported | `semctl install cursor` |
| Claude Code | Supported | `semctl install claude-code` |
| Codex CLI | Supported (Bash hooks; file edit limited) | `semctl install codex` |
| Gemini CLI | Supported | `semctl install gemini-cli` |
| Copilot CLI | Best-effort (varies by version) | `semctl install copilot-cli` |

```bash
semctl install --all
semctl doctor
```

See [adapters/README.md](adapters/README.md) for per-tool hook mapping.

## Development

Requirements: Rust, Node.js 20+, npm.

```bash
npm install
npm run tauri dev
```

Build CLI:

```bash
cargo build -p semctl --release
```

## Architecture

```
AI tool hooks → sem-hook → semctl → Unix socket / named pipe → Semaphore app
```

- **sem-core** — state machine, session aggregation, IPC
- **semctl** — CLI for hooks and installer
- **semaphore** (Tauri) — floating UI, tray, settings

## Themes & i18n

Built-in themes: Classic, Minimal, Neon (`src/themes/*.json`). English (default) and Portuguese. See `locales/CONTRIBUTING-i18n.md`.

## Stealth mode

Hides the window from many screen-capture tools. Works best on Windows; macOS 15+ may still capture in some apps.

## License

MIT — see [LICENSE](LICENSE).
