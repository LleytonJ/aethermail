## ADDED Requirements

### Requirement: Clean text input for phishing detection

The system SHALL ensure phishing detection receives clean text content without CSS/JS interference.

#### Scenario: Detect with clean text input
- **WHEN** phishing detection is triggered for an email
- **THEN** system provides extracted clean text to AI
- **AND** clean text excludes CSS styles and JavaScript code
- **AND** AI receives only meaningful email content for analysis

#### Scenario: Text extraction before detection
- **WHEN** email is parsed for phishing detection
- **THEN** system applies HTML text extraction capability
- **AND** extracted text is used as input for AI prompt