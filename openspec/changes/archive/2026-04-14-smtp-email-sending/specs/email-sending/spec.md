## ADDED Requirements

### Requirement: Compose new email

The system SHALL allow users to compose and send new emails.

#### Scenario: Open compose window
- **WHEN** user clicks "Compose" button in sidebar
- **THEN** system opens email compose interface
- **AND** compose interface includes recipient, subject, and body fields

#### Scenario: Enter recipient
- **WHEN** user enters recipient email address
- **THEN** system validates email format
- **AND** accepts valid email addresses

#### Scenario: Enter subject
- **WHEN** user enters subject line
- **THEN** system accepts subject text

#### Scenario: Enter email body
- **WHEN** user enters email body text
- **THEN** system accepts plain text content

#### Scenario: Send email
- **WHEN** user clicks "Send" button
- **AND** all required fields are filled
- **THEN** system sends email via SMTP
- **AND** displays sending progress indicator
- **AND** shows success or failure message

---

### Requirement: Reply to email

The system SHALL allow users to reply to received emails.

#### Scenario: Reply to email
- **WHEN** user clicks "Reply" button in email detail view
- **THEN** system opens compose interface
- **AND** pre-fills recipient with original sender's email
- **AND** pre-fills subject with "Re: {original subject}"
- **AND** quotes original email body in reply

#### Scenario: Reply-all
- **WHEN** user clicks "Reply All" button
- **THEN** system pre-fills all recipients from original email
- **AND** includes CC recipients from original email

---

### Requirement: Forward email

The system SHALL allow users to forward received emails.

#### Scenario: Forward email
- **WHEN** user clicks "Forward" button in email detail view
- **THEN** system opens compose interface
- **AND** pre-fills subject with "Fwd: {original subject}"
- **AND** includes original email content as forwarded message
- **AND** leaves recipient field empty for user to specify

---

### Requirement: Send email using SMTP

The system SHALL send emails through configured SMTP server.

#### Scenario: Send with SMTP
- **WHEN** user sends an email
- **THEN** system retrieves SMTP configuration from active account
- **AND** connects to SMTP server
- **AND** authenticates with account credentials
- **AND** sends email with MIME format

#### Scenario: Send failure handling
- **WHEN** SMTP send fails
- **THEN** system displays error message
- **AND** allows user to retry sending

#### Scenario: From address
- **WHEN** system sends email
- **THEN** email "From" address uses active account's email address
- **AND** display name uses account's display name if configured

---

### Requirement: Email sending status feedback

The system SHALL provide visual feedback during email sending.

#### Scenario: Sending progress
- **WHEN** email is being sent
- **THEN** system shows sending progress indicator
- **AND** disables send button during sending

#### Scenario: Send success
- **WHEN** email is successfully sent
- **THEN** system shows success notification
- **AND** closes compose interface
- **AND** clears compose form

#### Scenario: Send failure
- **WHEN** email sending fails
- **THEN** system shows error notification with failure reason
- **AND** preserves email content for retry
- **AND** keeps compose interface open