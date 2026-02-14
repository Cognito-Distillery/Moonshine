<p align="center">
  <img src="../src-tauri/icons/128x128.png" width="96" alt="Moonshine" />
</p>

<h1 align="center">Moonshine — Development Guide</h1>

---

## Prerequisites

An API key from **at least one** of the following AI providers is required:

| Provider | Key | Usage |
|----------|-----|-------|
| [OpenAI](https://platform.openai.com/api-keys) | `sk-...` | Embeddings + Relationship extraction |
| [Google Gemini](https://aistudio.google.com/apikey) | `AI...` | Embeddings + Relationship extraction |

> Both are **paid APIs**. Configure your key in **Settings** after launching the app.

---

## Tech Stack

```
Frontend    Svelte 5 · SvelteKit · TypeScript · Tailwind CSS 4 · DaisyUI 5
Backend     Rust · SQLite (FTS5) · reqwest · Cytoscape.js
Desktop     Tauri 2
AI          OpenAI · Gemini (embeddings + relationship extraction)
```

---

## Development

### Prerequisites

- [Bun](https://bun.sh) (or Node.js)
- [Rust toolchain](https://rustup.rs)
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/)

```bash
bun install
bun run tauri dev
```

### Build

```bash
bun run tauri build
```

### Release (for forks)

1. Generate a signing key:

```bash
bun tauri signer generate -w ~/.tauri/moonshine.key
```

2. Add the following **Repository Secrets** in GitHub (Settings > Secrets and variables > Actions):

| Secret | Value |
|--------|-------|
| `TAURI_SIGNING_PRIVATE_KEY` | Contents of the generated `.key` file |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Password used during key generation |

3. Push a version tag to trigger the build:

```bash
git tag v0.1.0
git push origin v0.1.0
```

A draft release will be created on GitHub with builds for Linux, macOS, and Windows.

---

## Project Structure

```
src/
├── routes/              # Pages (mashing, mashes, still, jar-shelf, settings, help)
├── lib/
│   ├── commands/        # Tauri IPC wrappers
│   ├── components/      # Svelte components
│   ├── graph/           # Cytoscape.js config & events
│   ├── stores/          # State management (Svelte 5 runes)
│   ├── i18n/            # Internationalization (ko, en)
│   ├── types/           # TypeScript type definitions
│   └── utils/           # Utility functions
src-tauri/
├── src/
│   ├── ai/              # Embedding generation (OpenAI, Gemini)
│   ├── commands/        # Tauri IPC command handlers
│   ├── db/              # SQLite database (mashes, edges, settings, search cache)
│   ├── pipeline/        # Auto-distillation pipeline & scheduler
│   ├── models.rs        # Data models
│   └── similarity.rs    # Cosine similarity & vector search
```
