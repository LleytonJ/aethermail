## ADDED Requirements

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