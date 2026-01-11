## ADDED Requirements

### Requirement: Three-Column Layout

系统 SHALL 采用三栏布局：

| 区域 | 宽度 | 说明 |
|------|------|------|
| 侧边栏 | 200px (可折叠) | 状态分类导航 |
| 任务列表 | flex: 1 | Todo 列表展示 |
| 详情面板 | 360px (可隐藏) | 选中 Todo 的详情 |

#### Scenario: 默认布局展示

- **GIVEN** 用户打开应用
- **WHEN** 应用加载完成
- **THEN** 显示三栏布局
- **AND** 侧边栏显示状态分类
- **AND** 列表区域显示 Todo 项

#### Scenario: 侧边栏折叠

- **GIVEN** 侧边栏处于展开状态
- **WHEN** 用户点击折叠按钮
- **THEN** 侧边栏收起为图标模式
- **AND** 列表区域扩展填充空间

---

### Requirement: Sidebar Navigation

侧边栏 SHALL 显示以下状态分类：

| 图标 | 名称 | 筛选条件 |
|------|------|----------|
| 📥 | 待调研 | status = pending |
| 🔍 | 调研中 | status = researching |
| 📋 | 待审阅 | status = review |
| ✅ | 已完成 | status = done |
| 📁 | 归档 | status = archived |

底部显示设置入口 (⚙️)。

#### Scenario: 点击状态分类

- **GIVEN** 用户在应用主界面
- **WHEN** 用户点击侧边栏 "调研中"
- **THEN** 该分类高亮显示
- **AND** 列表区域只显示 "researching" 状态的 Todo

#### Scenario: 显示状态计数

- **GIVEN** 有 3 个 pending 状态的 Todo
- **WHEN** 用户查看侧边栏
- **THEN** "待调研" 旁边显示数字 "3"

---

### Requirement: Todo List Display

任务列表 SHALL 以卡片形式展示 Todo 项。

每个卡片显示：
- 状态指示器（圆形图标）
- 标题
- 副标题（描述或 URL 的截断显示）
- 状态标签（调研中显示进度）

#### Scenario: 列表项展示

- **GIVEN** 一个标题为 "调查 XX 公司" 的 Todo
- **WHEN** 用户查看列表
- **THEN** 显示状态指示器 + 标题
- **AND** 如有描述则显示截断的描述

#### Scenario: 选中 Todo

- **GIVEN** 列表中有多个 Todo
- **WHEN** 用户点击某个 Todo
- **THEN** 该项高亮显示
- **AND** 详情面板从右侧滑入显示该 Todo 详情

---

### Requirement: Detail Panel

详情面板 SHALL 从右侧滑入显示，包含：

1. 标题（可编辑）
2. 状态显示
3. 描述/URL（可编辑）
4. 调研结果区域（Markdown 渲染）
5. 操作按钮（开始调研/查看实时画面）

#### Scenario: 详情面板滑入

- **GIVEN** 用户点击列表中的 Todo
- **WHEN** 点击动作完成
- **THEN** 详情面板从右侧滑入（300ms ease-out）
- **AND** 显示该 Todo 的完整信息

#### Scenario: 关闭详情面板

- **GIVEN** 详情面板处于显示状态
- **WHEN** 用户点击关闭按钮或点击列表其他区域
- **THEN** 详情面板滑出隐藏

---

### Requirement: Quick Add Input

系统 SHALL 在列表区域底部提供快速添加输入框。

输入框特性：
- 透明背景，聚焦时显示边框
- 左侧 "+" 图标
- 回车键提交

#### Scenario: 快速添加 Todo

- **GIVEN** 用户在列表区域
- **WHEN** 用户在输入框输入 "新任务" 并按回车
- **THEN** 创建新 Todo
- **AND** 新 Todo 出现在列表顶部
- **AND** 输入框清空

---

### Requirement: Status Visual Indicators

系统 SHALL 使用以下视觉元素表示状态：

| 状态 | 视觉表现 |
|------|----------|
| pending | 空心圆 ○ |
| researching | 旋转动画图标 🔄 |
| review | 实心蓝点 ● |
| done | 勾选 ✓ + 删除线 + 透明度 0.6 |
| archived | 灰色显示 |

#### Scenario: 调研中状态显示

- **GIVEN** 一个状态为 "researching" 的 Todo
- **WHEN** 用户查看该 Todo
- **THEN** 显示旋转动画图标
- **AND** 显示调研耗时 "调研中... (2分30秒)"

---

### Requirement: Fluent Design Style

UI 样式 SHALL 遵循 Microsoft Fluent Design 2 规范：

- 圆角: 4-8px
- 阴影: 柔和投影
- 颜色: 使用 Fluent 调色板
- 动画: 200-400ms ease-out
- 字体: Segoe UI / system-ui

#### Scenario: 视觉一致性

- **GIVEN** 用户浏览应用各界面
- **WHEN** 查看任意组件
- **THEN** 组件样式符合 Fluent Design 规范
- **AND** 交互动画流畅自然
