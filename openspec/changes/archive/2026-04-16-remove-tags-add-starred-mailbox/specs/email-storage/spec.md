## ADDED Requirements

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