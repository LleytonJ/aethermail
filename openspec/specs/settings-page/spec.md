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

### Requirement: Phishing detection settings section

The system SHALL provide a phishing detection settings section in the account management page.

#### Scenario: Display phishing detection toggle
- **WHEN** user views account settings
- **THEN** system displays "钓鱼邮件检测" toggle switch
- **AND** toggle shows current enabled/disabled state

#### Scenario: Enable phishing detection
- **WHEN** user toggles phishing detection to enabled
- **THEN** system checks if AI provider is configured
- **AND** if AI not configured, system shows warning message
- **AND** system saves enabled state to account configuration

#### Scenario: Disable phishing detection
- **WHEN** user toggles phishing detection to disabled
- **THEN** system saves disabled state to account configuration
- **AND** system does not require additional confirmation

#### Scenario: Show AI provider status
- **WHEN** user views phishing detection settings
- **THEN** system displays current AI provider status
- **AND** system shows link to AI provider settings if not configured

### Requirement: Rules file management in settings

The system SHALL provide phishing rules file management in settings.

#### Scenario: View rules file location
- **WHEN** user views phishing detection settings
- **THEN** system displays rules file path
- **AND** system provides button to open rules file in editor

#### Scenario: Reset rules to default
- **WHEN** user clicks "重置规则" button
- **THEN** system overwrites rules file with default template
- **AND** system shows confirmation before reset

#### Scenario: Show rules file error
- **WHEN** rules file cannot be read or parsed
- **THEN** system displays error indicator in settings
- **AND** system provides reset option