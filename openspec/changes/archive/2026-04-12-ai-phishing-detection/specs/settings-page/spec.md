## ADDED Requirements

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