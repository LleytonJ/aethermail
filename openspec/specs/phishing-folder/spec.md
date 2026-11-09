## ADDED Requirements

### Requirement: Phishing folder display

The system SHALL provide a dedicated phishing folder in the sidebar for displaying detected phishing emails.

#### Scenario: Show phishing folder in sidebar
- **WHEN** user views the sidebar
- **THEN** system displays "钓鱼邮件" folder entry
- **AND** folder shows count of phishing emails if any exist

#### Scenario: Navigate to phishing folder
- **WHEN** user clicks phishing folder in sidebar
- **THEN** system displays list of emails marked as phishing
- **AND** system sorts emails by date descending

#### Scenario: Empty phishing folder
- **WHEN** phishing folder has no emails
- **THEN** system displays empty state message
- **AND** system shows guidance on enabling phishing detection

### Requirement: Phishing email display

The system SHALL display phishing emails with detection indicators.

#### Scenario: Show detection reasons
- **WHEN** user views phishing email in list or detail
- **THEN** system displays detection reasons from AI analysis
- **AND** reasons include specific indicators found (e.g., "可疑链接", "伪造发件人")

#### Scenario: Show sender warning
- **WHEN** user views phishing email
- **THEN** system displays prominent warning banner
- **AND** banner advises user not to click links or respond

### Requirement: Phishing email actions

The system SHALL provide actions for managing phishing emails.

#### Scenario: Move to inbox
- **WHEN** user clicks "移回收件箱" action on phishing email
- **THEN** system updates email is_phishing = false
- **AND** system clears phishing_reasons field
- **AND** system moves email to inbox folder

#### Scenario: Mark as confirmed phishing
- **WHEN** user clicks "确认是钓鱼" action on suspected email
- **THEN** system updates is_phishing = true
- **AND** system moves email to phishing folder

#### Scenario: Delete phishing email
- **WHEN** user deletes phishing email
- **THEN** system removes email from database
- **AND** system does not require confirmation beyond normal delete flow

### Requirement: Phishing folder badge

The system SHALL display a badge indicating phishing email count.

#### Scenario: Show count badge
- **WHEN** phishing folder has emails
- **THEN** system displays badge with count
- **AND** badge uses distinctive color (e.g., red)

#### Scenario: Badge with new phishing
- **WHEN** new phishing emails are detected
- **THEN** system updates badge count immediately