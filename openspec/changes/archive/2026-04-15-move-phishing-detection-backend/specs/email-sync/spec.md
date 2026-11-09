## MODIFIED Requirements

### Requirement: Phishing detection during sync

The system SHALL integrate phishing detection into the email synchronization process with atomic save operation.

#### Scenario: Detect phishing on new email
- **WHEN** email is downloaded during sync
- **AND** account has phishing detection enabled
- **THEN** system calls backend phishing detection before database save
- **AND** system waits for detection result (with timeout)
- **AND** system saves email to database with correct is_phishing flag
- **AND** phishing emails are never visible in inbox

#### Scenario: Continue sync on detection failure
- **WHEN** phishing detection fails or times out
- **THEN** system saves email with is_phishing = false
- **AND** system logs detection failure for that email
- **AND** system continues syncing remaining emails

#### Scenario: Skip detection for user-overridden emails
- **WHEN** email already exists in database with user_overridden = true
- **THEN** system does not re-run phishing detection
- **AND** system preserves user_overridden flag and folder position

#### Scenario: Skip detection for existing emails
- **WHEN** email UIDL already exists in database
- **THEN** system skips downloading that email
- **AND** system does not re-run phishing detection

### Requirement: Sync progress notification

The system SHALL report phishing detection progress to frontend.

#### Scenario: Report detection progress
- **WHEN** phishing detection is in progress
- **THEN** system reports "正在检测钓鱼邮件..." to frontend
- **AND** frontend displays detection status to user

#### Scenario: Report phishing count
- **WHEN** sync completes with phishing emails detected
- **THEN** result includes count of phishing emails detected
- **AND** frontend can update phishing folder badge