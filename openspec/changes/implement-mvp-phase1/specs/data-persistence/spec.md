## ADDED Requirements

### Requirement: SQLite Database

系统 SHALL 使用 SQLite 作为本地数据库。

数据库位置：应用数据目录 (app_data_dir)

#### Scenario: 数据库初始化

- **GIVEN** 应用首次启动
- **WHEN** Tauri 初始化完成
- **THEN** 在应用数据目录创建 SQLite 数据库文件
- **AND** 执行 schema 迁移创建表结构

#### Scenario: 数据库连接

- **GIVEN** 应用启动
- **WHEN** 需要读写数据
- **THEN** 使用连接池管理数据库连接
- **AND** 保证线程安全

---

### Requirement: Todo Data Schema

系统 SHALL 使用以下 Todo 表结构：

```sql
CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    url TEXT,
    status TEXT DEFAULT 'pending',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_todos_status ON todos(status);
```

#### Scenario: 创建 Todo 记录

- **GIVEN** 用户创建新 Todo
- **WHEN** 调用 create_todo
- **THEN** 生成 UUID 作为 id
- **AND** 插入记录到 todos 表
- **AND** created_at 和 updated_at 设为当前时间

#### Scenario: 更新 Todo 记录

- **GIVEN** 修改已有 Todo
- **WHEN** 调用 update_todo
- **THEN** 更新指定字段
- **AND** updated_at 更新为当前时间

---

### Requirement: Research Result Schema

系统 SHALL 使用以下调研结果表结构：

```sql
CREATE TABLE research_results (
    id TEXT PRIMARY KEY,
    todo_id TEXT NOT NULL REFERENCES todos(id) ON DELETE CASCADE,
    source TEXT NOT NULL,
    content TEXT,
    raw_html TEXT,
    started_at DATETIME,
    completed_at DATETIME,
    duration_seconds INTEGER
);

CREATE INDEX idx_research_todo ON research_results(todo_id);
```

#### Scenario: 保存调研结果

- **GIVEN** 调研完成
- **WHEN** 收到 research_complete 事件
- **THEN** 插入记录到 research_results 表
- **AND** 计算 duration_seconds
- **AND** 更新关联 Todo 状态为 "review"

#### Scenario: 级联删除

- **GIVEN** 一个有调研结果的 Todo
- **WHEN** 删除该 Todo
- **THEN** 关联的 research_results 记录也被删除

---

### Requirement: Settings Storage

系统 SHALL 存储用户设置。

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT
);
```

初始设置项：
- theme: light/dark/system
- sidebar_collapsed: true/false
- webview_visible_by_default: true/false

#### Scenario: 读取设置

- **GIVEN** 应用启动
- **WHEN** 加载设置
- **THEN** 从 settings 表读取所有配置
- **AND** 对于不存在的 key 使用默认值

#### Scenario: 保存设置

- **GIVEN** 用户修改设置
- **WHEN** 点击保存
- **THEN** UPSERT 到 settings 表

---

### Requirement: Data Migration

系统 SHALL 支持数据库 schema 迁移。

#### Scenario: 版本升级迁移

- **GIVEN** 应用升级且 schema 有变化
- **WHEN** 应用启动
- **THEN** 检测当前 schema 版本
- **AND** 执行必要的迁移脚本
- **AND** 保留现有数据

---

### Requirement: Data Integrity

系统 SHALL 保证数据完整性。

#### Scenario: 事务支持

- **GIVEN** 需要执行多步数据操作
- **WHEN** 执行事务
- **THEN** 所有操作原子性执行
- **AND** 失败时全部回滚

#### Scenario: 外键约束

- **GIVEN** research_results 引用 todos
- **WHEN** 尝试插入不存在 todo_id 的记录
- **THEN** 操作失败
- **AND** 返回外键约束错误
