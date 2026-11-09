## MODIFIED Requirements

### Requirement: Send email using SMTP

The system SHALL send emails with attachments through configured SMTP server.

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

#### Scenario: Send email with attachments
- **WHEN** user sends an email with attachments
- **THEN** system builds MIME message with multipart/mixed structure
- **AND** attachments are encoded and included as MIME parts
- **AND** each attachment has appropriate Content-Type header
- **AND** each attachment has Content-Disposition: attachment header

#### Scenario: Send HTML email with attachments
- **WHEN** user sends an email with rich text content and attachments
- **THEN** system builds MIME structure: multipart/mixed containing multipart/alternative
- **AND** multipart/alternative contains text/plain and text/html parts
- **AND** attachments are added as separate parts after the body

#### Scenario: Attachment encoding
- **WHEN** system includes an attachment in MIME message
- **THEN** attachment content is Base64 encoded
- **AND** Content-Transfer-Encoding header is set to base64

---

### Requirement: Email sending status feedback

The system SHALL provide visual feedback during email sending including attachment upload progress.

#### Scenario: Sending progress
- **WHEN** email is being sent
- **THEN** system shows sending progress indicator
- **AND** disables send button during sending

#### Scenario: Send success
- **WHEN** email is successfully sent
- **THEN** system shows success notification
- **AND** closes compose interface
- **AND** clears compose form and attachment list

#### Scenario: Send failure
- **WHEN** email sending fails
- **THEN** system shows error notification with failure reason
- **AND** preserves email content and attachments for retry
- **AND** keeps compose interface open

#### Scenario: Attachment file not found
- **WHEN** system attempts to send email with attachment
- **AND** attachment file path is no longer valid
- **THEN** system shows error notification indicating file not found
- **AND** preserves other email content
- **AND** allows user to remove the missing attachment or re-add it