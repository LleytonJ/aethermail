## ADDED Requirements

### Requirement: HTML text extraction for email content

The system SHALL provide HTML text extraction capability that extracts clean text from HTML email content, removing CSS styles and JavaScript code.

#### Scenario: Extract clean text from HTML with CSS
- **WHEN** email contains only text/html part with embedded CSS in `<style>` tags
- **THEN** system uses DOM parser to extract text
- **AND** extracted text excludes all CSS content from `<style>` tags
- **AND** extracted text preserves visible content structure

#### Scenario: Extract clean text from HTML with JavaScript
- **WHEN** email contains only text/html part with embedded JavaScript in `<script>` tags
- **THEN** system uses DOM parser to extract text
- **AND** extracted text excludes all JavaScript content from `<script>` tags

#### Scenario: Prefer existing text/plain content
- **WHEN** email contains both text/plain and text/html parts
- **THEN** system uses mail-parser's body_text() directly
- **AND** system does not process HTML content for text extraction

#### Scenario: Handle HTML-only email with valid body_text
- **WHEN** email contains only text/html but mail-parser provides valid body_text without CSS/JS
- **THEN** system uses mail-parser's body_text() directly
- **AND** system skips DOM parsing

#### Scenario: Handle empty email content
- **WHEN** email HTML body contains no visible text content (e.g., only images)
- **THEN** system returns empty string
- **AND** phishing detection proceeds with empty content

#### Scenario: CSS selector for content extraction
- **WHEN** system extracts text from HTML using DOM parser
- **THEN** system uses selector `body :not(style):not(script):not(head)`
- **AND** system collects text from all matching elements
- **AND** system joins text with appropriate whitespace