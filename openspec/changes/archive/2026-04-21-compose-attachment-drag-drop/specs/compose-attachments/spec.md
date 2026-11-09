## ADDED Requirements

### Requirement: 添加附件到撰写邮件

The system SHALL allow users to add attachments to emails being composed.

#### Scenario: 点击按钮添加附件
- **WHEN** user clicks "Add Attachment" button in compose interface
- **THEN** system opens file picker dialog
- **AND** user can select one or multiple files
- **AND** selected files are added to attachment list

#### Scenario: 拖拽文件添加附件
- **WHEN** user drags a file from desktop or file manager onto compose area
- **THEN** system displays drag highlight indicator
- **AND** after drop, file is added to attachment list
- **AND** compose area remains unchanged

#### Scenario: 拖拽多个文件添加附件
- **WHEN** user drags multiple files onto compose area
- **THEN** all files are added to attachment list

#### Scenario: 附件列表显示
- **WHEN** user has added attachments to compose
- **THEN** system displays attachment list below compose body
- **AND** each attachment shows filename, file size, and file type icon
- **AND** attachment list is collapsible if empty

#### Scenario: 删除附件
- **WHEN** user clicks remove button on an attachment in compose attachment list
- **THEN** attachment is removed from list
- **AND** file is no longer included in email

---

### Requirement: 附件类型识别

The system SHALL identify and display attachment file types.

#### Scenario: 显示文件类型图标
- **WHEN** attachment is displayed in compose attachment list
- **THEN** system shows appropriate icon based on file extension
- **AND** common file types have distinct icons (PDF, image, document, archive, etc.)

#### Scenario: 显示文件大小
- **WHEN** attachment is displayed in compose attachment list
- **THEN** system shows file size in human-readable format (KB, MB, GB)
- **AND** size is formatted with appropriate unit

---

### Requirement: 撰写附件状态管理

The system SHALL manage attachment state during compose session.

#### Scenario: 附件添加时显示上传状态
- **WHEN** user adds an attachment
- **THEN** system briefly shows "processing" indicator while reading file
- **AND** indicator disappears when file is ready

#### Scenario: 大附件处理
- **WHEN** user adds an attachment larger than 10MB
- **THEN** system stores file path reference instead of full content
- **AND** attachment is still displayed in list normally

#### Scenario: 文件读取失败
- **WHEN** user adds a file that cannot be read
- **THEN** system shows error notification
- **AND** file is not added to attachment list

---

### Requirement: 撰写附件持久化（草稿）

The system SHALL preserve attachments when saving drafts.

#### Scenario: 保存草稿时保留附件
- **WHEN** user saves a draft with attachments
- **THEN** system saves attachment metadata to database
- **AND** attachment file paths are preserved

#### Scenario: 编辑草稿时恢复附件
- **WHEN** user opens a saved draft for editing
- **AND** draft has attachments
- **THEN** system displays attachment list with saved attachments
- **AND** attachments can be removed or new ones added

#### Scenario: 发送草稿时包含附件
- **WHEN** user sends a saved draft with attachments
- **THEN** system reads files from saved paths
- **AND** attachments are included in sent email