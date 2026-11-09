## MODIFIED Requirements

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