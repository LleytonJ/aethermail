## MODIFIED Requirements

### Requirement: Compose new email

The system SHALL allow users to compose and send new emails with rich text content.

#### Scenario: Open compose window
- **WHEN** user clicks "Compose" button in sidebar
- **THEN** system opens email compose interface
- **AND** compose interface includes recipient, subject, and rich text body editor
- **AND** signature toggle is displayed if active account has signature

#### Scenario: Enter recipient
- **WHEN** user enters recipient email address
- **THEN** system validates email format
- **AND** accepts valid email addresses

#### Scenario: Enter subject
- **WHEN** user enters subject line
- **THEN** system accepts subject text

#### Scenario: Enter email body with rich text
- **WHEN** user enters email body in rich text editor
- **THEN** system accepts formatted HTML content
- **AND** supports bold, italic, lists, links, and images
- **AND** displays formatting toolbar

#### Scenario: Toggle signature inclusion
- **WHEN** user toggles signature switch
- **THEN** system enables or disables signature attachment
- **AND** toggle state is persisted during compose session

#### Scenario: Signature toggle default state
- **WHEN** compose window opens
- **AND** active account has non-empty signature
- **THEN** signature toggle is enabled by default
- **AND** signature will be included in email

#### Scenario: Signature toggle hidden when no signature
- **WHEN** compose window opens
- **AND** active account has no signature or empty signature
- **THEN** signature toggle is not displayed
- **AND** email will be sent without signature

#### Scenario: Send email
- **WHEN** user clicks "Send" button
- **AND** all required fields are filled
- **THEN** system sends email via SMTP with HTML body
- **AND** includes signature if toggle is enabled
- **AND** displays sending progress indicator
- **AND** shows success or failure message

---

### Requirement: Reply to email

The system SHALL allow users to reply to received emails with rich text.

#### Scenario: Reply to email
- **WHEN** user clicks "Reply" button in email detail view
- **THEN** system opens compose interface with rich text editor
- **AND** pre-fills recipient with original sender's email
- **AND** pre-fills subject with "Re: {original subject}"
- **AND** quotes original email body in reply
- **AND** signature toggle reflects account signature status

#### Scenario: Reply-all
- **WHEN** user clicks "Reply All" button
- **THEN** system pre-fills all recipients from original email
- **AND** includes CC recipients from original email

---

### Requirement: Forward email

The system SHALL allow users to forward received emails with rich text.

#### Scenario: Forward email
- **WHEN** user clicks "Forward" button in email detail view
- **THEN** system opens compose interface with rich text editor
- **AND** pre-fills subject with "Fwd: {original subject}"
- **AND** includes original email content as forwarded message
- **AND** leaves recipient field empty for user to specify
- **AND** signature toggle reflects account signature status

---

### Requirement: Send email using SMTP

The system SHALL send HTML emails through configured SMTP server.

#### Scenario: Send HTML email with SMTP
- **WHEN** user sends an email with rich text content
- **THEN** system retrieves SMTP configuration from active account
- **AND** connects to SMTP server
- **AND** authenticates with account credentials
- **AND** sends email with MIME HTML format

#### Scenario: Include signature in email
- **WHEN** user sends email with signature toggle enabled
- **AND** account has signature_html content
- **THEN** system appends signature HTML to email body
- **AND** signature is separated from body with divider line
- **AND** full HTML (body + signature) is sent via SMTP

#### Scenario: Send without signature
- **WHEN** user sends email with signature toggle disabled
- **OR** account has no signature configured
- **THEN** system sends email body only
- **AND** no signature content is included

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