## ADDED Requirements

### Requirement: SMTP server configuration step in add account flow

The system SHALL include SMTP server configuration in the add account wizard.

#### Scenario: SMTP config step
- **WHEN** user completes incoming server configuration
- **THEN** system navigates to SMTP server configuration step
- **AND** step indicator shows "SMTP" or "发件服务器" as next step

#### Scenario: SMTP auto-fill from preset
- **WHEN** user's email domain matches a preset provider
- **THEN** system auto-fills SMTP host, port, and SSL settings
- **AND** displays hint that settings were auto-filled

#### Scenario: SMTP manual configuration
- **WHEN** user needs to customize SMTP settings
- **THEN** system allows editing SMTP host, port, and SSL settings

---

### Requirement: SMTP connection test in add account flow

The system SHALL test SMTP connection during account setup.

#### Scenario: SMTP test step
- **WHEN** user reaches test connection step
- **THEN** system tests both incoming server and SMTP server
- **AND** displays separate results for each

#### Scenario: Combined test display
- **WHEN** connection tests complete
- **THEN** system shows POP3/IMAP test result
- **AND** shows SMTP test result
- **AND** both must succeed to enable "Add Account" button

---

### Requirement: Compose button in sidebar

The system SHALL provide a compose button in the sidebar.

#### Scenario: Compose button visible
- **WHEN** sidebar is visible
- **THEN** compose button appears at top of sidebar
- **AND** button shows "撰写" or pencil icon

#### Scenario: Compose button click
- **WHEN** user clicks compose button
- **THEN** system opens compose interface in main content area

---

### Requirement: Reply and Forward buttons in email detail

The system SHALL provide reply and forward actions in email detail view.

#### Scenario: Reply button visible
- **WHEN** user views email detail
- **THEN** reply button appears in toolbar
- **AND** button shows "回复" or reply icon

#### Scenario: Forward button visible
- **WHEN** user views email detail
- **THEN** forward button appears in toolbar
- **AND** button shows "转发" or forward icon