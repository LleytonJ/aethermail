## ADDED Requirements

### Requirement: SMTP server configuration

The system SHALL support SMTP server configuration for email sending.

#### Scenario: SMTP config structure
- **WHEN** user configures SMTP settings
- **THEN** system accepts host, port, and SSL settings
- **AND** default values are: port 465, SSL enabled

#### Scenario: SMTP port options
- **WHEN** user selects port
- **THEN** system supports standard SMTP ports (25, 465, 587)
- **AND** port 465 is recommended for SSL connections

---

### Requirement: SMTP connection test

The system SHALL provide SMTP connection testing before account creation.

#### Scenario: Test SMTP connection
- **WHEN** user clicks "Test SMTP Connection"
- **THEN** system attempts to connect to SMTP server
- **AND** performs authentication with provided credentials
- **AND** displays success or failure message

#### Scenario: SMTP connection success
- **WHEN** SMTP connection and authentication succeed
- **THEN** system displays "SMTP connection successful" message
- **AND** enables "Add Account" button

#### Scenario: SMTP connection failure
- **WHEN** SMTP connection fails
- **THEN** system displays specific error message
- **AND** error message indicates failure reason (timeout, auth failed, host unreachable)

#### Scenario: SMTP test timeout
- **WHEN** SMTP server does not respond within 30 seconds
- **THEN** system displays "Connection timeout" error

---

### Requirement: SMTP authentication

The system SHALL authenticate with SMTP server using provided credentials.

#### Scenario: SMTP authentication
- **WHEN** system connects to SMTP server
- **THEN** system authenticates using email address as username
- **AND** uses account password for authentication

#### Scenario: SMTP auth methods
- **WHEN** SMTP server requires authentication
- **THEN** system supports PLAIN and LOGIN authentication methods

---

### Requirement: SMTP SSL/TLS encryption

The system SHALL support encrypted SMTP connections.

#### Scenario: SSL connection
- **WHEN** smtp_use_ssl is true
- **THEN** system establishes TLS connection to SMTP server
- **AND** password is transmitted over encrypted channel

#### Scenario: STARTTLS support
- **WHEN** smtp_use_ssl is true and port is 587
- **THEN** system uses STARTTLS to upgrade connection to encrypted