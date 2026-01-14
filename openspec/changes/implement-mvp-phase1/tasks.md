## 1. 项目基础设施 (Week 1)

### 1.1 开发环境
- [x] 1.1.1 安装 Rust + Cargo
- [x] 1.1.2 安装 Node.js + pnpm
- [x] 1.1.3 安装 Tauri CLI
- [x] 1.1.4 验证 lencx/ChatGPT 基础项目可运行

### 1.2 项目结构调整
- [x] 1.2.1 更新 `tauri.conf.json` 中的应用名称和标识符
- [x] 1.2.2 更新 `package.json` 中的项目信息
- [x] 1.2.3 创建 `src/view/` 下的 Todo 相关目录结构
- [x] 1.2.4 创建 `src/styles/` 目录

### 1.3 数据库集成
- [x] 1.3.1 添加 `rusqlite` 依赖到 `Cargo.toml`
- [x] 1.3.2 创建 `src-tauri/src/core/db.rs` 数据库初始化模块
- [x] 1.3.3 实现数据库迁移/schema 创建
- [x] 1.3.4 编写数据库连接测试

## 2. Todo 数据层 (Week 1-2)

### 2.1 数据模型
- [x] 2.1.1 创建 `src-tauri/src/core/todo.rs` 模块
- [x] 2.1.2 定义 `Todo` 结构体
- [x] 2.1.3 定义 `ResearchResult` 结构体
- [x] 2.1.4 实现 Todo CRUD 函数

### 2.2 IPC 命令
- [x] 2.2.1 在 `cmd.rs` 添加 `create_todo` 命令
- [x] 2.2.2 在 `cmd.rs` 添加 `get_todos` 命令
- [x] 2.2.3 在 `cmd.rs` 添加 `update_todo` 命令
- [x] 2.2.4 在 `cmd.rs` 添加 `delete_todo` 命令
- [x] 2.2.5 在 `cmd.rs` 添加 `get_todo_detail` 命令
- [x] 2.2.6 在 `main.rs` 注册新命令

## 3. Todo UI 界面 (Week 2)

### 3.1 布局组件
- [x] 3.1.1 创建 `TodoApp.tsx` 主容器组件
- [x] 3.1.2 实现三栏布局（Sidebar | List | Detail）
- [x] 3.1.3 添加响应式适配

### 3.2 侧边栏
- [x] 3.2.1 创建 `TodoSidebar.tsx` 组件
- [x] 3.2.2 实现状态分类导航（待调研/调研中/待审阅/已完成）
- [x] 3.2.3 实现侧边栏折叠功能
- [x] 3.2.4 添加设置入口

### 3.3 任务列表
- [x] 3.3.1 创建 `TodoList.tsx` 组件
- [x] 3.3.2 创建 `TodoItem.tsx` 单项组件
- [x] 3.3.3 实现列表筛选（按状态）
- [x] 3.3.4 实现任务点击选中

### 3.4 详情面板
- [x] 3.4.1 创建 `TodoDetail.tsx` 组件
- [x] 3.4.2 实现右侧滑入动画
- [x] 3.4.3 显示任务基本信息
- [x] 3.4.4 显示调研结果区域（Markdown）
- [x] 3.4.5 添加"开始调研"按钮

### 3.5 新建任务
- [x] 3.5.1 创建 `TodoCreate.tsx` 组件
- [x] 3.5.2 实现输入框（标题 + 可选描述/链接）
- [x] 3.5.3 实现快速添加交互

### 3.6 样式系统
- [x] 3.6.1 创建 `todo.css` 基础样式
- [x] 3.6.2 实现 Fluent Design 风格变量
- [x] 3.6.3 添加状态指示器样式
- [x] 3.6.4 添加动画过渡效果

## 4. 调研自动化 (Week 3)

### 4.1 WebView 设置
- [x] 4.1.1 在 `setup.rs` 添加 ChatGPT WebView 创建
- [x] 4.1.2 配置 WebView 数据目录（Cookie 持久化）
- [x] 4.1.3 实现 WebView 显示/隐藏切换

### 4.2 自动化脚本
- [x] 4.2.1 创建 `deep_research.js` 脚本
- [x] 4.2.2 实现输入框定位和填充
- [x] 4.2.3 实现 Deep Research 模式触发
- [x] 4.2.4 实现 MutationObserver 监控结果
- [x] 4.2.5 实现结果提取和格式化
- [x] 4.2.6 实现 IPC 事件发送

