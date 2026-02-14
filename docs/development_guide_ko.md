<p align="center">
  <img src="../src-tauri/icons/128x128.png" width="96" alt="Moonshine" />
</p>

<h1 align="center">Moonshine — 개발 가이드</h1>

---

## 사전 요구사항

아래 AI 제공자 중 **최소 하나**의 API 키가 필요합니다:

| 제공자 | 키 | 용도 |
|--------|-----|------|
| [OpenAI](https://platform.openai.com/api-keys) | `sk-...` | 임베딩 + 관계 추출 |
| [Google Gemini](https://aistudio.google.com/apikey) | `AI...` | 임베딩 + 관계 추출 |

> 모두 **유료 API**입니다. 앱 실행 후 **설정**에서 키를 입력하세요.

---

## 기술 스택

```
프론트엔드    Svelte 5 · SvelteKit · TypeScript · Tailwind CSS 4 · DaisyUI 5
백엔드       Rust · SQLite (FTS5) · reqwest · Cytoscape.js
데스크톱     Tauri 2
AI          OpenAI · Gemini (임베딩 + 관계 추출)
```

---

## 개발

### 사전 요구사항

- [Bun](https://bun.sh) (또는 Node.js)
- [Rust 툴체인](https://rustup.rs)
- [Tauri 2 사전 요구사항](https://v2.tauri.app/start/prerequisites/)

```bash
bun install
bun run tauri dev
```

### 빌드

```bash
bun run tauri build
```

### 릴리스 (포크용)

1. 서명 키 생성:

```bash
bun tauri signer generate -w ~/.tauri/moonshine.key
```

2. GitHub 저장소 시크릿 추가 (Settings > Secrets and variables > Actions):

| 시크릿 | 값 |
|--------|-----|
| `TAURI_SIGNING_PRIVATE_KEY` | 생성된 `.key` 파일 내용 |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | 키 생성 시 사용한 비밀번호 |

3. 버전 태그를 푸시하여 빌드 트리거:

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub에 Linux, macOS, Windows 빌드가 포함된 드래프트 릴리스가 생성됩니다.

---

## 프로젝트 구조

```
src/
├── routes/              # 페이지 (매싱, 매시 턴, 스틸, 찬장, 설정, 도움말)
├── lib/
│   ├── commands/        # Tauri IPC 래퍼
│   ├── components/      # Svelte 컴포넌트
│   ├── graph/           # Cytoscape.js 설정 & 이벤트
│   ├── stores/          # 상태 관리 (Svelte 5 runes)
│   ├── i18n/            # 다국어 지원 (한국어, 영어)
│   ├── types/           # TypeScript 타입 정의
│   └── utils/           # 유틸리티 함수
src-tauri/
├── src/
│   ├── ai/              # 임베딩 생성 (OpenAI, Gemini)
│   ├── commands/        # Tauri IPC 커맨드 핸들러
│   ├── db/              # SQLite 데이터베이스 (매시, 엣지, 설정, 검색 캐시)
│   ├── pipeline/        # 자동 증류 파이프라인 & 스케줄러
│   ├── models.rs        # 데이터 모델
│   └── similarity.rs    # 코사인 유사도 & 벡터 검색
```
