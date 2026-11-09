## ADDED Requirements

### Requirement: Account editing capability

The system SHALL allow users to edit existing account configuration.

#### Scenario: Edit button availability
- **WHEN** user views account management page
- **THEN** system displays edit button for each account

#### Scenario: Open edit modal
- **WHEN** user clicks edit button on an account
- **THEN** system opens edit modal with pre-filled account data

#### Scenario: Edit modal displays account data
- **WHEN** edit modal opens
- **THEN** system displays current account email, display name, and server settings
- **AND** password field is empty with hint "不修改请留空"

### Requirement: Edit account form fields

The system SHALL allow editing the following account properties:

- Display name
- Password (optional)
- POP3 server host
- POP3 server port
- Security type (SSL/TLS or None)

#### Scenario: Modify display name
- **WHEN** user changes display name in edit modal
- **THEN** system updates display name field

#### Scenario: Modify server settings
- **WHEN** user changes server host or port
- **THEN** system updates server configuration fields

#### Scenario: Password change optional
- **WHEN** user leaves password field empty
- **THEN** system preserves existing password

### Requirement: Save edited account

The system SHALL save account changes when user confirms.

#### Scenario: Save account changes
- **WHEN** user clicks save button after editing
- **THEN** system updates account configuration in storage
- **AND** system displays success notification
- **AND** system closes edit modal

#### Scenario: Cancel editing
- **WHEN** user clicks cancel button
- **THEN** system discards all changes
- **AND** system closes edit modal

### Requirement: Connection test during edit

The system SHALL optionally allow testing connection before saving edited account.

#### Scenario: Test connection button
- **WHEN** user is in edit mode
- **THEN** system displays test connection button

#### Scenario: Connection test execution
- **WHEN** user clicks test connection
- **THEN** system performs mock connection test
- **AND** system displays test result