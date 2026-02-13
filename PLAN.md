# Session 1: Project Scaffolding - Implementation Plan

## Environment

- Node.js v22.22.0, npm 11.10.0
- Rust 1.93.1, Cargo 1.93.1
- uv 0.10.2 (Python 3.12.12 available via uv)
- Tauri CLI 2.10.0 (via npx)
- Windows 11 x86_64

## Target Structure

```
personal-aid/
├── src/                          # Vue 3 + TypeScript frontend
│   ├── App.vue
│   ├── main.ts
│   └── ...
├── backend/                      # Python FastAPI backend
│   ├── pyproject.toml
│   ├── src/
│   │   └── personal_aid/
│   │       ├── __init__.py
│   │       ├── main.py           # FastAPI app entry point
│   │       └── health.py         # Health check router
│   └── tests/
│       ├── conftest.py
│       └── test_health.py
├── src-tauri/                    # Tauri v2 (Rust)
│   ├── src/
│   │   ├── main.rs               # Sidecar lifecycle management
│   │   └── lib.rs
│   ├── binaries/                  # Sidecar binaries (PyInstaller output)
│   ├── capabilities/
│   │   └── default.json           # Shell/sidecar permissions
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── tsconfig.json
├── vite.config.ts
├── index.html
└── ...
```

## Steps

### Step 1: Scaffold Tauri v2 + Vue 3 + TypeScript

- Run `npm create tauri-app@latest` in the project directory to generate the Tauri + Vue + TS scaffold
- This creates: `src/`, `src-tauri/`, `package.json`, `vite.config.ts`, `index.html`, `tsconfig.json`
- Install frontend dependencies with `npm install`

### Step 2: Initialize Python Backend with uv

- Create `backend/` directory
- Run `uv init --lib --python 3.12` inside `backend/` to scaffold a Python project
- Pin Python 3.12 (stable, meets the 3.11+ requirement)
- Add dependencies: `fastapi`, `uvicorn[standard]`
- Add dev dependencies: `pytest`, `pytest-asyncio`, `httpx` (for testing FastAPI)
- Create `backend/src/personal_aid/main.py` with a minimal FastAPI app
- Create `backend/src/personal_aid/health.py` with `/api/health` endpoint
- Create `backend/run.py` as the sidecar entry point (runs uvicorn)

### Step 3: Configure Tauri Sidecar

- Add `tauri-plugin-shell` to `src-tauri/Cargo.toml`
- Configure `bundle.externalBin` in `tauri.conf.json` pointing to `binaries/personal-aid-api`
- Add sidecar permissions in `src-tauri/capabilities/default.json`
- Update `src-tauri/src/lib.rs` with sidecar lifecycle management:
  - `start_sidecar()` — spawns the Python process, stores child handle in app state
  - `stop_sidecar()` — kills the sidecar process
  - `check_sidecar_health()` — HTTP GET to `/api/health`
  - Auto-start on app setup, auto-stop on exit
- During **dev mode**: run `uv run python run.py` directly (no PyInstaller)
- During **prod builds**: use the PyInstaller binary from `binaries/`

### Step 4: Dev Workflow

- For development, the Python backend runs as a separate process (not as a sidecar binary)
- Add npm scripts:
  - `dev:backend` — starts the Python FastAPI server via uv
  - `dev:frontend` — starts the Tauri dev window
  - `build:sidecar` — runs PyInstaller to compile `backend/run.py` into `src-tauri/binaries/`
  - `build` — builds sidecar + Tauri app

### Step 5: Sidecar Lifecycle Tests

- **Python tests** (`backend/tests/`):
  - `test_health.py` — test `/api/health` returns 200 with expected payload
- **Rust integration** (verified manually + via `tauri dev`):
  - Sidecar starts when app launches
  - Health check succeeds after sidecar startup
  - Sidecar stops when app closes

### Step 6: Prototype Packaging (Fail-Fast)

- Install PyInstaller: `uv add --dev pyinstaller`
- Build the Python backend as a standalone `.exe`:
  ```
  uv run pyinstaller -F backend/run.py --name personal-aid-api-x86_64-pc-windows-msvc --distpath src-tauri/binaries
  ```
- Run `npx tauri build` to verify the full bundle works
- This validates the critical risk: Tauri + Python sidecar bundling

## Key Design Decisions

1. **Python 3.12** — Stable, well-supported, meets the 3.11+ requirement
2. **`backend/` as a separate uv project** — Clean separation, own virtualenv, own test suite
3. **PyInstaller single-file mode (`-F`)** — One .exe for the sidecar, simplifies bundling
4. **Dev mode runs Python directly** — No PyInstaller rebuild during development
5. **Port 18008** — Unusual port to avoid conflicts, used for frontend→backend HTTP
6. **Graceful shutdown via process kill** — Tauri kills the sidecar child process on exit; uvicorn handles SIGTERM cleanly
