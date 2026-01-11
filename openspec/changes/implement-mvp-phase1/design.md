## Context

AI Todo 是一个基于 Tauri 2.0 的桌面应用，fork 自 lencx/ChatGPT。核心创新是将 Todo 管理与 AI 深度调研结合，利用用户已订阅的 AI 服务（ChatGPT/Perplexity/Gemini）进行自动化调研。

### 技术约束

- 基于 lencx/ChatGPT v2-dev 分支，需要保持与上游的兼容性
- 使用内嵌 WebView 而非 API，以复用用户订阅
- AI 服务的 DOM 结构可能变化，需要可维护的选择器策略

### 利益相关者

- 终端用户：信息工作者、研究者、投资人
- 开发者：需要维护 AI 服务选择器

## Goals / Non-Goals

### Goals

- 实现完整的 Todo CRUD 功能
- 集成 ChatGPT Deep Research 自动化
- 提供简洁易用的类 Microsoft Todo 界面
- 本地优先，数据安全

### Non-Goals (Phase 1)

- 多 AI 服务并行（Phase 2）
- 浏览器扩展（Phase 3）
- 云同步
- 协作功能

## Decisions

### D1: 界面布局 - 三栏布局

**决定**: 采用类 Microsoft Todo 的三栏布局（侧边栏 | 列表 | 详情面板）

**理由**: 
- 用户熟悉的模式
- 信息层级清晰
- 详情面板可容纳调研结果

**替代方案**:
- 看板布局：适合项目管理，但对单条目详情展示不友好
- 单栏布局：移动端友好，但桌面端空间利用率低

### D2: AI 服务集成 - 内嵌 WebView

**决定**: 使用 Tauri 多 WebView 内嵌 AI 服务网页，通过 JS 注入实现自动化

**理由**:
- 复用用户已有订阅，零额外成本
- 完整的 Deep Research 功能（API 可能没有）
- lencx/ChatGPT 已验证可行

**替代方案**:
- API 调用：简单稳定，但需额外付费
- 浏览器自动化（Playwright）：独立进程，资源开销大

### D3: 数据存储 - SQLite

**决定**: 使用 SQLite 作为本地数据库

**理由**:
- 轻量，无需服务器
- Tauri 生态有成熟的 rusqlite 支持
- 适合桌面应用

**替代方案**:
- JSON 文件：简单但不支持复杂查询
- IndexedDB：前端方案，无法在 Rust 层访问

### D4: WebView 可见性 - 混合模式

**决定**: AI 服务 WebView 默认隐藏，可手动展开查看实时进度

**理由**:
- 保持界面简洁
- 用户可选择性查看
- 便于调试

## Risks / Trade-offs

### R1: AI 服务 DOM 变化

**风险**: ChatGPT/Perplexity/Gemini 更新 UI 后，选择器失效

**缓解**: 
- 使用语义化选择器（data-*, aria-*）
- 定期维护和更新脚本
- 添加失败检测和用户提示

### R2: Deep Research 响应时间

**风险**: 调研可能需要 2-10 分钟，用户体验差

**缓解**:
- 实时进度展示
- 支持取消操作
- 允许多任务并行

### R3: 登录状态失效

**风险**: Cookie 过期导致需要重新登录

**缓解**:
- 检测登录状态
- 引导用户重新登录
- 保持 WebView 会话持久化

## Migration Plan

不适用（新项目）

## Open Questions

1. 是否需要支持离线模式？（当前假设需要网络）
2. 调研结果是否需要支持编辑？
3. 是否需要调研历史对比功能？
