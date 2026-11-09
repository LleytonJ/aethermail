## 1. Dependencies Setup

- [x] 1.1 Add scraper crate to Cargo.toml
- [x] 1.2 Create src-tauri/src/utils/mod.rs module structure
- [x] 1.3 Create src-tauri/src/utils/html_extractor.rs file

## 2. HTML Text Extraction Implementation

- [x] 2.1 Implement HtmlExtractor struct with scraper dependency
- [x] 2.2 Implement extract_clean_text() function using CSS selector
- [x] 2.3 Implement is_text_clean() helper to detect CSS/JS patterns
- [x] 2.4 Add error handling for malformed HTML

## 3. Email Parsing Integration

- [x] 3.1 Modify parse_email() in commands/email.rs to use HtmlExtractor
- [x] 3.2 Implement fallback logic: body_text → clean check → scraper extraction
- [x] 3.3 Update detect_phishing_for_email() to use clean extracted text

## 4. Testing

- [ ] 4.1 Test with multipart/alternative email (has both text/plain and text/html)
- [ ] 4.2 Test with HTML-only email containing CSS in `<style>` tags
- [ ] 4.3 Test with HTML-only email containing JavaScript
- [ ] 4.4 Test with empty HTML body
- [ ] 4.5 Verify phishing detection receives clean text