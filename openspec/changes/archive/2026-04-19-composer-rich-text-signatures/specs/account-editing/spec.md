## ADDED Requirements

### Requirement: Signature section in account edit form

The system SHALL display signature editing section in account edit form.

#### Scenario: Signature section placement
- **WHEN** account edit form is displayed
- **THEN** system shows signature section after server settings
- **AND** signature section is visually distinct from other sections

#### Scenario: Signature enable toggle
- **WHEN** signature section is displayed
- **THEN** system shows enable/disable toggle
- **AND** toggle default state reflects whether signature_html has content

#### Scenario: Signature editor integration
- **WHEN** signature section is enabled
- **THEN** system displays rich text editor for signature content
- **AND** editor loads existing signature_html if present