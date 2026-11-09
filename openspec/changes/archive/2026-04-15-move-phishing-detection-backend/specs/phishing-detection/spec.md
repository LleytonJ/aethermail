## MODIFIED Requirements

### Requirement: Phishing detection service

The system SHALL provide a phishing detection service that analyzes emails using AI to identify potential phishing attempts. Detection is executed in Rust backend during sync.

#### Scenario: Detect phishing email
- **WHEN** email is downloaded during sync and phishing detection is enabled for the account
- **THEN** system calls backend AI client using async-openai
- **AND** system waits for detection result before saving
- **AND** system saves email with correct is_phishing flag directly

#### Scenario: Skip detection when disabled
- **WHEN** email is downloaded and phishing detection is disabled for the account
- **THEN** system does not call AI provider
- **AND** email is saved with is_phishing = false directly

#### Scenario: Detection timeout handling
- **WHEN** AI detection call exceeds 5 second timeout
- **THEN** system saves email with is_phishing = false
- **AND** system logs timeout warning
- **AND** email appears in inbox (not phishing folder)

#### Scenario: Detection error handling
- **WHEN** AI detection call fails with error
- **THEN** system saves email with is_phishing = false
- **AND** system logs error without blocking sync

### Requirement: User override tracking

The system SHALL track when user manually overrides phishing classification.

#### Scenario: Mark user override when moving to inbox
- **WHEN** user moves phishing email back to inbox
- **THEN** system sets user_overridden = true
- **AND** system sets is_phishing = false

#### Scenario: Skip detection for overridden emails
- **WHEN** email has user_overridden = true
- **AND** sync detects same email again
- **THEN** system does not re-run phishing detection
- **AND** system preserves user_overridden flag
- **AND** email remains in inbox

#### Scenario: Database schema update
- **WHEN** system initializes database
- **THEN** emails table includes user_overridden column (boolean, default false)