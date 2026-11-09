## ADDED Requirements

### Requirement: Account configuration storage

The system SHALL persist account configurations to a local JSON file.

#### Scenario: Save new account
- **WHEN** user adds a new account
- **THEN** system saves account configuration to accounts.json

#### Scenario: Load accounts on startup
- **WHEN** application starts
- **THEN** system loads all saved accounts from accounts.json

#### Scenario: Update account
- **WHEN** user modifies account settings
- **THEN** system updates the account in accounts.json

#### Scenario: Delete account
- **WHEN** user removes an account
- **THEN** system removes the account from accounts.json

### Requirement: Password storage

The system SHALL store account passwords in the configuration file (plaintext for current iteration).

#### Scenario: Store password
- **WHEN** user provides password during account setup
- **THEN** system stores password in accounts.json

#### Scenario: Retrieve password for sync
- **WHEN** system needs to authenticate with POP3 server
- **THEN** system retrieves password from accounts.json

### Requirement: Configuration file location

The system SHALL store configuration in a standard location.

#### Scenario: Windows configuration path
- **WHEN** running on Windows
- **THEN** system stores config at `%APPDATA%/aethermail/accounts.json`

#### Scenario: macOS configuration path
- **WHEN** running on macOS
- **THEN** system stores config at `~/Library/Application Support/aethermail/accounts.json`

#### Scenario: Linux configuration path
- **WHEN** running on Linux
- **THEN** system stores config at `~/.config/aethermail/accounts.json`

### Requirement: Configuration file structure

The system SHALL use a defined JSON structure for account configuration.

#### Scenario: Valid JSON structure
- **WHEN** reading or writing accounts.json
- **THEN** system uses the following structure:
```json
{
  "accounts": [
    {
      "id": "unique-id",
      "email": "user@example.com",
      "display_name": "User Name",
      "password": "plaintext-password",
      "protocol": "pop3",
      "pop3_host": "pop.example.com",
      "pop3_port": 995,
      "use_ssl": true,
      "imap_host": "imap.example.com",
      "imap_port": 993,
      "imap_use_ssl": true,
      "smtp_host": "smtp.example.com",
      "smtp_port": 465,
      "smtp_use_ssl": true,
      "is_active": true,
      "phishing_detection_enabled": false
    }
  ]
}
```

#### Scenario: Handle corrupted config
- **WHEN** accounts.json is corrupted or invalid JSON
- **THEN** system creates a new empty configuration file

#### Scenario: Handle missing phishing field
- **WHEN** accounts.json exists but lacks phishing_detection_enabled field
- **THEN** system treats missing field as false (disabled)

#### Scenario: Handle missing SMTP fields
- **WHEN** accounts.json exists but lacks SMTP fields
- **THEN** system treats missing smtp_host as empty string
- **AND** system treats missing smtp_port as 465 (default)
- **AND** system treats missing smtp_use_ssl as true (default)

### Requirement: SMTP configuration storage

The system SHALL persist SMTP server configuration for each account.

#### Scenario: Save SMTP settings
- **WHEN** user configures SMTP settings for an account
- **THEN** system saves smtp_host, smtp_port, smtp_use_ssl to account configuration

#### Scenario: Retrieve SMTP for sending
- **WHEN** system needs to send an email
- **THEN** system retrieves SMTP configuration from account settings

### Requirement: SMTP server configuration per account

The system SHALL support unique SMTP configuration for each account.

#### Scenario: Different SMTP per account
- **WHEN** multiple accounts exist
- **THEN** each account can have different SMTP server settings
- **AND** sending email uses active account's SMTP configuration

#### Scenario: SMTP config optional
- **WHEN** account is created without SMTP configuration
- **THEN** SMTP fields are stored with default values
- **AND** compose/send functionality shows warning about missing SMTP config

### Requirement: Phishing detection setting per account

The system SHALL support per-account phishing detection enable/disable setting.

#### Scenario: Store phishing detection preference
- **WHEN** user enables or disables phishing detection for an account
- **THEN** system stores `phishing_detection_enabled` boolean in account configuration

#### Scenario: Default phishing detection state
- **WHEN** new account is created
- **THEN** system sets `phishing_detection_enabled` to false by default

#### Scenario: Load phishing detection preference
- **WHEN** application loads account configuration
- **THEN** system reads `phishing_detection_enabled` setting
- **AND** system uses setting to determine if phishing detection should run