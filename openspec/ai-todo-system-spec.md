# AI Todo System - OpenSPEC Specification

> **Version**: 1.0  
> **Date**: 2026-01-13  
> **Status**: Specification Complete - Ready for Implementation  
> **Based on**: Comprehensive analysis report and existing MVP plan

---

## Executive Summary

### Project Vision
Create an intelligent desktop Todo application that transforms task management from simple completion tracking to AI-powered research automation. The system leverages users' existing AI service subscriptions (ChatGPT, Perplexity, Gemini) to automatically conduct deep research on Todo items, turning ambiguous tasks into actionable insights.

### Core Innovation
**From "Remember what to do" to "Understand what this task actually requires"** - Traditional Todo apps assume users know task details, while AI Todo System recognizes that many tasks require research before they become actionable.

### Target Market
- Information workers, researchers, investors
- Professionals requiring frequent information verification
- Users with multiple AI subscriptions seeking unified utilization

---

## Functional Requirements

### 1. User Stories & Epics

#### Epic 1: Todo Foundation Management
**User Story**: As a professional, I want to create, organize, and manage research tasks so that I can track what needs investigation.

**Acceptance Criteria**:
- Create new Todo with title, description, and optional URL
- Edit existing Todo items inline
- Delete Todo items with confirmation
- Organize Todos by status (pending → researching → review → done → archived)
- Search and filter Todos by content and status

#### Epic 2: AI Research Automation
**User Story**: As a busy professional, I want the system to automatically research my Todo items using my existing AI subscriptions so that I get comprehensive insights without manual effort.

**Acceptance Criteria**:
- Trigger research on-demand for any Todo item
- Automatically inject research prompts into ChatGPT Deep Research
- Monitor research progress with real-time updates
- Extract and store research results in structured format
- Handle research failures gracefully with retry mechanisms

#### Epic 3: Multi-Service Integration
**User Story**: As a power user, I want to leverage multiple AI services for research so that I can get diverse perspectives and comprehensive coverage.

**Acceptance Criteria**:
- Configure ChatGPT, Perplexity, and Gemini integrations
- Select preferred AI service per Todo or use default
- Execute parallel research across multiple services
- Merge and compare results from different sources
- Handle service-specific login session management

#### Epic 4: Research Result Management
**User Story**: As a decision-maker, I want to review, organize, and act upon research findings so that I can make informed decisions.

**Acceptance Criteria**:
- Display research results in rich Markdown format
- Show research metadata (duration, source, timestamp)
- Export results to various formats (PDF, Markdown, JSON)
- Add personal notes and annotations to research
- Archive completed research for future reference

### 2. Functional Matrix

| Feature | Priority | Phase | Complexity | Dependencies |
|---------|----------|---------|------------|--------------|
| Todo CRUD Operations | High | 1 | Database Layer |
| Status Management | High | 1 | Todo CRUD |
| ChatGPT Integration | High | 2 | WebView Management |
| Deep Research Automation | High | 3 | ChatGPT Integration |
| Multi-Service Support | Medium | 2 | Service Abstraction |
| Result Display | High | 2 | Storage Layer |
| System Notifications | Medium | 2 | Research Automation |
| Search & Filter | Medium | 2 | Todo CRUD |
| Export Functionality | Low | 3 | Result Management |
| Browser Extension | Low | 3 | Todo Management |

---

## Technical Architecture

