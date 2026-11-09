## ADDED Requirements

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