## ADDED Requirements

### Requirement: POP3 server connection

The system SHALL be able to establish TCP connection to POP3 servers.

#### Scenario: Connect to POP3 server with SSL
- **WHEN** user configures account with SSL enabled
- **THEN** system establishes TLS connection to the server on the specified port

#### Scenario: Connect to POP3 server without SSL
- **WHEN** user configures account without SSL
- **THEN** system establishes plain TCP connection to the server

#### Scenario: Connection timeout
- **WHEN** server is unreachable
- **THEN** system returns connection timeout error within 30 seconds

### Requirement: POP3 authentication

The system SHALL authenticate users with POP3 servers using username and password.

#### Scenario: Successful authentication
- **WHEN** user provides correct email and password
- **THEN** system authenticates successfully and returns session

#### Scenario: Failed authentication
- **WHEN** user provides incorrect credentials
- **THEN** system returns authentication error with descriptive message

### Requirement: Connection test

The system SHALL provide a way to test POP3 connection before saving account.

#### Scenario: Test connection success
- **WHEN** user clicks test connection button with valid configuration
- **THEN** system attempts connection and returns success result

#### Scenario: Test connection failure
- **WHEN** user clicks test connection button with invalid configuration
- **THEN** system returns error message explaining the failure reason