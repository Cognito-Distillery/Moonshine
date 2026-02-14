<p align="center">
  <img src="../src-tauri/icons/128x128.png" width="96" alt="Moonshine" />
</p>

<h1 align="center">Moonshine</h1>

<p align="center">
  <em>생각을 증류하여 숙성된 지식으로.</em>
</p>

<p align="center">
  <strong><a href="../README.md">English</a></strong>
</p>

---

```
  매싱          매시 턴          스틸         파이프라인         찬장
  ─────       ─────────      ─────       ──────────      ─────────
 [ 생각 ] ──→ [  보관  ] ──→ [ 선별 ] ──→ [  증류  ] ──→ [ 탐색  ]
               ↑                  │
               └──── 내리기 ──────┘
```

Moonshine은 **흩어진 생각을 지식 그래프로 정제하는** 독립형 데스크톱 앱입니다.
밀주(Moonshine) 제조 과정을 메타포로 사용합니다.

결정, 문제, 인사이트, 질문을 **매시**로 기록하고,
**스틸**에 올려 선별하세요.
파이프라인이 자동으로 임베딩을 생성하고 관계를 추출하여
**찬장**(지식 그래프)에 저장합니다.

서버 불필요. 모든 것이 로컬에서 동작합니다.

---

## 사전 요구사항

임베딩 생성과 관계 추출을 위해 아래 AI 제공자 중 **최소 하나**의 API 키가 필요합니다:

| 제공자 | 키 | 용도 |
|--------|-----|------|
| [OpenAI](https://platform.openai.com/api-keys) | `sk-...` | 임베딩 + 관계 추출 |
| [Google Gemini](https://aistudio.google.com/apikey) | `AI...` | 임베딩 + 관계 추출 |

> 모두 **유료 API**입니다. 설치 후 **설정**에서 키를 입력하세요.

---

## 설치

플랫폼에 맞는 최신 릴리스를 다운로드하세요:

**[GitHub Releases](https://github.com/Cognito-Distillery/moonshine/releases/latest)**

| 플랫폼 | 파일 |
|--------|------|
| **Linux** | `.AppImage` / `.deb` |
| **macOS** | `.dmg` |
| **Windows** | `.msi` / `.exe` |

> **macOS:** *"moonshine이(가) 손상되었기 때문에 열 수 없습니다"* 메시지가 나타나면 터미널에서 다음을 실행하세요:
> ```bash
> xattr -cr /Applications/moonshine.app
> ```
> macOS가 서명되지 않은 앱에 적용하는 격리 플래그를 제거합니다.

### 데이터 저장 위치

모든 데이터는 단일 SQLite 파일에 저장됩니다:

| 플랫폼 | 경로 |
|--------|------|
| **Linux** | `~/.local/share/com.moonshine.app/moonshine.db` |
| **macOS** | `~/Library/Application Support/com.moonshine.app/moonshine.db` |
| **Windows** | `C:\Users\{user}\AppData\Roaming\com.moonshine.app\moonshine.db` |

---

## 과정

### 1. 매싱 (Mashing)

생각을 매시로 만듭니다. 네 가지 유형 중 하나를 선택하세요:

| 유형 | 설명 |
|------|------|
| **결정** | 확정된 의사결정 |
| **문제** | 해결이 필요한 이슈 |
| **인사이트** | 발견한 통찰 |
| **질문** | 논의가 필요한 질문 |

### 2. 매시 턴 (Mash Tun)

매시를 보관하는 곳입니다. 검색, 수정, 삭제가 가능합니다.

### 3. 스틸 (Still)

증류할 매시를 선별합니다. 파이프라인이 설정한 주기에 따라 자동 실행되며,
AI를 통해 임베딩 생성과 관계 추출을 처리합니다.

### 4. 찬장 (Jar Shelf)

증류 완료된 매시의 지식 그래프를 탐색합니다.
키워드 또는 자연어로 검색하고, 연결을 확인하고, 관계를 편집할 수 있습니다.
최근 자연어 검색은 캐시되어 불필요한 API 호출을 방지합니다.

---

## 플로팅 메모 (빠른 매싱)

메인 앱으로 전환하지 않고 즉시 생각을 기록할 수 있는 작은 플로팅 윈도우입니다.
앱은 **시스템 트레이**에 상주하며, 메인 윈도우를 닫아도 백그라운드에서 계속 실행됩니다.

| 플랫폼 | 단축키 | 설정 |
|--------|--------|------|
| **macOS** | `Cmd+Shift+M` | 별도 설정 불필요 |
| **Windows** | `Ctrl+Shift+M` | 별도 설정 불필요 |
| **Linux (X11)** | `Ctrl+Shift+M` | 별도 설정 불필요 |
| **Linux (Wayland)** | 원하는 키 지정 | 컴포지터에서 DBus 명령 바인딩 (아래 참고) |

### Linux Wayland 설정

Wayland는 앱 수준의 전역 단축키를 지원하지 않습니다.
대신 DBus 서비스를 노출하므로, 컴포지터/DE 설정에서 다음 명령을 바인딩하세요:

```bash
dbus-send --session --type=method_call --dest=com.moonshine.App /com/moonshine/App com.moonshine.App.ToggleFloatingMemo
```

<details>
<summary>niri</summary>

`config.kdl`의 `binds` 블록에 추가:

```kdl
Ctrl+Shift+M { spawn "dbus-send" "--session" "--type=method_call" "--dest=com.moonshine.App" "/com/moonshine/App" "com.moonshine.App.ToggleFloatingMemo"; }
```

리로드: `niri msg action reload-config`
</details>

<details>
<summary>Hyprland</summary>

`hyprland.conf`에 추가:

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

시스템 설정 > 단축키 > 사용자 정의 단축키 > 새 단축키 추가:

- 트리거: `Ctrl+Shift+M`
- 명령: `dbus-send --session --type=method_call --dest=com.moonshine.App /com/moonshine/App com.moonshine.App.ToggleFloatingMemo`
</details>

---

## 개발

**[개발 가이드](development_guide_ko.md)**에서 기술 스택, 빌드 방법, 프로젝트 구조를 확인하세요.

---

## 라이선스

[MIT](../LICENSE)

---

<p align="center"><sub>달빛 아래, 고요히.</sub></p>
