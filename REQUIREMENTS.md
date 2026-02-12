# AI-Powered Personal Assistant - Requirements

## Product Overview
An AI-powered personal assistant desktop application for Windows. Combines a chat interface with a proactive background agent that orchestrates workflows across email, calendar, messaging, and local files.

## Target Users
Small group of power users (< 100). Distributed via basic MSI installer.

## Architecture
- **Frontend:** Tauri v2 + Vue 3 (chat UI, notification panel, confirmation dialogs)
- **Backend:** Python sidecar process (AI orchestration, integrations, background agent)
- **AI:** Claude API behind swappable LLMProvider abstraction
- **Distribution:** Tauri MSI/NSIS installer for Windows

## MVP Features

### 1. Gmail Integration
- Read emails (inbox, labels, search)
- Draft email responses (AI-generated)
- Human-in-the-loop: user confirms before sending
- OAuth2 authentication (Google, testing mode)
- Event-driven via Gmail Pub/Sub push notifications, polling fallback

### 2. Google Calendar Integration
- Create calendar entries from email and Signal context
- Modify existing entries
- AI extracts event details (date, time, participants, location) from messages
- Human-in-the-loop: user confirms before creating/modifying

### 3. Signal Messaging Integration
- Read incoming messages via signal-cli daemon mode
- Send messages (user confirms before sending)
- Summarize unread conversations via LLM
- signal-cli linked as secondary device
- Requires bundled JVM runtime

### 4. File Organization
- Scan local filesystem directories (user-configured)
- Categorize files via LLM
- Recommend file moves and reorganization
- Recommend deletions with confidence thresholds
- Human-in-the-loop: user confirms before any move/delete
- Uses watchdog for filesystem monitoring

### 5. Workflow Orchestration
- Connect integrations (e.g., email content → calendar entry)
- AI-powered: LLM decides which tools/actions to invoke
- Claude tool_use / function calling for structured actions
- Proactive suggestions surfaced as notifications
- Event-driven architecture with polling fallback

## Cross-Cutting Concerns

### AI Layer
- LLMProvider abstraction interface (swap Claude for other models later)
- Tool definitions in standardized internal format
- Translation to/from Claude tool_use format at the edge
- Mock LLM responses for deterministic testing

### Human-in-the-Loop
- All actions require user confirmation in MVP
- Draft → Review → Confirm → Execute flow
- Confirmation dialogs in Vue 3 frontend
- Action history / audit log

### Background Agent
- Event-driven where possible (Gmail Pub/Sub, signal-cli daemon)
- Polling fallback for services without push support
- Proactive suggestions (e.g., "3 unread Signal messages, want a summary?")
- Runs as Python sidecar managed by Tauri

### Security
- OAuth2 tokens stored securely (Windows Credential Manager)
- No plaintext secrets
- signal-cli credentials managed separately
- Local-only data processing (no cloud storage of user data)

### Distribution
- Tauri MSI/NSIS installer
- Bundle Python runtime (embedded Python or pyinstaller)
- Bundle JVM for signal-cli
- Google OAuth2 in testing mode (< 100 users)
- Basic auto-update mechanism

## Technical Constraints
- Windows-only for MVP
- Python 3.11+ backend
- Node.js / Vue 3 / TypeScript frontend
- Tauri v2 with Rust
- signal-cli requires JVM 17+
- Google OAuth2 testing mode (unverified app warning)

## Testing Strategy
- TDD approach: tests first, implementation second
- Unit tests: LLM provider, message parsing, file categorization, tool definitions
- Integration tests: Gmail API, Calendar API, signal-cli wrapper, filesystem operations
- E2E tests: full workflows (email → calendar, Signal → summary)
- Mock LLM responses for deterministic testing
- 85% coverage target

## Risks
- signal-cli stability (unofficial, may break on protocol updates)
- JVM dependency adds ~150MB+ to installer
- Google OAuth "unverified app" warning for test-mode users
- Proactive agent tuning (too many/few suggestions)
- File deletion recommendations need careful confidence thresholds
