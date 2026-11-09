## ADDED Requirements

### Requirement: Backend AI client

The system SHALL provide a Rust-based AI client using async-openai crate for phishing detection.

#### Scenario: Initialize AI client
- **WHEN** phishing detection is triggered
- **THEN** system reads AI configuration from ai-provider.json
- **AND** system creates async-openai client with API key and base URL
- **AND** system uses configured model name

#### Scenario: Custom base URL support
- **WHEN** AI provider configuration has custom base_url
- **THEN** system uses custom base_url for API calls
- **AND** system supports OpenAI, DeepSeek, Bailian, and other OpenAI-compatible APIs

#### Scenario: API call with system prompt
- **WHEN** phishing detection is called
- **THEN** system loads phishing-prompt.md content as system prompt
- **AND** system constructs user message with email content
- **AND** system sends chat completion request

#### Scenario: Response parsing
- **WHEN** AI returns chat completion response
- **THEN** system extracts content from response
- **AND** system parses JSON to get is_phishing and reasons

### Requirement: Atomic detection and storage

The system SHALL perform phishing detection before saving email to database.

#### Scenario: Detect before save
- **WHEN** new email is downloaded from server
- **AND** account has phishing detection enabled
- **THEN** system calls AI detection
- **AND** system waits for detection result (with timeout)
- **AND** system saves email with correct is_phishing value
- **AND** fishing emails are never saved to inbox folder

#### Scenario: Skip detection when disabled
- **WHEN** new email is downloaded
- **AND** account has phishing detection disabled
- **THEN** system skips AI detection call
- **AND** system saves email with is_phishing = false

#### Scenario: Detection timeout
- **WHEN** AI detection exceeds 5 second timeout
- **THEN** system saves email with is_phishing = false
- **AND** system logs timeout warning
- **AND** sync continues without blocking

#### Scenario: Detection error
- **WHEN** AI detection call fails with network or API error
- **THEN** system saves email with is_phishing = false
- **AND** system logs error details
- **AND** sync continues without blocking

### Requirement: Mock mode for testing

The system SHALL support mock mode for development and testing.

#### Scenario: Enable mock mode
- **WHEN** environment variable or configuration enables mock mode
- **THEN** system returns pre-defined mock response
- **AND** system does not call real AI API
- **AND** mock response simulates phishing detection result