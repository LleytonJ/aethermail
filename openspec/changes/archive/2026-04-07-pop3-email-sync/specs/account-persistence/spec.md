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
      "pop3_host": "pop.example.com",
      "pop3_port": 995,
      "use_ssl": true,
      "is_active": true
    }
  ]
}
```

#### Scenario: Handle corrupted config
- **WHEN** accounts.json is corrupted or invalid JSON
- **THEN** system creates a new empty configuration file