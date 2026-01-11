# Change: Implement AI Todo MVP Phase 1

## Why

用户需要一个将 Todo 管理与 AI 深度调研能力结合的工具。传统 Todo 假设用户已知任务内容，但很多 Todo 本身需要调研才能变成可执行任务。本 MVP 旨在验证核心流程：创建 Todo → AI 自动调研 → 结果回填 → 用户审阅。

## What Changes

### 新增能力

1. **Todo 管理 (todo-management)**: Todo 的创建、编辑、删除、状态管理
2. **Todo UI (todo-ui)**: 三栏布局界面（侧边栏、列表、详情面板）
3. **调研自动化 (research-automation)**: ChatGPT Deep Research 的自动触发和结果提取
4. **调研展示 (research-display)**: 调研结果的 Markdown 渲染和实时进度
5. **数据持久化 (data-persistence)**: SQLite 本地存储
6. **系统通知 (system-notifications)**: 调研完成时的系统级通知

### 修改的文件

- `src-tauri/src/core/setup.rs`: 添加 Todo UI WebView
- `src-tauri/src/core/cmd.rs`: 添加 Todo/Research IPC 命令
- `src-tauri/Cargo.toml`: 添加 rusqlite 依赖
- `src/App.tsx`: 添加路由和布局
- `package.json`: 添加前端依赖

### 新增的文件

- `src-tauri/src/core/todo.rs`: Todo 数据管理
- `src-tauri/scripts/deep_research.js`: Deep Research 自动化脚本
- `src/view/TodoApp.tsx`: 主应用容器
- `src/view/TodoSidebar.tsx`: 左侧导航
- `src/view/TodoList.tsx`: 任务列表
- `src/view/TodoDetail.tsx`: 右侧详情面板
- `src/view/TodoCreate.tsx`: 新建任务
- `src/view/ResearchProgress.tsx`: 调研进度
- `src/styles/todo.css`: Todo 样式

## Impact

- Affected specs: 6 个新能力模块
- Affected code: 
  - Rust: `src-tauri/src/core/` 
  - React: `src/view/`, `src/components/`
  - Scripts: `src-tauri/scripts/`
- 预计开发时间: 3-4 周
