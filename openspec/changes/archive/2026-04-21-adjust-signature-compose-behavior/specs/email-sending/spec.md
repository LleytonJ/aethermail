## MODIFIED Requirements

### Requirement: Compose new email

The system SHALL allow users to compose and send new emails with rich text content.

#### Scenario: Open compose window
- **WHEN** user clicks "Compose" button in sidebar
- **THEN** system opens email compose interface
- **AND** compose interface includes recipient, subject, and rich text body editor
- **AND** signature is automatically appended to email body if account has signature configured

#### Scenario: Signature appended on compose
- **WHEN** compose window opens
- **AND** active account has non-empty signature configured
- **THEN** system appends signature HTML to the bottom of email body in editor
- **AND** signature is visible and editable in the rich text editor

#### Scenario: No signature on compose
- **WHEN** compose window opens
- **AND** active account has no signature or empty signature
- **THEN** system opens compose editor without any signature content

#### Scenario: Enter email body with rich text
- **WHEN** user enters email body in rich text editor
- **THEN** system accepts formatted HTML content
- **AND** supports bold, italic, lists, links, and images
- **AND** displays formatting toolbar

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

## REMOVED Requirements

### Requirement: Toggle signature inclusion

**Reason**: Signature toggle button removed from compose page; signature is now automatically appended when account has signature configured, and user can edit/delete in the editor.

**Migration**: Users should configure signature in account settings. Signature will be automatically appended to all emails. Users can delete signature in the compose editor if they don't want it for a specific email.

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