### 4.3 调研流程集成
- [x] 4.3.1 在 `cmd.rs` 添加 `start_research` 命令
- [x] 4.3.2 实现调研状态更新逻辑
- [x] 4.3.3 实现调研结果保存逻辑
- [x] 4.3.4 添加 `research_complete` 事件监听

### 4.4 进度展示
- [x] 4.4.1 创建 `ResearchProgress.tsx` 组件
- [x] 4.4.2 实现调研中状态动画
- [x] 4.4.3 实现耗时计时器
- [x] 4.4.4 实现"查看实时画面"按钮

## 5. 调研结果展示 (Week 3-4)

### 5.1 Markdown 渲染
- [x] 5.1.1 添加 Markdown 渲染库依赖
- [x] 5.1.2 创建 `MarkdownViewer.tsx` 组件
- [x] 5.1.3 实现语法高亮
- [x] 5.1.4 实现链接可点击

### 5.2 结果操作
- [x] 5.2.1 实现复制内容功能
- [x] 5.2.2 实现展开/折叠长内容

## 6. 系统通知 (Week 4)

### 6.1 通知集成
- [x] 6.1.1 配置 Tauri 通知权限
- [x] 6.1.2 实现调研完成通知
- [x] 6.1.3 实现点击通知跳转到对应 Todo

## 7. 错误处理与完善 (Week 4)

### 7.1 错误处理
- [x] 7.1.1 实现登录状态检测
- [x] 7.1.2 实现调研失败处理
- [x] 7.1.3 添加用户友好的错误提示

### 7.2 边缘情况
- [x] 7.2.1 处理长时间调研超时
- [x] 7.2.2 实现调研取消功能
- [x] 7.2.3 处理网络断开情况

### 7.3 最终验证
- [x] 7.3.1 端到端测试：创建 Todo → 调研 → 查看结果
- [x] 7.3.2 验证 Cookie 持久化
- [x] 7.3.3 验证系统通知
- [x] 7.3.4 UI/UX 审查

## 依赖关系

```
1.1 → 1.2 → 1.3 → 2.1 → 2.2
                    ↓
              3.1 → 3.2/3.3/3.4/3.5 → 3.6
                    ↓
              4.1 → 4.2 → 4.3 → 4.4
                              ↓
                         5.1 → 5.2
                              ↓
                         6.1 → 7.1 → 7.2 → 7.3
```

## 可并行工作

- 3.2 (侧边栏) 和 3.3 (列表) 和 3.4 (详情) 可并行
- 5.1 (Markdown) 可与 4.x 并行开发
- 6.1 (通知) 可与 5.x 并行开发

## Implementation Status

**MVP Phase 1 Core Implementation: COMPLETE**

### Files Created:
- `src-tauri/src/core/db.rs` - SQLite database module
- `src-tauri/src/core/todo.rs` - Todo data model and CRUD operations
- `src-tauri/scripts/deep_research.js` - ChatGPT automation script
- `src/stores/todoStore.ts` - Zustand state management
- `src/view/TodoApp.tsx` - Main container with 3-column layout
- `src/view/TodoSidebar.tsx` - Status navigation sidebar
- `src/view/TodoList.tsx` - Task list display
- `src/view/TodoItem.tsx` - Individual task item
- `src/view/TodoDetail.tsx` - Detail panel with markdown
- `src/view/TodoCreate.tsx` - Quick add input
- `src/view/ResearchProgress.tsx` - Research progress display
- `src/styles/todo.css` - Fluent Design styling

### Files Modified:
- `src-tauri/Cargo.toml` - Added rusqlite, uuid, chrono, notification dependencies
- `src-tauri/src/main.rs` - Registered new commands, added DB init
- `src-tauri/src/core/mod.rs` - Added db and todo modules
- `src-tauri/src/core/cmd.rs` - Added Todo IPC commands
- `src-tauri/src/core/window.rs` - Added open_todo_app command
- `src-tauri/src/core/constant.rs` - Added WINDOW_TODO constant
- `src-tauri/capabilities/desktop.json` - Added notification permissions
- `src/App.tsx` - Added 'todo' route mapping
- `src/types.d.ts` - Added Todo/ResearchResult TypeScript types
- `package.json` - Added react-markdown, zustand, remark-gfm, notification plugin
- `tsconfig.json` - Added stores and styles path aliases

### Build Status:
- ✅ Frontend (TypeScript + Vite): BUILD SUCCESS
- ✅ Backend (Rust + Tauri): BUILD SUCCESS
