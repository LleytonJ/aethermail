## ADDED Requirements

### Requirement: 系统自动创建并维护联系人记录

系统 SHALL 在保存邮件时自动识别发件人，并将发件人信息存储到联系人表。

#### Scenario: 新联系人自动创建

- **WHEN** 系统保存一封来自 `alice@example.com`（显示名称 `Alice Smith`）的邮件
- **AND** 联系人表中不存在该邮箱地址
- **THEN** 系统创建新联系人记录，email 为 `alice@example.com`，display_name 为 `Alice Smith`，frequency 为 1

#### Scenario: 已存在联系人频率更新

- **WHEN** 系统保存一封来自 `bob@example.com` 的邮件
- **AND** 联系人表中已存在该邮箱地址（当前 frequency 为 5）
- **THEN** 系统更新联系人记录，frequency 增加为 6，last_contacted_at 更新为当前时间

#### Scenario: 邮箱地址规范化处理

- **WHEN** 系统保存一封来自 `John@Example.COM` 的邮件
- **THEN** 系统将邮箱地址规范化为 `john@example.com` 后存储

### Requirement: 系统支持联系人 CRUD 操作

系统 SHALL 提供联系人的增删改查 API。

#### Scenario: 根据邮箱获取联系人

- **WHEN** 用户查询邮箱 `alice@example.com` 的联系人信息
- **THEN** 系统返回该联系人的完整信息（id, email, display_name, avatar_url, frequency, last_contacted_at）

#### Scenario: 搜索联系人

- **WHEN** 用户输入搜索关键词 `ali`
- **THEN** 系统返回所有 email 或 display_name 包含 `ali` 的联系人

#### Scenario: 更新联系人显示名称

- **WHEN** 用户将联系人 `alice@example.com` 的显示名称修改为 `Alice Wang`
- **THEN** 系统更新联系人记录的 display_name 字段

#### Scenario: 更新联系人头像

- **WHEN** 用户为联系人设置头像 URL `https://example.com/avatar.png`
- **THEN** 系统更新联系人记录的 avatar_url 字段

#### Scenario: 删除联系人

- **WHEN** 用户删除联系人 `alice@example.com`
- **THEN** 系统从联系人表中移除该记录

### Requirement: 联系人数据持久化

系统 SHALL 使用 SQLite 数据库持久化联系人数据，并支持数据库迁移。

#### Scenario: 新数据库自动创建表

- **WHEN** 应用首次启动（数据库不存在）
- **THEN** 系统创建 contacts 表及相关索引

#### Scenario: 存量数据库迁移兼容

- **WHEN** 应用升级启动（数据库已存在但无 contacts 表）
- **THEN** 系统通过迁移创建 contacts 表，不影响现有数据