### 1. System Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                    AI Todo Desktop Application                      │
│                        (Tauri 2.0)                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┬───────────────────────────┬──────────────────────┐│
│  │   Sidebar    │       Todo List           │     Detail Panel     ││
│  │ (TodoNav)   │      (TodoList)           │   (TodoDetail)      ││
│  │             │                           │                      ││
│  │ 📥 Pending  │ ┌─────────────────────┐  │ ┌────────────────┐  ││
│  │ 🔍 Research │ │ ☐ Research Company 🔄 │  │ │ Research     │  ││
│  │ 📋 Review   │ │ ☐ Verify News     ⏳ │  │ │ Results      │  ││
│  │ ✅ Done      │ └─────────────────────┘  │ │ (Markdown)   │  ││
│  │             │                           │ │              │  ││
│  │ ⚙️ Settings │ [＋ New Research Task]   │ │ [👁️ Live]   │  ││
│  └─────────────┴───────────────────────────┴──────────────────────┘│
│                                                                 │
├─────────────────────────────────────────────────────────────────────┤
│                 Embedded WebView Layer (Default Hidden)              │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │  ChatGPT WebView (Persistent Session)                      ││
│  │  ├─ Auto-inject deep_research.js                           ││
│  │  ├─ Monitor DOM changes (MutationObserver)                  ││
│  │  └─ IPC results → Rust → React UI                         ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                                                 │
├─────────────────────────────────────────────────────────────────────┤
│                          Rust Core Layer                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │   cmd.rs     │  │   todo.rs    │  │  setup.rs    │              │
│  │  IPC Commands │  │  Todo CRUD   │  │  Window Mgmt  │              │
│  │  start_research│  │  SQLite Ops  │  │  WebView     │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                                                                 │
├─────────────────────────────────────────────────────────────────────┤
│                        SQLite Local Storage                         │
│  todos | research_results | settings | webview_sessions               │
└─────────────────────────────────────────────────────────────────────┘
```

### 2. Technology Stack

| Layer | Technology | Rationale |
|--------|-------------|------------|
| **Desktop Framework** | Tauri 2.0 | Lightweight (10MB), native WebView support, Rust performance |
| **Base Project** | Fork lencx/ChatGPT | 50k stars, proven automation patterns, multi-WebView architecture |
| **Frontend** | React 18 + TypeScript | Existing codebase, strong ecosystem, type safety |
| **Styling** | Tailwind CSS + Fluent Design | Microsoft Todo consistency, rapid development |
| **Database** | SQLite + Rusqlite | Serverless, ACID compliant, Rust native |
| **State Management** | React Context + useReducer | Simple, sufficient for current scope |
| **WebView Automation** | JavaScript Injection + IPC | Proven approach, bypasses API limitations |

### 3. Data Models

```sql
-- Core Todos table
CREATE TABLE todos (
    id TEXT PRIMARY KEY,                    -- UUID v4
    title TEXT NOT NULL,
    description TEXT,
    url TEXT,                             -- Optional reference URL
    status TEXT DEFAULT 'pending',          -- pending/researching/review/done/archived
    priority INTEGER DEFAULT 0,              -- 0=low, 1=medium, 2=high
    preferred_service TEXT,                  -- chatgpt/perplexity/gemini
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME                   -- When marked done
);

