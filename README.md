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

## Prerequisites

An API key from **at least one** of the following AI providers is required for embedding generation and relationship extraction:

| Provider | Key | Usage |
|----------|-----|-------|
| [OpenAI](https://platform.openai.com/api-keys) | `sk-...` | Embeddings + Relationship extraction |
| [Google Gemini](https://aistudio.google.com/apikey) | `AI...` | Embeddings + Relationship extraction |

> Both are **paid APIs**. Configure your key in **Settings** after installation.

---

## Installation

Download the latest release for your platform:

**[GitHub Releases](https://github.com/Cognito-Distillery/moonshine/releases/latest)**

| Platform | File |
|----------|------|
| **Linux** | `.AppImage` / `.deb` |
| **macOS** | `.dmg` |
| **Windows** | `.msi` / `.exe` |

> **macOS:** If you see *"moonshine is damaged and can't be opened"*, run this in Terminal:
> ```bash
> xattr -cr /Applications/moonshine.app
> ```
> This removes the quarantine flag that macOS applies to unsigned apps.

### Data Location

All data is stored in a single SQLite file:

| Platform | Path |
|----------|------|
| **Linux** | `~/.local/share/com.moonshine.app/moonshine.db` |
| **macOS** | `~/Library/Application Support/com.moonshine.app/moonshine.db` |
| **Windows** | `C:\Users\{user}\AppData\Roaming\com.moonshine.app\moonshine.db` |

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

## Development

See **[Development Guide](docs/development_guide.md)** for tech stack, build instructions, and project structure.

---

## License

[MIT](LICENSE)

---

<p align="center"><sub>under the moonlight, in silence.</sub></p>
