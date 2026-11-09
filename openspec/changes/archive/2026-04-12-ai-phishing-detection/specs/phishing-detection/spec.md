## ADDED Requirements

### Requirement: Phishing detection service

The system SHALL provide a phishing detection service that analyzes emails using AI to identify potential phishing attempts.

#### Scenario: Detect phishing email
- **WHEN** email is downloaded during sync and phishing detection is enabled for the account
- **THEN** system calls AI provider to analyze the email for phishing indicators
- **AND** system receives detection result with is_phishing flag and reasons array

#### Scenario: Skip detection when disabled
- **WHEN** email is downloaded and phishing detection is disabled for the account
- **THEN** system does not call AI provider
- **AND** email is stored with is_phishing = false

#### Scenario: Detection timeout handling
- **WHEN** AI detection call exceeds 5 second timeout
- **THEN** system marks email as is_phishing = false
- **AND** system logs timeout warning
- **AND** sync continues without blocking

#### Scenario: Detection error handling
- **WHEN** AI detection call fails with error
- **THEN** system marks email as is_phishing = false
- **AND** system logs error without blocking sync

### Requirement: Phishing prompt configuration

The system SHALL use the phishing-prompt.md file content directly as the AI System Prompt.

#### Scenario: Load prompt from file
- **WHEN** phishing detection is triggered
- **THEN** system reads phishing-prompt.md file content
- **AND** system uses file content directly as System Prompt (no template processing)

#### Scenario: Create default prompt file
- **WHEN** phishing-prompt.md file does not exist
- **THEN** system creates file with default prompt content
- **AND** default content is a complete, ready-to-use System Prompt
- **AND** file path is same directory as accounts.json

#### Scenario: User customizes prompt
- **WHEN** user modifies phishing-prompt.md file
- **THEN** system uses customized content on next detection
- **AND** user has full control over the entire prompt structure

#### Scenario: Prompt file location
- **WHEN** system needs to access prompt file
- **THEN** system uses platform-specific path:
  - Windows: `%APPDATA%/aethermail/phishing-prompt.md`
  - macOS: `~/Library/Application Support/aethermail/phishing-prompt.md`
  - Linux: `~/.config/aethermail/phishing-prompt.md`

### Requirement: Phishing classification

The system SHALL classify emails as phishing or not phishing based on AI analysis.

#### Scenario: Mark as phishing
- **WHEN** AI returns is_phishing = true
- **THEN** system marks email as is_phishing = true
- **AND** system stores phishing_reasons array
- **AND** system moves email to phishing folder

#### Scenario: Mark as not phishing
- **WHEN** AI returns is_phishing = false
- **THEN** system marks email as is_phishing = false
- **AND** email remains in original folder
- **AND** system may store empty reasons array

### Requirement: AI response parsing

The system SHALL parse AI response as JSON and validate required fields.

#### Scenario: Parse valid JSON response
- **WHEN** AI returns valid JSON with is_phishing and reasons fields
- **THEN** system extracts is_phishing boolean value
- **AND** system extracts reasons array
- **AND** system stores parsed result

#### Scenario: Handle invalid JSON response
- **WHEN** AI response cannot be parsed as JSON
- **THEN** system returns safe default: is_phishing = false
- **AND** system stores reasons = ["AI 响应解析失败"]

#### Scenario: Validate is_phishing field
- **WHEN** JSON has is_phishing field but not boolean type
- **THEN** system coerces to false as safe default

#### Scenario: Validate reasons field
- **WHEN** JSON has reasons field but not array type
- **THEN** system uses empty array as default
- **AND** if is_phishing = true, system adds fallback reason