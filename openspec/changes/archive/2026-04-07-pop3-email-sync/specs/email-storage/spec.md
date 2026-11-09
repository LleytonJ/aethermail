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