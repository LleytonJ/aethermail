## ADDED Requirements

### Requirement: Phishing detection during sync

The system SHALL integrate phishing detection into the email synchronization process.

#### Scenario: Detect phishing on new email
- **WHEN** email is downloaded during sync
- **AND** account has phishing detection enabled
- **THEN** system calls phishing detection service
- **AND** system waits for detection result (with timeout)
- **AND** system stores detection result with email

#### Scenario: Continue sync on detection failure
- **WHEN** phishing detection fails or times out
- **THEN** system continues syncing remaining emails
- **AND** system logs detection failure for that email
- **AND** system stores email without phishing mark

#### Scenario: Batch detection for multiple emails
- **WHEN** multiple emails are downloaded in one sync session
- **THEN** system may batch detection calls to optimize API usage
- **AND** system limits concurrent detection calls to avoid rate limits

#### Scenario: Skip detection for existing emails
- **WHEN** email UIDL already exists in database
- **THEN** system skips downloading that email
- **AND** system does not re-run phishing detection