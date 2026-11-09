## MODIFIED Requirements

### Requirement: Configuration file structure

The system SHALL use a defined JSON structure for account configuration.

#### Scenario: Valid JSON structure with SMTP
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

#### Scenario: Handle missing SMTP fields
- **WHEN** accounts.json exists but lacks SMTP fields
- **THEN** system treats missing smtp_host as empty string
- **AND** system treats missing smtp_port as 465 (default)
- **AND** system treats missing smtp_use_ssl as true (default)

---

## ADDED Requirements

### Requirement: SMTP configuration storage

The system SHALL persist SMTP server configuration for each account.

#### Scenario: Save SMTP settings
- **WHEN** user configures SMTP settings for an account
- **THEN** system saves smtp_host, smtp_port, smtp_use_ssl to account configuration

#### Scenario: Retrieve SMTP for sending
- **WHEN** system needs to send an email
- **THEN** system retrieves SMTP configuration from account settings

---

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