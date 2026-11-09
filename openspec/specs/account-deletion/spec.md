### Requirement: Account deletion capability

The system SHALL allow users to delete accounts from the settings page.

#### Scenario: Delete button availability
- **WHEN** user views account management page
- **THEN** system displays delete button for each account

#### Scenario: Delete button disabled for last account
- **WHEN** there is only one account
- **THEN** system disables delete button with tooltip hint

### Requirement: Delete confirmation dialog

The system SHALL display a confirmation dialog before deleting an account.

#### Scenario: Confirmation dialog trigger
- **WHEN** user clicks delete button
- **THEN** system displays confirmation dialog

#### Scenario: Confirmation dialog content
- **WHEN** confirmation dialog displays
- **THEN** system shows account email to be deleted
- **AND** system shows warning message about deletion consequences

#### Scenario: Confirm deletion
- **WHEN** user confirms deletion
- **THEN** system removes account from storage
- **AND** system displays success notification
- **AND** system updates account list

#### Scenario: Cancel deletion
- **WHEN** user cancels deletion
- **THEN** system closes confirmation dialog
- **AND** account remains unchanged

### Requirement: Account deletion handling

The system SHALL handle account deletion consequences appropriately.

#### Scenario: Delete active account
- **WHEN** user deletes the currently active account
- **THEN** system switches to another account if available
- **AND** system updates active account indicator

#### Scenario: Delete non-active account
- **WHEN** user deletes a non-active account
- **THEN** system removes account without affecting active account

#### Scenario: Storage cleanup on deletion
- **WHEN** account is deleted
- **THEN** system removes account data from localStorage