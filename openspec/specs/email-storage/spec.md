## ADDED Requirements

### Requirement: SQLite database initialization

The system SHALL initialize SQLite database for email storage.

#### Scenario: Create database on first run
- **WHEN** application starts for the first time
- **THEN** system creates SQLite database with required tables

#### Scenario: Database schema migration
- **WHEN** database schema needs update
- **THEN** system applies migrations without data loss

### Requirement: Email persistence

The system SHALL persist emails to SQLite database.

#### Scenario: Store new email
- **WHEN** email is downloaded from server
- **THEN** system stores email in database with all fields

#### Scenario: Retrieve emails by account
- **WHEN** frontend requests emails for an account
- **THEN** system returns all emails for that account sorted by date

#### Scenario: Retrieve single email
- **WHEN** frontend requests a specific email
- **THEN** system returns the email with full body and attachments

### Requirement: Email state management

The system SHALL persist email state (read, starred) locally.

#### Scenario: Mark email as read
- **WHEN** user opens an email
- **THEN** system updates is_read flag in database

#### Scenario: Toggle star status
- **WHEN** user stars or unstars an email
- **THEN** system updates is_starred flag in database

### Requirement: Attachment storage

The system SHALL store email attachments.

#### Scenario: Store attachment metadata
- **WHEN** email has attachments
- **THEN** system stores filename, MIME type, and size

#### Scenario: Store attachment content
- **WHEN** attachment is extracted from email
- **THEN** system stores content in database or filesystem

#### Scenario: Retrieve attachment
- **WHEN** user requests to view or download attachment
- **THEN** system retrieves attachment content

### Requirement: Phishing email storage fields

The system SHALL store phishing detection results in the email database.

#### Scenario: Store phishing detection result
- **WHEN** phishing detection completes for an email
- **THEN** system stores is_phishing boolean field
- **AND** system stores phishing_reasons text field (JSON array of strings)

#### Scenario: Retrieve phishing emails
- **WHEN** user requests phishing folder contents
- **THEN** system returns emails where is_phishing = true
- **AND** system sorts by date descending

### Requirement: Email folder assignment for phishing

The system SHALL assign folder based on phishing detection result.

#### Scenario: Assign to phishing folder
- **WHEN** email is detected as phishing
- **THEN** system sets is_phishing = true
- **AND** email is listed in phishing folder view

#### Scenario: Keep in original folder
- **WHEN** email is not detected as phishing
- **THEN** system sets is_phishing = false
- **AND** email remains in original folder

### Requirement: Database schema for phishing

The system SHALL extend email database schema with phishing fields.

#### Scenario: Database migration
- **WHEN** application starts with new version
- **THEN** system adds is_phishing column (INTEGER DEFAULT 0)
- **AND** system adds phishing_reasons column (TEXT DEFAULT NULL)

#### Scenario: Query optimization
- **WHEN** system queries phishing emails frequently
- **THEN** system creates index on is_phishing column

### Requirement: Starred email retrieval

The system SHALL provide methods to retrieve starred emails from database.

#### Scenario: Get all starred emails for account
- **WHEN** frontend requests starred emails for an account
- **THEN** system returns all emails where is_starred = true
- **AND** system excludes phishing emails (is_phishing = false)
- **AND** system sorts by date descending

#### Scenario: Get starred count for account
- **WHEN** frontend requests starred count for an account
- **THEN** system returns total count of starred emails
- **AND** count excludes phishing emails

### Requirement: Backend commands for starred emails

The system SHALL provide Tauri commands for starred email operations.

#### Scenario: get_starred_emails command
- **WHEN** frontend invokes get_starred_emails command
- **THEN** system queries database for starred emails
- **AND** system returns array of Email objects
- **AND** system excludes phishing emails

#### Scenario: get_starred_count command
- **WHEN** frontend invokes get_starred_count command
- **THEN** system queries database for starred count
- **AND** system returns integer count

### Requirement: Delete single email method

The system SHALL provide a method to delete a single email record.

#### Scenario: Delete email by ID
- **WHEN** delete_email method is called with an email ID
- **THEN** system executes DELETE query on emails table
- **AND** system returns success result
- **AND** attachments are automatically deleted via CASCADE

#### Scenario: Delete non-existent email
- **WHEN** delete_email method is called with invalid ID
- **THEN** system returns success (DELETE affects 0 rows)
- **AND** system does not throw error