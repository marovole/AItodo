# Project Context

## Purpose

AI Todo 深度调研助手 - 一个将 Todo 管理与 AI 深度调研能力结合的本地桌面应用。

核心理念：**不是"记住要做什么"，而是"搞清楚这件事到底是什么"**

用户创建待调研的问题/链接/想法，AI 自动启动深度调研（利用用户已订阅的 ChatGPT/Perplexity/Gemini），调研完成后通知用户，结果存入 Todo 详情。

## Tech Stack

- **桌面框架**: Tauri 2.0 (Rust + WebView)
- **前端**: React + TypeScript + Vite
- **样式**: TailwindCSS
- **本地数据库**: SQLite (via rusqlite)
- **基础项目**: Fork from [lencx/ChatGPT](https://github.com/lencx/ChatGPT) v2-dev branch
- **UI 风格**: 类 Microsoft Todo (Fluent Design)

## Project Conventions

### Code Style

- TypeScript strict mode
- React functional components with hooks
- Rust 使用 rustfmt 格式化
- 文件命名: PascalCase for components, camelCase for utilities

### Architecture Patterns

- **三层架构**: React UI → Tauri IPC → Rust Core
- **多 WebView 模式**: 主 UI WebView + 隐藏的 AI 服务 WebView
- **JS 注入自动化**: 通过 initialization_script 和 eval() 控制 AI 服务
- **事件驱动**: 使用 Tauri events 进行跨层通信

### Testing Strategy

- 前端: Vitest + React Testing Library
- Rust: 内置测试框架
- E2E: Playwright (可选)

### Git Workflow

- 主分支: main
- 功能分支: feature/xxx
- Commit 格式: `type(scope): message` (conventional commits)

## Domain Context

### 核心概念

- **Todo**: 待调研任务，包含标题、描述、可选链接
- **Research**: 调研过程，由 AI 服务执行
- **Research Result**: 调研结果，Markdown 格式
- **Status Flow**: pending → researching → review → done/archived

### AI 服务集成

- 使用内嵌 WebView 复用用户已登录的 AI 服务
- 不使用 API（节省成本，利用订阅）
- 通过 JS 注入实现自动化

## Important Constraints

- 必须保持 Cookie/登录状态持久化
- AI 服务 UI 可能变化，选择器需要维护
- Deep Research 可能需要 2-10 分钟，需要良好的进度提示
- 本地优先，所有数据存储在本地

## External Dependencies

- ChatGPT Web (chat.openai.com)
- Perplexity Web (perplexity.ai) - Phase 2
- Gemini Web (gemini.google.com) - Phase 2
- Fluent UI System Icons
