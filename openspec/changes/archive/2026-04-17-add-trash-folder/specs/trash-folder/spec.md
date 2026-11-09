## ADDED Requirements

### Requirement: Trash folder display

The system SHALL provide a trash folder to display deleted emails.

#### Scenario: Load trash emails
- **WHEN** user clicks trash folder in sidebar
- **THEN** system loads emails where deleted_at is not NULL
- **AND** system sorts by deleted_at descending

#### Scenario: Trash folder empty
- **WHEN** user clicks trash folder with no deleted emails
- **THEN** system shows empty state message

### Requirement: Move email to trash

The system SHALL move emails to trash instead of permanently deleting.

#### Scenario: Move to trash from inbox
- **WHEN** user deletes an email from inbox
- **THEN** system sets deleted_at to current timestamp
- **AND** email no longer appears in inbox
- **AND** email appears in trash folder

#### Scenario: Move to trash from phishing folder
- **WHEN** user deletes an email from phishing folder
- **THEN** system sets deleted_at to current timestamp
- **AND** email no longer appears in phishing folder
- **AND** email appears in trash folder

### Requirement: Restore email from trash

The system SHALL allow restoring emails from trash to inbox or phishing folder.

#### Scenario: Restore to inbox
- **WHEN** user selects restore to inbox for a trash email
- **THEN** system sets deleted_at to NULL
- **AND** system sets is_phishing to false
- **AND** email appears in inbox

#### Scenario: Restore to phishing folder
- **WHEN** user selects restore to phishing folder for a trash email
- **THEN** system sets deleted_at to NULL
- **AND** system sets is_phishing to true
- **AND** email appears in phishing folder

### Requirement: Permanent delete from trash

The system SHALL allow permanently deleting emails from trash.

#### Scenario: Permanent delete
- **WHEN** user selects permanent delete for a trash email
- **THEN** system removes email record from database
- **AND** system removes all attachments via CASCADE
- **AND** email no longer appears anywhere

### Requirement: Auto cleanup expired trash emails

The system SHALL automatically clean up emails deleted more than 30 days ago.

#### Scenario: Cleanup on app start
- **WHEN** application starts
- **THEN** system permanently deletes emails where deleted_at > 30 days ago
- **AND** cleanup runs silently without user notification

#### Scenario: No expired emails
- **WHEN** application starts with no expired emails in trash
- **THEN** cleanup does nothing

### Requirement: Trash folder UI

The system SHALL display trash folder in sidebar.

#### Scenario: Trash folder item
- **WHEN** sidebar renders folder list
- **THEN** trash folder item is visible
- **AND** trash folder uses Trash2 icon
- **AND** trash folder shows label "回收站"

### Requirement: Trash email actions in ReadingPane

The system SHALL show restore and permanent delete buttons for trash emails.

#### Scenario: Trash email toolbar
- **WHEN** user views a trash email
- **THEN** system shows "恢复到收件箱" button
- **AND** system shows "恢复到钓鱼邮件" button
- **AND** system shows "永久删除" button