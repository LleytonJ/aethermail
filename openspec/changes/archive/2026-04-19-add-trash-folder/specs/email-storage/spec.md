## ADDED Requirements

### Requirement: Database schema for trash folder

The system SHALL extend email database schema with deleted_at field.

#### Scenario: Database migration for deleted_at
- **WHEN** application starts with new version
- **THEN** system adds deleted_at column (INTEGER DEFAULT NULL)

#### Scenario: Query optimization for trash
- **WHEN** system queries trash emails frequently
- **THEN** system creates index on deleted_at column

### Requirement: Trash email queries

The system SHALL provide methods to query deleted emails.

#### Scenario: Get trash emails for account
- **WHEN** frontend requests trash emails for an account
- **THEN** system returns emails where deleted_at IS NOT NULL
- **AND** system sorts by deleted_at descending

#### Scenario: Get trash count for account
- **WHEN** frontend requests trash count for an account
- **THEN** system returns count of emails where deleted_at IS NOT NULL

### Requirement: Email soft delete

The system SHALL provide a method to soft delete emails.

#### Scenario: Move email to trash
- **WHEN** move_to_trash method is called with email ID
- **THEN** system sets deleted_at to current timestamp
- **AND** system does not remove email record

### Requirement: Email restore from trash

The system SHALL provide methods to restore emails from trash.

#### Scenario: Restore email to inbox
- **WHEN** restore_email method is called with email ID and target "inbox"
- **THEN** system sets deleted_at to NULL
- **AND** system sets is_phishing to false

#### Scenario: Restore email to phishing folder
- **WHEN** restore_email method is called with email ID and target "phishing"
- **THEN** system sets deleted_at to NULL
- **AND** system sets is_phishing to true

### Requirement: Permanent delete method

The system SHALL provide a method to permanently delete emails.

#### Scenario: Permanent delete by ID
- **WHEN** permanent_delete method is called with email ID
- **THEN** system executes DELETE query on emails table
- **AND** system removes all attachments via CASCADE

### Requirement: Auto cleanup expired emails

The system SHALL provide a method to clean up expired trash emails.

#### Scenario: Cleanup emails older than 30 days
- **WHEN** cleanup_expired_trash method is called
- **THEN** system permanently deletes emails where deleted_at < (now - 30 days)