## ADDED Requirements

### Requirement: Todo Creation

系统 SHALL 允许用户创建新的 Todo 任务。

每个 Todo MUST 包含：
- 唯一标识符 (id)
- 标题 (title) - 必填
- 描述 (description) - 可选
- URL 链接 (url) - 可选
- 状态 (status) - 默认为 "pending"
- 创建时间 (created_at)
- 更新时间 (updated_at)

#### Scenario: 创建基本 Todo

- **GIVEN** 用户在应用主界面
- **WHEN** 用户输入标题 "调查 XX 公司融资情况" 并提交
- **THEN** 系统创建新 Todo，状态为 "pending"
- **AND** Todo 出现在 "待调研" 列表中

#### Scenario: 创建带链接的 Todo

- **GIVEN** 用户在应用主界面
- **WHEN** 用户输入标题和 URL "https://example.com/news"
- **THEN** 系统创建新 Todo 并保存 URL
- **AND** URL 在详情面板中可点击

---

### Requirement: Todo Status Management

系统 SHALL 支持以下 Todo 状态流转：

| 状态 | 说明 |
|------|------|
| pending | 待调研 |
| researching | 调研中 |
| review | 待审阅 |
| done | 已完成 |
| archived | 已归档 |

状态转换规则：
- pending → researching: 开始调研时自动转换
- researching → review: 调研完成时自动转换
- review → done: 用户手动标记完成
- any → archived: 用户手动归档

#### Scenario: 状态自动转换 - 开始调研

- **GIVEN** 一个状态为 "pending" 的 Todo
- **WHEN** 用户点击 "开始调研"
- **THEN** Todo 状态变为 "researching"
- **AND** Todo 移动到 "调研中" 列表

#### Scenario: 状态自动转换 - 调研完成

- **GIVEN** 一个状态为 "researching" 的 Todo
- **WHEN** AI 调研完成并返回结果
- **THEN** Todo 状态变为 "review"
- **AND** Todo 移动到 "待审阅" 列表

#### Scenario: 手动标记完成

- **GIVEN** 一个状态为 "review" 的 Todo
- **WHEN** 用户点击完成按钮
- **THEN** Todo 状态变为 "done"
- **AND** Todo 移动到 "已完成" 列表

---

### Requirement: Todo Editing

系统 SHALL 允许用户编辑已有 Todo 的信息。

可编辑字段：
- 标题
- 描述
- URL

#### Scenario: 编辑 Todo 标题

- **GIVEN** 一个已存在的 Todo
- **WHEN** 用户在详情面板修改标题并保存
- **THEN** 标题更新成功
- **AND** updated_at 更新为当前时间

---

### Requirement: Todo Deletion

系统 SHALL 允许用户删除 Todo。

删除规则：
- 删除操作需要确认
- 关联的调研结果一并删除

#### Scenario: 删除 Todo

- **GIVEN** 一个已存在的 Todo
- **WHEN** 用户点击删除并确认
- **THEN** Todo 从数据库中移除
- **AND** 关联的 ResearchResult 也被删除

---

### Requirement: Todo Listing

系统 SHALL 支持按状态筛选 Todo 列表。

#### Scenario: 按状态筛选

- **GIVEN** 多个不同状态的 Todo
- **WHEN** 用户点击侧边栏 "待调研"
- **THEN** 列表只显示状态为 "pending" 的 Todo

#### Scenario: 显示所有 Todo

- **GIVEN** 多个不同状态的 Todo
- **WHEN** 用户点击侧边栏 "全部"
- **THEN** 列表显示所有未归档的 Todo
