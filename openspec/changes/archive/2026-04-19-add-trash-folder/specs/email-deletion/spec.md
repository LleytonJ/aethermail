## MODIFIED Requirements

### Requirement: Single email deletion in database

The system SHALL provide a method to soft delete a single email (move to trash).

#### Scenario: Delete email by ID (soft delete)
- **WHEN** backend receives delete request for an email ID
- **THEN** system sets deleted_at to current timestamp
- **AND** email is moved to trash folder
- **AND** attachments remain intact

#### Scenario: Email not found
- **WHEN** backend receives delete request for non-existent email ID
- **THEN** system returns error message "邮件不存在"

### Requirement: Backend command for email deletion

The system SHALL provide a Tauri command to move emails to trash.

#### Scenario: move_to_trash command
- **WHEN** frontend invokes move_to_trash command with email ID
- **THEN** system calls database soft delete method
- **AND** system returns success or error result

### Requirement: Frontend delete email service

The system SHALL provide a service function to move emails to trash.

#### Scenario: Move to trash service call
- **WHEN** frontend calls moveToTrash function with email ID
- **THEN** service invokes Tauri move_to_trash command
- **AND** service returns success or throws error

### Requirement: Mailbox store delete action

The system SHALL provide a store action to move email to trash.

#### Scenario: Move email to trash from store
- **WHEN** user triggers delete action with email ID
- **THEN** store calls moveToTrash service
- **AND** store removes email from current emails array
- **AND** store clears selectedEmailId if deleted email was selected
- **AND** store shows success notification

#### Scenario: Delete confirmation
- **WHEN** user clicks delete button
- **THEN** system shows confirmation dialog with email subject
- **AND** system only proceeds if user confirms

### Requirement: Delete button functionality

The system SHALL bind delete button to move-to-trash functionality.

#### Scenario: Delete normal email (move to trash)
- **WHEN** user clicks delete button on normal email
- **THEN** system shows confirmation dialog
- **AND** system moves email to trash after confirmation

#### Scenario: Delete phishing email (move to trash)
- **WHEN** user clicks delete button on phishing email
- **THEN** system shows confirmation dialog
- **AND** system moves email to trash after confirmation