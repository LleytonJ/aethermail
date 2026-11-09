## ADDED Requirements

### Requirement: Display address with format awareness

The system SHALL display email addresses in a format-aware manner, showing the complete format when a display name is present.

#### Scenario: Display address with name
- **WHEN** an address has both email and name properties
- **THEN** system displays `"Display Name" <email@example.com>`

#### Scenario: Display address without name
- **WHEN** an address has only email property
- **THEN** system displays `email@example.com`

### Requirement: Support address tag removal

The system SHALL allow users to remove individual addresses from the address list by clicking a remove button on each address tag.

#### Scenario: Remove single address
- **WHEN** user clicks the remove button (×) on an address tag
- **THEN** system removes that address from the list and updates the model value

#### Scenario: Remove address via backspace
- **WHEN** input field is empty and user presses Backspace
- **THEN** system removes the last address in the list

### Requirement: Provide visual feedback for invalid input

The system SHALL display clear error messages when address parsing or validation fails.

#### Scenario: Show validation error
- **WHEN** parsed addresses contain invalid entries
- **THEN** system displays an error message below the input field listing the invalid addresses

#### Scenario: Clear error on successful input
- **WHEN** user successfully adds valid addresses after an error
- **THEN** system clears the error message
