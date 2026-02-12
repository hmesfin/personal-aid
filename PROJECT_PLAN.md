# Project Plan: AI-Powered Personal Assistant (Windows MVP)

## Overview

Desktop application combining a chat interface with a proactive background agent. Orchestrates workflows across Gmail, Google Calendar, Signal, and local filesystem.

## Technical Stack

- **Frontend:** Tauri v2 (Rust) + Vue 3 + TypeScript
- **Backend:** Python 3.11+ sidecar (FastAPI for local HTTP, asyncio event loop)
- **AI:** Claude API via `anthropic` SDK, behind `LLMProvider` abstraction
- **Database:** SQLite (action history, audit log, file index, settings)
- **Distribution:** Tauri MSI/NSIS installer, bundled Python + JVM runtimes

## Architecture

```
┌─────────────────────────────────────┐
│         Tauri v2 Shell              │
│  ┌───────────────────────────────┐  │
│  │      Vue 3 Frontend           │  │
│  │  - Chat interface             │  │
│  │  - Notification panel         │  │
│  │  - Confirmation dialogs       │  │
│  │  - Settings UI                │  │
│  └──────────┬────────────────────┘  │
│             │ HTTP/WebSocket        │
│  ┌──────────▼────────────────────┐  │
│  │    Python Sidecar (FastAPI)   │  │
│  │  - LLM orchestration          │  │
│  │  - Gmail/Calendar client      │  │
│  │  - signal-cli wrapper         │  │
│  │  - File scanner               │  │
│  │  - Background agent (asyncio) │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

**IPC:** Python sidecar exposes a local FastAPI server on localhost. Tauri frontend communicates via HTTP + WebSocket (for real-time notifications/events).

## Phases

### Phase 1: Foundation (Sessions 1-4)
**Goal:** Project scaffolding, IPC, and core abstractions

#### Session 1: Project Scaffolding (3-4h)
- Initialize Tauri v2 project with Vue 3 + TypeScript
- Initialize Python project with `uv`, FastAPI, pytest
- Configure Tauri sidecar management for Python process
- Verify sidecar starts/stops with the app
- **Tests:** Sidecar lifecycle (start, health check, shutdown)

#### Session 2: IPC Layer (3-4h)
- FastAPI server with health endpoint
- WebSocket endpoint for real-time events (backend → frontend)
- Vue 3 composable for HTTP + WebSocket communication
- Request/response type definitions (shared schema)
- **Tests:** HTTP round-trip, WebSocket connection/reconnection, message serialization

#### Session 3: LLM Provider Abstraction (3h)
- `LLMProvider` interface (send message, tool calling, streaming)
- `ClaudeProvider` implementation using `anthropic` SDK
- Internal tool definition format + translation to Claude `tool_use`
- Conversation history management
- **Tests:** Provider interface contract tests, tool definition translation, mock provider for testing

#### Session 4: Chat Interface (3h)
- Vue 3 chat component (message list, input, markdown rendering)
- Connect to Python backend via IPC
- Basic conversation flow (user sends message → LLM responds)
- Streaming response display
- **Tests:** Component tests (message rendering, input handling), E2E chat flow with mock LLM

### Phase 2: Gmail & Calendar (Sessions 5-8)
**Goal:** Full email and calendar integration with AI orchestration

#### Session 5: Google OAuth2 (3h)
- OAuth2 flow (open browser → callback → token storage)
- Token refresh logic
- Secure storage via Windows Credential Manager (`keyring` library)
- Scopes: Gmail read/send, Calendar read/write
- **Tests:** Token lifecycle (acquire, refresh, revoke), credential storage/retrieval

#### Session 6: Gmail Integration (4h)
- Gmail API client (list messages, read message, search, labels)
- Gmail Pub/Sub push notification setup (watch endpoint)
- Polling fallback (configurable interval)
- Email parsing and normalization
- **Tests:** API client with mocked responses, push notification handling, email parsing edge cases

#### Session 7: Google Calendar Integration (3h)
- Calendar API client (list events, create event, update event)
- Event data model (title, time, participants, location, description)
- Date/time parsing utilities
- **Tests:** API client with mocked responses, date parsing (natural language → datetime)

#### Session 8: Email/Calendar AI Tools (4h)
- LLM tool: `read_emails` (search, filter, summarize)
- LLM tool: `draft_email` (compose response, new email)
- LLM tool: `create_calendar_event` (extract details from context)
- LLM tool: `list_calendar_events` (query upcoming events)
- Human-in-the-loop: draft → confirm → execute flow
- **Tests:** Tool execution with mock LLM, confirmation flow, email→calendar extraction

### Phase 3: Signal Integration (Sessions 9-10)
**Goal:** Send/receive Signal messages with AI summarization

#### Session 9: signal-cli Wrapper (4h)
- signal-cli daemon mode management (start, stop, health check)
- Message receiving (daemon JSON-RPC or dbus interface)
- Message sending
- Contact/group resolution
- JVM runtime detection and management
- **Tests:** Message send/receive with mock signal-cli, daemon lifecycle, error handling

#### Session 10: Signal AI Tools (3h)
- LLM tool: `read_signal_messages` (unread, by contact, by group)
- LLM tool: `send_signal_message` (with confirmation)
- LLM tool: `summarize_signal_conversations` (unread summary)
- Notification: "X unread messages from Y"
- **Tests:** Tool execution with mock signal-cli, conversation summarization, notification generation

### Phase 4: File Organization (Sessions 11-12)
**Goal:** AI-powered file scanning, categorization, and organization

#### Session 11: File Scanner (3-4h)
- Directory scanner (configurable root directories)
- File metadata extraction (size, type, modified date, path)
- `watchdog` filesystem monitoring for real-time changes
- File index stored in SQLite
- **Tests:** Scanner accuracy, watchdog event handling, index CRUD operations

#### Session 12: File Organization AI Tools (3-4h)
- LLM tool: `scan_files` (list files with metadata)
- LLM tool: `categorize_files` (LLM-based categorization)
- LLM tool: `recommend_organization` (suggest moves with rationale)
- LLM tool: `recommend_deletions` (with confidence scores, threshold filtering)
- Execute moves/deletes with confirmation
- **Tests:** Categorization accuracy with mock LLM, confidence thresholds, move/delete operations on temp filesystem

### Phase 5: Background Agent & Orchestration (Sessions 13-14)
**Goal:** Proactive agent, cross-feature workflows, notification system

#### Session 13: Event System & Background Agent (4h)
- Event bus (asyncio-based, internal pub/sub)
- Event sources: Gmail push, signal-cli daemon, watchdog, polling scheduler
- Agent loop: receive event → evaluate → generate suggestion/action
- Notification model (type, priority, content, suggested actions)
- **Tests:** Event routing, agent decision logic with mock events, notification generation

#### Session 14: Workflow Orchestration & Notifications (4h)
- Cross-integration workflows (email → calendar, Signal context → email draft)
- Notification panel in Vue 3 frontend (WebSocket-driven)
- Action queue (pending confirmations displayed in UI)
- Action history / audit log in SQLite
- **Tests:** Multi-step workflow execution, notification delivery, audit log integrity

### Phase 6: Packaging & Distribution (Sessions 15-16)
**Goal:** Installable Windows application

#### Session 15: Packaging (4h)
- Bundle Python runtime (embedded Python or PyInstaller)
- Bundle JVM for signal-cli
- Tauri MSI/NSIS build configuration
- First-run setup wizard (Google OAuth, Signal linking, directory config)
- **Tests:** Installer build succeeds, sidecar starts from installed location, first-run flow

#### Session 16: Polish & Hardening (3-4h)
- Error handling and graceful degradation across all integrations
- Logging (structured, rotated, queryable)
- Settings persistence and UI
- E2E smoke tests for all major workflows
- **Tests:** Error scenarios (API down, signal-cli crash, network loss), settings persistence

## Total Estimates

- **Total Sessions:** 16
- **Estimated Time:** 52-60 hours
- **Phases:** 6

## Session Dependencies

```
Session 1 (Scaffolding)
  └→ Session 2 (IPC)
       ├→ Session 3 (LLM Provider)
       │    └→ Session 4 (Chat UI)
       │         └→ Session 8 (Email/Calendar AI Tools)
       │         └→ Session 10 (Signal AI Tools)
       │         └→ Session 12 (File Org AI Tools)
       │         └→ Session 14 (Workflow Orchestration)
       ├→ Session 5 (Google OAuth2)
       │    ├→ Session 6 (Gmail)
       │    └→ Session 7 (Calendar)
       ├→ Session 9 (signal-cli Wrapper)
       └→ Session 11 (File Scanner)