-- Research results storage
CREATE TABLE research_results (
    id TEXT PRIMARY KEY,
    todo_id TEXT REFERENCES todos(id) ON DELETE CASCADE,
    service TEXT NOT NULL,                  -- Source AI service
    prompt TEXT,                           -- Original research prompt
    content TEXT NOT NULL,                  -- Formatted Markdown result
    raw_html TEXT,                         -- Backup of original HTML
    metadata TEXT,                          -- JSON: duration, sources, confidence
    started_at DATETIME,
    completed_at DATETIME,
    duration_seconds INTEGER,
    status TEXT DEFAULT 'completed',         -- completed/failed/timeout
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Application settings
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    type TEXT DEFAULT 'string',              -- string/number/boolean/json
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- WebView session persistence
CREATE TABLE webview_sessions (
    id TEXT PRIMARY KEY,
    service TEXT NOT NULL,                   -- chatgpt/perplexity/gemini
    user_data_dir TEXT NOT NULL,             -- Isolated storage path
    cookies_file TEXT,                       -- Cookie persistence
    last_active DATETIME,
    is_logged_in BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Research progress tracking
CREATE TABLE research_progress (
    todo_id TEXT REFERENCES todos(id) ON DELETE CASCADE,
    service TEXT NOT NULL,
    stage TEXT NOT NULL,                    -- searching/analyzing/synthesizing/completed
    progress_percentage INTEGER DEFAULT 0,      -- 0-100
    current_step TEXT,
    estimated_remaining INTEGER,               -- Seconds
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (todo_id, service)
);
```

### 4. API Design

#### IPC Command Interface

```rust
// Todo Management Commands
#[tauri::command]
async fn create_todo(title: String, description: Option<String>, url: Option<String>) -> Result<Todo, String>

#[tauri::command]
async fn update_todo(id: String, updates: TodoUpdate) -> Result<Todo, String>

#[tauri::command]
async fn delete_todo(id: String) -> Result<bool, String>

#[tauri::command]
async fn list_todos(filter: TodoFilter) -> Result<Vec<Todo>, String>

// Research Automation Commands
#[tauri::command]
async fn start_research(todo_id: String, service: Option<String>) -> Result<ResearchSession, String>

#[tauri::command]
async fn cancel_research(todo_id: String) -> Result<bool, String>

#[tauri::command]
async fn get_research_progress(todo_id: String) -> Result<ResearchProgress, String>

#[tauri::command]
async fn get_research_results(todo_id: String) -> Result<Vec<ResearchResult>, String>

// WebView Management Commands
#[tauri::command]
async fn create_research_webview(service: String) -> Result<String, String>

#[tauri::command]
async fn inject_research_script(webview_label: String, script: String) -> Result<bool, String>

#[tauri::command]
async fn check_login_status(service: String) -> Result<LoginStatus, String>
```

#### Event System

```rust
// Frontend Events (Rust → JavaScript)
#[tauri::command]
async fn emit_research_started(todo_id: String, service: String)

#[tauri::command]
async fn emit_research_progress(todo_id: String, progress: ResearchProgress)

#[tauri::command]
async fn emit_research_completed(todo_id: String, results: ResearchResult)

#[tauri::command]
async fn emit_research_failed(todo_id: String, error: String)

// Backend Events (JavaScript → Rust)
window.__TAURI__.event.emit('research_result_extracted', { todo_id, content })
window.__TAURI__.event.emit('research_stage_changed', { todo_id, stage })
```

---

## UI/UX Specification

### 1. Design Principles

| Principle | Implementation |
|------------|----------------|
| **Clarity through Hierarchy** | Clear visual hierarchy with typography and spacing |
| **Progressive Disclosure** | Show relevant information, hide complexity until needed |
| **System Integration** | Native notifications, system tray integration |
| **Responsive Feedback** | Loading states, progress indicators, micro-interactions |
| **Microsoft Fluent Design** | Consistent with Windows design language |

### 2. Layout Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                     AI Todo Application Layout                │
├────────────┬─────────────────────────────┬───────────────────────┤
│            │                             │                       │
│  Sidebar    │         Todo List            │      Detail Panel     │
│  (200px)   │        (flex: 1)            │      (360px)         │
│ Collapsible │                             │     Slide-in          │
│            │  ┌───────────────────────┐  │  ┌─────────────────┐  │
│ ┌────────┐ │  │ ○ Task Title     ⭐  │  │  │ Task Title     │  │
│ │📥 Pending│ │  │   Subtitle/URL        │  │  │ ────────────── │  │
│ │🔍 Research│ │  ├───────────────────────┤  │  │ Status: 🔄 Research│  │
│ │📋 Review  │  │ ○ Task Title     🔄  │  │  │                 │  │
│ │✅ Done    │  │   Researching...        │  │  │ 📊 Research    │  │
│ │────────│ │  └───────────────────────┘  │  │ │ Results       │  │
│ │⚙️ Settings│ │                             │  │ │ (Markdown)    │  │
│ └────────┘ │  ┌───────────────────────┐  │  │               │  │
│            │  │ ＋ New Research Task   │  │  │ [👁️ Live View]│  │
│            │  └───────────────────────┘  │  └─────────────────┘  │
│            │                             │                       │
└────────────┴─────────────────────────────┴───────────────────────┘
```

### 3. Component Library

#### Core Components

| Component | Props | Behavior | Styling |
|-----------|--------|-----------|----------|
| **TodoItem** | todo, onEdit, onDelete, onStatusChange | Hover actions, checkbox animation | Card with subtle border, 16px padding |
| **SidebarNav** | todos, selectedId, onSelect | Collapsible sections, keyboard navigation | 200px width, smooth transitions |
| **DetailPanel** | todo, isVisible, onClose | Slide-in from right, escape to close | 360px width, backdrop blur |
| **ResearchProgress** | progress, stage, onCancel | Circular progress, stage indicators | Semi-transparent overlay |
| **CreateTodoModal** | isVisible, onSubmit | Focus management, validation | Centered modal with backdrop |

#### Status Indicators

| Status | Visual | Animation | Color |
|---------|---------|------------|--------|
| **Pending** | ○ Hollow circle | None | Gray-400 |
| **Researching** | 🔄 Spinner + progress | Rotation 1s, fade in | Blue-500 |
| **Review** | ● Solid dot | Pulse 2s | Blue-600 |
| **Completed** | ✓ Checkmark with strikethrough | Checkmark draw 300ms | Green-500, opacity-0.7 |

### 4. Interaction Patterns

#### Micro-interactions
- **Task Creation**: Slide up from input field (200ms ease-out)
- **Status Change**: Smooth color transition (300ms ease-in-out)
- **Panel Open**: Translate from right (400ms cubic-bezier)
- **Button Hover**: Subtle lift effect (scale 1.02)
- **List Selection**: Highlight with background blur

#### Keyboard Navigation
- **Tab**: Logical navigation between sections
- **Enter**: Confirm actions, submit forms
- **Escape**: Close modals, cancel operations
- **Ctrl/Cmd+N**: New todo shortcut
- **Ctrl/Cmd+F**: Focus search input

---

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-4)

#### Week 1: Project Setup
- [ ] Fork lencx/ChatGPT repository
- [ ] Restructure project for Todo functionality
- [ ] Set up SQLite database layer with migrations
- [ ] Create core Rust modules (todo.rs, research.rs)
- [ ] Configure TypeScript build system

**Deliverable**: Basic project structure with database connectivity

#### Week 2: Todo Core
- [ ] Implement Todo CRUD operations in Rust
- [ ] Create React components for Todo management
- [ ] Build three-panel layout system
- [ ] Add state management with React Context
- [ ] Implement basic styling with Tailwind

**Deliverable**: Fully functional Todo management without AI features

#### Week 3: WebView Integration
- [ ] Integrate ChatGPT WebView with persistent sessions
- [ ] Develop JavaScript automation scripts
- [ ] Create IPC communication layer
- [ ] Implement research progress monitoring
- [ ] Add result extraction and parsing

**Deliverable**: Manual ChatGPT integration with basic automation

#### Week 4: Automation Complete
- [ ] Implement Deep Research automation pipeline
- [ ] Add system notifications for completion
- [ ] Create error handling and retry mechanisms
- [ ] Implement result display in Markdown
- [ ] Add UI polish and animations

**Deliverable**: Complete automated research workflow

### Phase 2: Multi-Service (Weeks 5-7)

#### Week 5-6: Service Expansion
- [ ] Abstract service interface in Rust
- [ ] Implement Perplexity WebView integration
- [ ] Add Gemini WebView integration
- [ ] Create service selection UI
- [ ] Implement parallel research execution

#### Week 7: Advanced Features
- [ ] Add result comparison and merging
- [ ] Implement advanced filtering and search
- [ ] Create export functionality
- [ ] Add research history and analytics

### Phase 3: Enhancement (Weeks 8-9)

#### Week 8-9: Polish & Integration
- [ ] Browser extension for one-click Todo creation
- [ ] Clipboard monitoring for quick task capture
- [ ] Advanced export options (PDF with formatting)
- [ ] Performance optimization and memory management
- [ ] Comprehensive testing and bug fixes

---

## Risk Assessment & Mitigation

### Technical Risks

| Risk | Probability | Impact | Mitigation Strategy |
|-------|-------------|----------|-------------------|
| **ChatGPT UI Changes** | Medium | High | Semantic selectors, versioned automation scripts, fallback to manual |
| **WebView Performance** | Low | Medium | Lazy loading, resource pooling, memory monitoring |
| **Session Persistence** | Medium | Medium | Isolated user data directories, cookie backup mechanisms |
| **Tauri 2.0 Compatibility** | Low | Medium | Track upstream changes, implement compatibility layer |

### Business Risks

| Risk | Probability | Impact | Mitigation Strategy |
|-------|-------------|----------|-------------------|
| **AI Service Terms Violation** | Low | High | Respect ToS, implement rate limiting, user responsibility |
| **User Adoption** | Medium | Medium | Focus on UX, provide migration tools, documentation |
| **Competitive Pressure** | High | Medium | Continuous innovation, community engagement, open source advantage |

---

## Testing Strategy

### 1. Test Pyramid

```
                    ┌─────────────────────┐
                    │   E2E Tests       │
                    │  (Cypress/Playwright)│
                    └─────────────────────┘
                ┌──────────────────────────────────┐
                │      Integration Tests          │
                │  (Tauri + React + SQLite)    │
                └──────────────────────────────────┘
        ┌──────────────────────────────────────────────────┐
        │           Unit Tests                       │
        │  (Rust modules + React components)       │
        └──────────────────────────────────────────────────┘
```

### 2. Test Coverage Requirements

| Layer | Target Coverage | Tools |
|--------|----------------|--------|
| **Rust Backend** | 90%+ | cargo test, tarpaulin |
| **React Components** | 85%+ | Jest, React Testing Library |
| **Integration** | 80%+ | Custom test harness, mock WebViews |
| **E2E** | 70%+ | Playwright, automated UI testing |

### 3. Quality Gates

| Gate | Criteria | Automation |
|-------|-----------|------------|
| **Code Review** | All PRs reviewed by 2+ devs | GitHub Actions |
| **Test Passing** | 100% tests pass | CI pipeline |
| **Performance** | <3s cold start, <100MB memory | Benchmarks |
| **Security** | No vulnerabilities in dependencies | Security scans |
| **Accessibility** | WCAG 2.1 AA compliance | Automated + manual testing |

---

## Deployment & Operations

### 1. Build & Release Pipeline

```
Development → Testing → Staging → Release
     │           │         │          │
  git push    CI Tests   Beta Test  Auto Release
     │           │         │          │
  Build All   Unit/Int   User      GitHub Releases
  Platforms   E2E Tests  Feedback   Auto-update
     │           │         │          │
  Artifacts   Coverage    Metrics    Statistics
```

### 2. Platform Support

| Platform | Build Target | Testing | Distribution |
|----------|--------------|----------|---------------|
| **macOS** | universal-apple-darwin | Intel + M1/M2 | Homebrew, GitHub |
| **Windows** | x86_64-pc-windows-msvc | Windows 10/11 | GitHub, Chocolatey |
| **Linux** | x86_64-unknown-linux-gnu | Ubuntu/Fedora | GitHub, Snap |

### 3. Monitoring & Analytics

| Metric | Collection | Usage |
|--------|------------|--------|
| **Performance** | Built-in telemetry | Optimization decisions |
| **Crash Reports** | Sentry/rust-panic | Stability monitoring |
| **Usage Patterns** | Opt-in analytics | Feature prioritization |
| **Error Tracking** | Structured logging | Bug resolution |

---

## Success Metrics

### 1. Technical KPIs

| Metric | Target | Measurement |
|--------|---------|-------------|
| **Research Success Rate** | >90% | Automated tracking |
| **Average Research Time** | <5 minutes | Per-service timing |
| **Application Startup** | <3 seconds cold | Performance monitoring |
| **Memory Usage** | <150MB peak | Resource monitoring |
| **Crash Rate** | <0.1% sessions | Error tracking |

### 2. User Experience KPIs

| Metric | Target | Measurement |
|--------|---------|-------------|
| **Task Creation Time** | <10 seconds | UI performance |
| **Research Initiation** | <3 clicks | Usability testing |
| **Result Comprehension** | >80% satisfaction | User feedback |
| **Feature Discovery** | >70% features used | Analytics |
| **User Retention** | >60% Day 7 return | Usage analytics |

---

## Appendix

### A. API References

- **Tauri 2.0 Documentation**: https://tauri.app/
- **SQLite Rust Guide**: https://docs.rs/rusqlite/
- **React TypeScript**: https://react-typescript-cheatsheet.netlify.app/

### B. Design Resources

- **Fluent Design System**: https://www.microsoft.com/design/fluent/
- **Microsoft Todo UI Reference**: https://todo.microsoft.com/
- **Figma UI Kit**: https://www.figma.com/community/file/1086534618953295138

### C. Development Environment Setup

```bash
# Prerequisites
rustup update stable
node --version  # >=18
pnpm --version   # >=8

# Project Setup
git clone https://github.com/lencx/ChatGPT.git AItodo
cd AItodo
pnpm install
cargo build

# Development
pnpm tauri dev  # Hot reload for frontend + backend
```

### D. Contribution Guidelines

1. **Code Style**: Rustfmt + Prettier
2. **Commit Messages**: Conventional Commits
3. **PR Process**: Draft → Review → Merge
4. **Testing**: All tests must pass
5. **Documentation**: Update relevant docs

---

**Specification Status**: ✅ Complete  
**Next Step**: Begin Phase 1 Implementation  
**Review Date**: Weekly during development sprints  

*Generated by OpenCode AI Assistant on 2026-01-13*