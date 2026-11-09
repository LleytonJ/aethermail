## ADDED Requirements

### Requirement: User can view account switcher in sidebar

The system SHALL display an account switcher component in the sidebar showing the active account.

#### Scenario: Display active account
- **WHEN** an account is active
- **THEN** sidebar shows account avatar (first letter), display name, and email address

#### Scenario: No accounts configured
- **WHEN** no accounts exist in the system
- **THEN** account switcher displays "Add Account" button with user-plus icon

#### Scenario: Account switcher dropdown
- **WHEN** user clicks on the account switcher
- **THEN** a dropdown menu appears showing all configured accounts and "Add account" option

---

### Requirement: User can switch between accounts

The system SHALL allow users to switch the active account from the account switcher dropdown.

#### Scenario: Switch active account
- **WHEN** user selects a different account from the dropdown
- **THEN** the selected account becomes active
- **AND** the dropdown closes
- **AND** the sidebar updates to show the new active account

#### Scenario: Multiple accounts display
- **WHEN** multiple accounts are configured
- **THEN** each account shows display name, email, and a checkmark for the active one

---

### Requirement: User can open add account modal

The system SHALL provide a way to open the add account modal from multiple entry points.

#### Scenario: Open from account switcher
- **WHEN** user clicks "Add account" in the account switcher dropdown
- **THEN** the add account modal opens

#### Scenario: Open from empty state
- **WHEN** no accounts exist and user clicks the "Add Account" button
- **THEN** the add account modal opens

#### Scenario: Close modal
- **WHEN** user clicks Cancel, presses Escape, or clicks outside the modal
- **THEN** the modal closes without saving

---

### Requirement: User can select email provider type

The system SHALL allow users to select their email provider type in the add account modal.

#### Scenario: Display provider options
- **WHEN** add account modal opens
- **THEN** system displays available provider options (POP3/SMTP, with Gmail/OAuth2 shown as coming soon)

#### Scenario: Select POP3 provider
- **WHEN** user selects POP3/SMTP option
- **THEN** system navigates to the POP3 account configuration form

---

### Requirement: User can configure POP3 account through multi-step form

The system SHALL guide users through a 4-step wizard to configure a POP3 account.

#### Scenario: Step indicator display
- **WHEN** user is on any step of the form
- **THEN** a step indicator shows current progress (Account → Incoming → Test → Done)

#### Scenario: Navigate between steps
- **WHEN** user clicks Next or Back buttons
- **THEN** system navigates to the corresponding step
- **AND** form data is preserved

---

### Requirement: User can enter basic account information

The system SHALL allow users to enter email address, display name, and password in step 1.

#### Scenario: Enter email address
- **WHEN** user enters an email address
- **THEN** the field accepts the input
- **AND** if email domain matches a preset, auto-fill is triggered

#### Scenario: Email validation
- **WHEN** user enters an invalid email format
- **THEN** system displays validation error

#### Scenario: Password input
- **WHEN** user enters password
- **THEN** characters are masked

---

### Requirement: System auto-fills server settings from presets

The system SHALL automatically fill POP3 server settings when the email domain matches a known provider.

#### Scenario: Gmail auto-fill
- **WHEN** user enters an email ending with @gmail.com
- **THEN** system auto-fills host: pop.gmail.com, port: 995, security: SSL

#### Scenario: 163 auto-fill
- **WHEN** user enters an email ending with @163.com
- **THEN** system auto-fills host: pop.163.com, port: 110, security: None

#### Scenario: Custom domain
- **WHEN** user enters an email with unknown domain
- **THEN** server settings remain empty for manual input

---

### Requirement: User can manually configure POP3 server settings

The system SHALL allow users to manually modify server host, port, and security settings.

#### Scenario: Edit server host
- **WHEN** user modifies the server host field
- **THEN** the value updates

#### Scenario: Edit port
- **WHEN** user modifies the port field
- **THEN** the value updates

#### Scenario: Toggle SSL
- **WHEN** user toggles SSL/TLS switch
- **THEN** the security setting updates

---

### Requirement: User can test POP3 connection

The system SHALL provide a mock connection test that simulates server verification.

#### Scenario: Successful mock test
- **WHEN** user clicks "Test Connection" with valid-looking settings
- **AND** password does not contain "wrong"
- **THEN** system shows a loading state for ~1.5 seconds
- **AND** displays success message with mock server info

#### Scenario: Failed mock test
- **WHEN** user clicks "Test Connection" with password containing "wrong"
- **THEN** system shows a loading state for ~1.5 seconds
- **AND** displays authentication error message

#### Scenario: Re-test connection
- **WHEN** test completes (success or failure)
- **THEN** user can click "Test Again" to retry

---

### Requirement: User can save account

The system SHALL allow users to save the configured account after successful test.

#### Scenario: Save after successful test
- **WHEN** connection test succeeds
- **AND** user clicks "Add Account"
- **THEN** account is added to the account list
- **AND** modal shows success confirmation

#### Scenario: Cannot save without test
- **WHEN** user has not tested connection
- **THEN** "Add Account" button is disabled

#### Scenario: Save to localStorage
- **WHEN** account is saved
- **THEN** account data is stored in localStorage for persistence

---

### Requirement: Account persists across sessions

The system SHALL persist account configuration across application restarts.

#### Scenario: Load accounts on startup
- **WHEN** application starts
- **THEN** previously saved accounts are loaded from localStorage

#### Scenario: Restore last active account
- **WHEN** application starts with saved accounts
- **THEN** the last active account becomes active