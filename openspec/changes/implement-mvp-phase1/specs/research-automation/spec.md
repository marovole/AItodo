## ADDED Requirements

### Requirement: ChatGPT WebView Integration

系统 SHALL 内嵌 ChatGPT WebView 用于执行 Deep Research。

WebView 配置：
- URL: https://chatgpt.com
- 数据目录: 独立持久化（保持登录状态）
- 可见性: 默认隐藏，可手动展开

#### Scenario: WebView 初始化

- **GIVEN** 应用启动
- **WHEN** Tauri 初始化完成
- **THEN** 创建隐藏的 ChatGPT WebView
- **AND** 加载 chatgpt.com
- **AND** 如用户之前已登录，保持登录状态

#### Scenario: 切换 WebView 可见性

- **GIVEN** 正在进行调研
- **WHEN** 用户点击 "查看实时画面"
- **THEN** WebView 区域展开显示
- **AND** 用户可以看到 ChatGPT 的实时交互过程

---

### Requirement: Login State Detection

系统 SHALL 检测 ChatGPT 的登录状态。

#### Scenario: 检测未登录

- **GIVEN** 用户首次使用或 Cookie 过期
- **WHEN** 尝试开始调研
- **THEN** 系统检测到未登录状态
- **AND** 显示 WebView 让用户登录
- **AND** 提示 "请先登录 ChatGPT"

#### Scenario: 登录状态持久化

- **GIVEN** 用户已登录 ChatGPT
- **WHEN** 关闭并重新打开应用
- **THEN** 登录状态保持
- **AND** 可以直接开始调研

---

### Requirement: Deep Research Trigger

系统 SHALL 自动触发 ChatGPT 的 Deep Research 功能。

触发流程：
1. 定位输入框元素
2. 注入用户的调研问题
3. 触发 input 事件使 React 响应
4. 定位并点击 Deep Research 按钮
5. 等待调研开始

#### Scenario: 成功触发调研

- **GIVEN** 用户已登录 ChatGPT
- **AND** 一个状态为 "pending" 的 Todo
- **WHEN** 用户点击 "开始调研"
- **THEN** 系统自动在 ChatGPT 输入框填入问题
- **AND** 自动选择 Deep Research 模式
- **AND** 自动开始调研

#### Scenario: 触发失败处理

- **GIVEN** ChatGPT 页面结构变化
- **WHEN** 系统无法定位输入框或按钮
- **THEN** 显示错误提示 "自动化失败，请手动操作"
- **AND** 展开 WebView 让用户手动操作

---

### Requirement: Research Result Extraction

系统 SHALL 自动提取 Deep Research 的结果。

提取机制：
- 使用 MutationObserver 监控 DOM 变化
- 检测调研完成标志（结果区域稳定）
- 提取 Markdown 格式的结果内容
- 保留引用链接

#### Scenario: 结果提取成功

- **GIVEN** Deep Research 正在进行
- **WHEN** ChatGPT 完成调研并显示结果
- **THEN** 系统检测到完成状态
- **AND** 提取完整的调研报告（Markdown）
- **AND** 提取所有引用链接
- **AND** 通过 IPC 发送到 Rust 层

#### Scenario: 长时间调研监控

- **GIVEN** Deep Research 正在进行
- **WHEN** 调研持续超过 5 分钟
- **THEN** 系统继续监控直到完成
- **AND** 定期更新进度状态

---

### Requirement: Research Cancellation

系统 SHALL 支持取消进行中的调研。

#### Scenario: 取消调研

- **GIVEN** 一个状态为 "researching" 的 Todo
- **WHEN** 用户点击 "取消调研"
- **THEN** 停止监控 WebView
- **AND** Todo 状态恢复为 "pending"
- **AND** 不保存任何部分结果

---

### Requirement: Automation Script Injection

系统 SHALL 通过 JS 注入实现自动化。

注入方式：
- initialization_script: 页面加载时注入基础脚本
- eval(): 运行时注入调研指令

脚本功能：
- DeepResearch.start(prompt): 开始调研
- DeepResearch.cancel(): 取消调研
- DeepResearch.getStatus(): 获取当前状态

#### Scenario: 脚本注入成功

- **GIVEN** ChatGPT WebView 加载完成
- **WHEN** initialization_script 执行
- **THEN** window.DeepResearch 对象可用
- **AND** MutationObserver 开始监控

#### Scenario: IPC 通信

- **GIVEN** 调研完成且结果已提取
- **WHEN** 脚本调用 window.__TAURI__.emit()
- **THEN** Rust 层收到 'research_complete' 事件
- **AND** 事件包含 { todoId, content, citations }
