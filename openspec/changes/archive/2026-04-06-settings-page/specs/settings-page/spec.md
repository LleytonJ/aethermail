## ADDED Requirements

### Requirement: Settings page routing

The system SHALL provide a settings page accessible via `/settings` route.

#### Scenario: Navigate to settings page
- **WHEN** user clicks settings button in sidebar
- **THEN** system navigates to `/settings/accounts` route

#### Scenario: Direct access to settings
- **WHEN** user navigates directly to `/settings`
- **THEN** system redirects to `/settings/accounts`

#### Scenario: Settings page layout
- **WHEN** user is on settings page
- **THEN** system displays two-column layout with navigation on left and content on right

### Requirement: Settings navigation menu

The system SHALL provide a navigation menu in the settings page for switching between setting categories.

#### Scenario: Display navigation options
- **WHEN** user is on settings page
- **THEN** system displays navigation items including "账户管理" and reserved "通用设置"

#### Scenario: Active navigation item
- **WHEN** user is on `/settings/accounts`
- **THEN** system highlights "账户管理" navigation item

#### Scenario: Switch settings category
- **WHEN** user clicks navigation item
- **THEN** system updates content area to show corresponding settings

### Requirement: Settings page entry point

The system SHALL provide an entry point to the settings page from the sidebar.

#### Scenario: Settings button in sidebar
- **WHEN** user views the sidebar
- **THEN** system displays a settings button

#### Scenario: Settings button click
- **WHEN** user clicks the settings button
- **THEN** system navigates to `/settings/accounts`