Session 13 (Event System) depends on Sessions 6, 9, 11
Session 14 (Orchestration) depends on Sessions 8, 10, 12, 13
Session 15 (Packaging) depends on all above
Session 16 (Polish) depends on Session 15
```

**Parallelizable:** Sessions 5-7 (Google) can run in parallel with Session 9 (Signal) and Session 11 (File Scanner), once IPC is done.

## Testing Strategy

- **TDD:** Tests written before implementation in every session
- **Unit Tests:** pytest (Python), Vitest (Vue/TypeScript)
- **Integration Tests:** Mocked external APIs (Gmail, Calendar, signal-cli)
- **E2E Tests:** Full workflow tests with mock LLM and mock services
- **Coverage Target:** 85%
- **Mock LLM:** Deterministic test responses for all tool-calling scenarios

## Success Criteria

- [ ] Chat interface sends messages and receives streamed LLM responses
- [ ] Gmail: read, search, draft, send (with confirmation)
- [ ] Calendar: create events from email/Signal context (with confirmation)
- [ ] Signal: read, send, summarize unread (with confirmation)
- [ ] Files: scan, categorize, recommend org/deletions (with confirmation)
- [ ] Background agent surfaces proactive suggestions
- [ ] Cross-integration workflows (email → calendar, etc.)
- [ ] Installable via MSI on Windows
- [ ] All actions have human-in-the-loop confirmation
- [ ] 85%+ test coverage

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| signal-cli breaks on protocol update | Signal feature unusable | Isolate behind interface; feature can be disabled independently |
| JVM adds ~150MB to installer | Large download | Consider GraalVM native-image for signal-cli in future |
| Google "unverified app" warning | Confusing UX for users | Document in setup guide; apply for verification if group grows |
| Proactive agent too noisy | User disables notifications | Configurable aggressiveness, per-source controls, quiet hours |
| File deletion false positives | Data loss | High confidence threshold, mandatory confirmation, recycle bin (no permanent delete) |
| Tauri v2 sidecar Python bundling | Complex packaging | Prototype packaging in Session 1, fail fast |
