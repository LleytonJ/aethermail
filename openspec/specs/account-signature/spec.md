## ADDED Requirements

### Requirement: Account signature setting

The system SHALL allow each account to have one rich text signature.

#### Scenario: Signature section in account edit
- **WHEN** user opens account edit form
- **THEN** system displays signature section below server settings
- **AND** signature section contains enable toggle and rich text editor

#### Scenario: Enable/disable signature
- **WHEN** user toggles signature enable switch
- **THEN** system enables or disables signature editor
- **AND** disabled signature will not be appended to emails

#### Scenario: Edit signature content
- **WHEN** user edits signature in rich text editor
- **THEN** system captures formatted content as HTML
- **AND** supports bold, italic, lists, links, and images

#### Scenario: Clear signature
- **WHEN** user clears signature editor content
- **THEN** system stores empty signature
- **AND** no signature will be appended to emails

---

### Requirement: Signature persistence

The system SHALL persist signature HTML to account storage.

#### Scenario: Save signature with account
- **WHEN** user saves account changes
- **THEN** system stores signature_html field with account
- **AND** signature persists across application restarts

#### Scenario: Load signature on account edit
- **WHEN** user opens account edit form
- **THEN** system loads existing signature_html into editor
- **AND** toggle reflects signature enable status

---

### Requirement: Signature per account

The system SHALL maintain one signature per account independently.

#### Scenario: Each account has own signature
- **WHEN** user edits different accounts
- **THEN** system shows respective signature for each account
- **AND** signatures are not shared between accounts

#### Scenario: Signature isolation
- **WHEN** user modifies signature for account A
- **THEN** signature for account B remains unchanged