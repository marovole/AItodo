## ADDED Requirements

### Requirement: Research Complete Notification

系统 SHALL 在调研完成时发送系统通知。

通知内容：
- 标题: "调研完成"
- 内容: "[Todo 标题] 的调研已完成，点击查看结果"
- 图标: 应用图标

#### Scenario: 发送完成通知

- **GIVEN** 一个 Todo 正在调研中
- **AND** 应用在后台运行
- **WHEN** 调研完成
- **THEN** 系统发送桌面通知
- **AND** 通知显示 Todo 标题

#### Scenario: 点击通知跳转

- **GIVEN** 收到调研完成通知
- **WHEN** 用户点击通知
- **THEN** 应用窗口激活并置前
- **AND** 自动选中该 Todo
- **AND** 显示详情面板

---

### Requirement: Notification Permission

系统 SHALL 请求并管理通知权限。

#### Scenario: 首次请求权限

- **GIVEN** 用户首次使用通知功能
- **WHEN** 触发需要通知的操作
- **THEN** 系统请求通知权限
- **AND** 提示用户允许通知

#### Scenario: 权限被拒绝

- **GIVEN** 用户拒绝了通知权限
- **WHEN** 调研完成
- **THEN** 不发送系统通知
- **AND** 仅在应用内显示提示

---

### Requirement: In-App Notification

系统 SHALL 在应用内显示通知提示。

#### Scenario: 应用内通知

- **GIVEN** 调研完成
- **AND** 应用窗口处于活动状态
- **WHEN** 收到完成事件
- **THEN** 在应用内显示 Toast 通知
- **AND** Toast 3秒后自动消失
- **AND** 点击 Toast 可跳转到该 Todo

---

### Requirement: Notification Settings

系统 SHALL 允许用户配置通知偏好。

设置项：
- 启用/禁用系统通知
- 启用/禁用应用内通知
- 通知声音开关

#### Scenario: 禁用通知

- **GIVEN** 用户在设置中禁用系统通知
- **WHEN** 调研完成
- **THEN** 不发送系统通知
- **AND** 仅在应用内显示提示（如果启用）
