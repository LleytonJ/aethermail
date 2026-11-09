## ADDED Requirements

### Requirement: 用户可查看联系人列表

系统 SHALL 提供联系人列表界面，展示所有已收集的联系人。

#### Scenario: 显示联系人列表

- **WHEN** 用户打开联系人管理界面
- **THEN** 系统显示所有联系人，按联系频率排序

#### Scenario: 搜索筛选联系人

- **WHEN** 用户在联系人管理界面输入搜索关键词
- **THEN** 系统过滤显示匹配的联系人

#### Scenario: 显示联系人数量统计

- **WHEN** 联系人列表加载完成
- **THEN** 系统显示总联系人数量

### Requirement: 用户可编辑联系人信息

系统 SHALL 允许用户编辑联系人的显示名称和头像。

#### Scenario: 编辑显示名称

- **WHEN** 用户点击编辑按钮
- **THEN** 进入编辑模式，显示名称变为可编辑输入框

#### Scenario: 保存编辑

- **WHEN** 用户修改显示名称后按 Enter 或点击保存
- **THEN** 系统更新联系人记录

#### Scenario: 取消编辑

- **WHEN** 用户修改显示名称后按 Escape
- **THEN** 恢复原有显示名称，退出编辑模式

#### Scenario: 设置头像

- **WHEN** 用户为联系人提供头像 URL
- **THEN** 系统保存头像 URL 到联系人记录

### Requirement: 用户可删除联系人

系统 SHALL 允许用户删除不再需要的联系人。

#### Scenario: 删除联系人确认

- **WHEN** 用户点击删除按钮
- **THEN** 系统提示确认对话框

#### Scenario: 确认删除

- **WHEN** 用户确认删除
- **THEN** 系统从数据库移除联系人记录

#### Scenario: 取消删除

- **WHEN** 用户取消删除确认
- **THEN** 联系人保留不变