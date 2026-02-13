<p align="center">
  <img src="src-tauri/icons/128x128.png" width="96" alt="Moonshine" />
</p>

<h1 align="center">Moonshine</h1>

<p align="center">
  <em>Distill thoughts into aged knowledge.</em>
</p>

<p align="center">
  <strong><a href="docs/README_ko.md">한국어</a></strong>
</p>

---

```
  Mashing       Mash Tun         Still         Pipeline        Jar Shelf
  ───────      ──────────      ───────       ──────────      ───────────
 [ Idea ] ──→ [  Store  ] ──→ [ Select ] ──→ [ Distill ] ──→ [ Explore ]
               ↑                   │
               └──── Take Off ─────┘
```

Moonshine is a standalone desktop app that **refines raw thoughts into a knowledge graph**,
borrowing the craft of moonshining as its metaphor.

Capture scattered decisions, problems, insights, and questions as **mashes**.
Place them on the **still** to curate.
The pipeline automatically distills them — generating embeddings, extracting relationships —
and stores the results in mason jars on the **jar shelf**: your knowledge graph.

No server required. Everything runs locally on your machine.

---

## The Process

### 1. Mashing

Turn a thought into a mash. Choose one of four types:

| Type | Description |
|------|-------------|
| **Decision** | A finalized decision |
| **Problem** | An issue that needs resolution |
| **Insight** | A discovered insight |
| **Question** | A question that needs discussion |

### 2. Mash Tun

Your warehouse of mashes. Search, edit, or delete them.

### 3. Still

Curate which mashes to distill. The pipeline runs automatically on a configurable interval,
processing embeddings and extracting relationships via AI.

### 4. Jar Shelf

Explore the knowledge graph of your distilled mashes.
Search by keyword or natural language, view connections, and edit relationships.
Recent natural-language searches are cached to avoid redundant API calls.

---

## Floating Memo (Quick Mash)

A small floating window for capturing thoughts instantly without switching to the main app.
The app stays in the **system tray** — close the main window and it keeps running in the background.

| Platform | Shortcut | Setup |
|----------|----------|-------|
| **macOS** | `Cmd+Shift+M` | Works out of the box |
| **Windows** | `Ctrl+Shift+M` | Works out of the box |
| **Linux (X11)** | `Ctrl+Shift+M` | Works out of the box |
| **Linux (Wayland)** | Any key you choose | Bind a DBus command in your compositor (see below) |

### Linux Wayland Setup

Wayland does not support app-level global shortcuts.
The app exposes a DBus service instead — bind the following command in your compositor/DE settings:

```bash
dbus-send --session --type=method_call --dest=com.moonshine.App /com/moonshine/App com.moonshine.App.ToggleFloatingMemo
```

<details>
<summary>niri</summary>

Add to your `config.kdl` inside the `binds` block:

```kdl
Ctrl+Shift+M { spawn "dbus-send" "--session" "--type=method_call" "--dest=com.moonshine.App" "/com/moonshine/App" "com.moonshine.App.ToggleFloatingMemo"; }
```

Then reload: `niri msg action reload-config`
</details>

<details>
<summary>Hyprland</summary>

Add to `hyprland.conf`:

```
bind = CTRL SHIFT, M, exec, dbus-send --session --type=method_call --dest=com.moonshine.App /com/moonshine/App com.moonshine.App.ToggleFloatingMemo
```
</details>

<details>
<summary>GNOME</summary>

```bash
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/moonshine/']"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/moonshine/ name 'Moonshine Floating Memo'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/moonshine/ command "dbus-send --session --type=method_call --dest=com.moonshine.App /com/moonshine/App com.moonshine.App.ToggleFloatingMemo"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/moonshine/ binding '<Ctrl><Shift>m'
```
</details>

<details>
<summary>KDE Plasma</summary>

System Settings > Shortcuts > Custom Shortcuts > Add new shortcut with:

- Trigger: `Ctrl+Shift+M`
- Command: `dbus-send --session --type=method_call --dest=com.moonshine.App /com/moonshine/App com.moonshine.App.ToggleFloatingMemo`
</details>

---

## Tech Stack

```
Frontend    Svelte 5 · SvelteKit · TypeScript · Tailwind CSS 4 · DaisyUI 5
Backend     Rust · SQLite (FTS5) · reqwest · Cytoscape.js
Desktop     Tauri 2
AI          OpenAI · Gemini (embeddings + relationship extraction)
```

---

## Getting Started

### Prerequisites

- [Bun](https://bun.sh) (or Node.js)
- [Rust toolchain](https://rustup.rs)
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/)

### Install

```bash
bun install
```

### Develop

```bash
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

---

## License

[MIT](LICENSE)

---

<p align="center"><sub>under the moonlight, in silence.</sub></p>
