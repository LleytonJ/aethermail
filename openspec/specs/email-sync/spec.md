## ADDED Requirements

### Requirement: Email synchronization

The system SHALL synchronize emails from POP3 server to local storage.

#### Scenario: Initial sync
- **WHEN** user adds a new account
- **THEN** system downloads all emails from the server

#### Scenario: Incremental sync
- **WHEN** user syncs an existing account
- **THEN** system only downloads new emails not previously synced

#### Scenario: Sync progress notification
- **WHEN** sync is in progress
- **THEN** system reports progress to the frontend

#### Scenario: Sync error handling
- **WHEN** sync encounters an error
- **THEN** system logs the error and continues with remaining emails

### Requirement: UIDL-based tracking

The system SHALL track synced emails using POP3 UIDL command.

#### Scenario: Store UIDL
- **WHEN** an email is successfully downloaded
- **THEN** system stores the UIDL in local database

#### Scenario: Check existing UIDLs
- **WHEN** sync starts
- **THEN** system retrieves list of existing UIDLs from database

#### Scenario: Skip downloaded emails
- **WHEN** email UIDL already exists in database
- **THEN** system skips downloading that email

### Requirement: Email parsing

The system SHALL parse downloaded emails into structured data.

#### Scenario: Parse MIME email
- **WHEN** raw email is downloaded
- **THEN** system extracts subject, from, to, date, body

#### Scenario: Extract HTML body
- **WHEN** email contains HTML content
- **THEN** system extracts HTML body

#### Scenario: Extract plain text body
- **WHEN** email contains plain text content
- **THEN** system extracts plain text body

#### Scenario: Extract attachments
- **WHEN** email contains attachments
- **THEN** system extracts attachment metadata and content

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