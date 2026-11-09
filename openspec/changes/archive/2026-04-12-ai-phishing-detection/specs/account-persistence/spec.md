## ADDED Requirements

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

## MODIFIED Requirements

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