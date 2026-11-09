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

---

## MODIFIED Requirements (from composer-rich-text-signatures)

### Requirement: Compose new email

The system SHALL allow users to compose and send new emails with rich text content.

#### Scenario: Open compose window
- **WHEN** user clicks "Compose" button in sidebar
- **THEN** system opens email compose interface
- **AND** compose interface includes recipient, subject, and rich text body editor

#### Scenario: Enter email body with rich text
- **WHEN** user enters email body in rich text editor
- **THEN** system accepts formatted HTML content
- **AND** supports bold, italic, lists, links, and images
- **AND** displays formatting toolbar

#### Scenario: Send email
- **WHEN** user clicks "Send" button
- **AND** all required fields are filled
- **THEN** system sends email via SMTP with HTML body
- **AND** displays sending progress indicator
- **AND** shows success or failure message

---

### Requirement: Send email using SMTP

The system SHALL send HTML emails through configured SMTP server.

#### Scenario: Send HTML email with SMTP
- **WHEN** user sends an email with rich text content
- **THEN** system retrieves SMTP configuration from active account
- **AND** connects to SMTP server
- **AND** authenticates with account credentials
- **AND** sends email with MIME HTML format

---

## MODIFIED Requirements (from adjust-signature-compose-behavior)

### Requirement: Compose new email

The system SHALL allow users to compose and send new emails with rich text content.

#### Scenario: Open compose window
- **WHEN** user clicks "Compose" button in sidebar
- **AND** active account has non-empty signature enabled
- **THEN** system displays signature attachment dialog first
- **AND** after user decision, opens email compose interface
- **AND** compose interface includes recipient, subject, and rich text body editor

#### Scenario: Open compose window without signature
- **WHEN** user clicks "Compose" button in sidebar
- **AND** active account has no signature or empty signature
- **THEN** system opens email compose interface directly
- **AND** compose interface includes recipient, subject, and rich text body editor

#### Scenario: Signature appended on compose
- **WHEN** compose window opens
- **AND** active account has non-empty signature configured
- **THEN** system appends signature HTML to the bottom of email body in editor
- **AND** signature is visible and editable in the rich text editor

#### Scenario: No signature on compose
- **WHEN** compose window opens
- **AND** active account has no signature or empty signature
- **THEN** system opens compose editor without any signature content

#### Scenario: User edits or removes signature
- **WHEN** signature is appended to email body
- **THEN** user can edit the signature content in editor
- **AND** user can delete the signature before sending

#### Scenario: Send email
- **WHEN** user clicks "Send" button
- **AND** all required fields are filled
- **THEN** system sends email via SMTP with HTML body
- **AND** includes whatever content is in editor (with or without signature)
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
- **AND** appends signature to the bottom if account has signature configured

#### Scenario: Reply-all
- **WHEN** user clicks "Reply All" button
- **THEN** system pre-fills all recipients from original email
- **AND** includes CC recipients from original email
- **AND** appends signature to the bottom if account has signature configured

---

### Requirement: Forward email

The system SHALL allow users to forward received emails.

#### Scenario: Forward email
- **WHEN** user clicks "Forward" button in email detail view
- **THEN** system opens compose interface
- **AND** pre-fills subject with "Fwd: {original subject}"
- **AND** includes original email content as forwarded message
- **AND** leaves recipient field empty for user to specify
- **AND** appends signature to the bottom if account has signature configured

---

## MODIFIED Requirements (from tiptap-editor-extensions)

### Requirement: Compose new email

The system SHALL allow users to compose and send new emails with rich text content.

#### Scenario: Open compose window
- **WHEN** user clicks "Compose" button in sidebar
- **THEN** system opens email compose interface
- **AND** compose interface includes recipient, subject, and rich text body editor
- **AND** rich text editor displays formatting toolbar

#### Scenario: Enter email body with rich text
- **WHEN** user enters email body in rich text editor
- **THEN** system accepts formatted HTML content
- **AND** supports bold, italic, underline, strikethrough
- **AND** supports heading levels H1, H2, H3
- **AND** supports ordered and unordered lists
- **AND** supports hyperlinks inserted via dialog
- **AND** supports inline images from local files

#### Scenario: Send email
- **WHEN** user clicks "Send" button
- **AND** all required fields are filled
- **THEN** system sends email via SMTP with HTML body
- **AND** HTML body preserves all formatting (bold, italic, underline, strike, headings, lists, links, images)
- **AND** displays sending progress indicator
- **AND** shows success or failure message