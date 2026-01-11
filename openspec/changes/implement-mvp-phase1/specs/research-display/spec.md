## ADDED Requirements

### Requirement: Research Progress Display

系统 SHALL 显示调研进度信息。

进度信息包括：
- 当前状态（调研中）
- 已用时间（实时更新）
- 来源服务（ChatGPT）

#### Scenario: 显示调研进度

- **GIVEN** 一个状态为 "researching" 的 Todo
- **WHEN** 用户查看该 Todo 详情
- **THEN** 显示进度区域
- **AND** 显示 "调研中..." + 旋转动画
- **AND** 显示 "已用时间: 2分30秒"（实时更新）

#### Scenario: 进度区域消失

- **GIVEN** 调研完成
- **WHEN** 状态变为 "review"
- **THEN** 进度区域消失
- **AND** 显示调研结果区域

---

### Requirement: Markdown Result Rendering

系统 SHALL 使用 Markdown 渲染调研结果。

支持的 Markdown 特性：
- 标题 (h1-h6)
- 段落和换行
- 粗体、斜体
- 有序/无序列表
- 代码块和行内代码
- 链接（可点击）
- 表格
- 引用块

#### Scenario: 渲染调研结果

- **GIVEN** 调研完成且有 Markdown 结果
- **WHEN** 用户查看 Todo 详情
- **THEN** 结果以格式化的 Markdown 显示
- **AND** 标题有层级样式
- **AND** 代码块有语法高亮

#### Scenario: 链接可点击

- **GIVEN** 调研结果中包含链接
- **WHEN** 用户点击链接
- **THEN** 链接在系统默认浏览器中打开

---

### Requirement: Live WebView Preview

系统 SHALL 允许用户查看 AI 服务的实时画面。

#### Scenario: 展开实时画面

- **GIVEN** 正在进行调研
- **WHEN** 用户点击 "👁️ 查看实时画面"
- **THEN** 详情面板扩展显示 WebView 区域
- **AND** 用户可以看到 ChatGPT 的实时交互

#### Scenario: 收起实时画面

- **GIVEN** 实时画面正在显示
- **WHEN** 用户点击 "🔽 收起"
- **THEN** WebView 区域收起
- **AND** 恢复简洁视图

---

### Requirement: Result Actions

系统 SHALL 提供结果操作功能。

操作列表：
- 复制内容
- 展开/折叠长内容

#### Scenario: 复制结果内容

- **GIVEN** 调研结果已显示
- **WHEN** 用户点击 "复制" 按钮
- **THEN** 结果的 Markdown 文本复制到剪贴板
- **AND** 显示 "已复制" 提示

#### Scenario: 展开折叠内容

- **GIVEN** 调研结果超过 500 行
- **WHEN** 用户查看结果
- **THEN** 默认显示前 50 行
- **AND** 显示 "展开全部" 按钮
- **WHEN** 用户点击 "展开全部"
- **THEN** 显示完整内容

---

### Requirement: Research Duration Display

系统 SHALL 显示调研耗时统计。

#### Scenario: 显示调研耗时

- **GIVEN** 调研已完成
- **WHEN** 用户查看 Todo 详情
- **THEN** 显示 "调研耗时: 3分45秒"
- **AND** 显示 "来源: ChatGPT Deep Research"

---

### Requirement: Empty State Display

系统 SHALL 在无调研结果时显示空状态。

#### Scenario: 待调研状态

- **GIVEN** 一个状态为 "pending" 的 Todo
- **WHEN** 用户查看详情
- **THEN** 结果区域显示空状态提示
- **AND** 显示 "点击下方按钮开始调研"
- **AND** 显示 "开始调研" 按钮
