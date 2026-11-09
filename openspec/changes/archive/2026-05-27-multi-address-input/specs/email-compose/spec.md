## MODIFIED Requirements

### Requirement: Accept recipient addresses as structured objects

The system SHALL accept recipient addresses (To, CC) as an array of `EmailAddress` objects instead of plain strings.

#### Scenario: Initialize compose form with structured addresses
- **WHEN** compose form is initialized
- **THEN** form.to and form.cc are arrays of `EmailAddress` objects

#### Scenario: Bind AddressInput to structured data
- **WHEN** AddressInput component emits update:modelValue
- **THEN** parent component receives `EmailAddress[]` type data

### Requirement: Format addresses for email sending

The system SHALL format `EmailAddress` objects into SMTP-compatible string format before sending.

#### Scenario: Format address with name for SMTP
- **WHEN** sending email with `EmailAddress { email: "user@example.com", name: "张三" }`
- **THEN** system sends `"张三" <user@example.com>` to SMTP server

#### Scenario: Format address without name for SMTP
- **WHEN** sending email with `EmailAddress { email: "user@example.com" }`
- **THEN** system sends `user@example.com` to SMTP server

### Requirement: Maintain draft compatibility

The system SHALL maintain backward compatibility with existing draft data that uses `string[]` format.

#### Scenario: Load legacy draft data
- **WHEN** loading a draft that was saved with `toAddresses: ["user1@example.com", "user2@example.com"]`
- **THEN** system converts to `EmailAddress[]` format with name field undefined

#### Scenario: Save draft with structured addresses
- **WHEN** saving a draft with `EmailAddress[]` format addresses
- **THEN** system converts to string array, using formatted string to preserve display name

## ADDED Requirements

### Requirement: Trigger parsing on all input termination points

The system SHALL trigger address parsing when user performs any of the following actions:
- Pastes content into the input field
- Types a comma or semicolon
- Presses Enter or Tab
- Loses focus on the input field

#### Scenario: Parse on paste
- **WHEN** user pastes `user1@example.com, user2@example.com` into the address field
- **THEN** system parses and displays two separate address tags

#### Scenario: Parse on comma input
- **WHEN** user types `user1@example.com,` in the address field
- **THEN** system parses and adds `user1@example.com` as an address tag

#### Scenario: Parse on Enter key
- **WHEN** user types `user1@example.com` and presses Enter
- **THEN** system parses and adds `user1@example.com` as an address tag

#### Scenario: Parse on blur
- **WHEN** user types `user1@example.com` and clicks outside the input field
- **THEN** system parses and adds `user1@example.com` as an address tag
