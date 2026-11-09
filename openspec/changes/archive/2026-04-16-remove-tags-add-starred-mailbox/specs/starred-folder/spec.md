## ADDED Requirements

### Requirement: Starred folder display

The system SHALL provide a dedicated starred folder in the sidebar for displaying starred emails.

#### Scenario: Show starred folder in sidebar
- **WHEN** user views the sidebar
- **THEN** system displays "星标邮件" folder entry
- **AND** folder uses Star icon from lucide-vue-next
- **AND** folder shows count of starred emails if any exist

#### Scenario: Navigate to starred folder
- **WHEN** user clicks starred folder in sidebar
- **THEN** system displays list of emails where is_starred = true
- **AND** system sorts emails by date descending
- **AND** system excludes phishing emails from starred folder

#### Scenario: Empty starred folder
- **WHEN** starred folder has no emails
- **THEN** system displays empty state message
- **AND** system shows guidance on starring emails

### Requirement: Starred email count badge

The system SHALL display a badge indicating starred email count.

#### Scenario: Show count badge
- **WHEN** starred folder has emails
- **THEN** system displays badge with count
- **AND** count reflects total starred emails (not unread starred)

#### Scenario: Update count on star change
- **WHEN** user toggles star status on any email
- **THEN** system updates starred folder count immediately
- **AND** count increases when email is starred
- **AND** count decreases when email is unstarred

### Requirement: Starred folder removal behavior

The system SHALL remove emails from starred folder when unstarred.

#### Scenario: Email unstarred in starred folder
- **WHEN** user unstars an email while viewing starred folder
- **THEN** email is removed from starred folder view immediately
- **AND** email remains in original folder (inbox or phishing)
- **AND** starred count is updated

#### Scenario: Email starred in other folder
- **WHEN** user stars an email in inbox or phishing folder
- **THEN** email appears in starred folder
- **AND** email also remains visible in original folder