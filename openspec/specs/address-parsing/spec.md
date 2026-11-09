## ADDED Requirements

### Requirement: Parse multiple email addresses from input

The system SHALL parse multiple email addresses from a single input string, supporting comma (`,`), semicolon (`;`), and newline (`\n`) as separators.

#### Scenario: Parse comma-separated addresses
- **WHEN** user inputs `user1@example.com, user2@example.com, user3@example.com`
- **THEN** system returns three separate addresses: `user1@example.com`, `user2@example.com`, `user3@example.com`

#### Scenario: Parse semicolon-separated addresses
- **WHEN** user inputs `user1@example.com; user2@example.com; user3@example.com`
- **THEN** system returns three separate addresses: `user1@example.com`, `user2@example.com`, `user3@example.com`

#### Scenario: Parse newline-separated addresses
- **WHEN** user pastes multiple lines of addresses
- **THEN** system returns each line as a separate address after trimming whitespace

#### Scenario: Parse mixed separators
- **WHEN** user inputs `user1@example.com, user2@example.com; user3@example.com`
- **THEN** system returns three separate addresses

### Requirement: Parse email with display name

The system SHALL parse email addresses with optional display names in the format `"Display Name" <email@example.com>` or `Display Name <email@example.com>`.

#### Scenario: Parse quoted display name format
- **WHEN** user inputs `"张三" <zhangsan@example.com>`
- **THEN** system returns `EmailAddress { email: "zhangsan@example.com", name: "张三" }`

#### Scenario: Parse unquoted display name format
- **WHEN** user inputs `张三 <zhangsan@example.com>`
- **THEN** system returns `EmailAddress { email: "zhangsan@example.com", name: "张三" }`

#### Scenario: Parse plain email without display name
- **WHEN** user inputs `zhangsan@example.com`
- **THEN** system returns `EmailAddress { email: "zhangsan@example.com" }` (no name field)

### Requirement: Validate parsed email addresses

The system SHALL validate each parsed email address using a standard email format regex.

#### Scenario: All addresses are valid
- **WHEN** user inputs `user1@example.com, user2@example.com`
- **THEN** system returns all addresses in the valid list, empty invalid list

#### Scenario: Some addresses are invalid
- **WHEN** user inputs `user1@example.com, invalid-address, user2@example.com`
- **THEN** system returns `user1@example.com` and `user2@example.com` in valid list, `invalid-address` in invalid list

#### Scenario: All addresses are invalid
- **WHEN** user inputs `invalid1, invalid2`
- **THEN** system returns empty valid list, both addresses in invalid list

### Requirement: Reject all addresses on validation failure

The system SHALL reject adding any addresses when the parsed result contains invalid addresses.

#### Scenario: User attempts to add addresses with some invalid
- **WHEN** parsed result contains both valid and invalid addresses
- **THEN** system does not add any addresses, displays error message listing invalid addresses, and preserves user input for correction

#### Scenario: Error message format
- **WHEN** invalid addresses are found
- **THEN** system displays error message in format `无效的地址: <address1>, <address2>`
