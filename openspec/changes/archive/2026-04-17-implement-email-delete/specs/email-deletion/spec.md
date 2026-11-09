## ADDED Requirements

### Requirement: Single email deletion in database

The system SHALL provide a method to delete a single email from the database.

#### Scenario: Delete email by ID
- **WHEN** backend receives delete request for an email ID
- **THEN** system removes the email record from emails table
- **AND** system removes all associated attachments via CASCADE

#### Scenario: Email not found
- **WHEN** backend receives delete request for non-existent email ID
- **THEN** system returns error message "邮件不存在"

### Requirement: Backend command for email deletion

The system SHALL provide a Tauri command for frontend to delete emails.

#### Scenario: delete_email command
- **WHEN** frontend invokes delete_email command with email ID
- **THEN** system calls database delete method
- **AND** system returns success or error result

### Requirement: Frontend delete email service

The system SHALL provide a service function for frontend to delete emails.

#### Scenario: Delete email service call
- **WHEN** frontend calls deleteEmail function with email ID
- **THEN** service invokes Tauri delete_email command
- **AND** service returns success or throws error

### Requirement: Mailbox store delete action

The system SHALL provide a store action to delete email from local state.

#### Scenario: Delete email from store
- **WHEN** user triggers delete action with email ID
- **THEN** store calls deleteEmail service
- **AND** store removes email from emails array
- **AND** store clears selectedEmailId if deleted email was selected
- **AND** store shows success notification

#### Scenario: Delete confirmation
- **WHEN** user clicks delete button
- **THEN** system shows confirmation dialog with email subject
- **AND** system only proceeds if user confirms

### Requirement: Delete button functionality

The system SHALL bind delete button to actual deletion functionality.

#### Scenario: Delete normal email
- **WHEN** user clicks delete button on normal email
- **THEN** system shows confirmation dialog
- **AND** system deletes email after confirmation

#### Scenario: Delete phishing email
- **WHEN** user clicks delete button on phishing email
- **THEN** system shows confirmation dialog
- **AND** system deletes email after confirmation