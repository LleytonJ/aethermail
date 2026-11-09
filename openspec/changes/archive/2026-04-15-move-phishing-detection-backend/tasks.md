## 1. Dependencies and Setup

- [x] 1.1 Add async-openai dependency to Cargo.toml
- [x] 1.2 Create src-tauri/src/ai/mod.rs module structure
- [x] 1.3 Create src-tauri/src/ai/client.rs AI client file

## 2. Database Schema

- [x] 2.1 Add user_overridden column to emails table schema
- [x] 2.2 Update DbEmail struct to include user_overridden field
- [x] 2.3 Create database migration for existing databases

## 3. Backend AI Client

- [x] 3.1 Implement AI configuration loading (from ai-provider.json)
- [x] 3.2 Implement async-openai client initialization with custom base_url
- [x] 3.3 Implement phishing detection function with system prompt loading
- [x] 3.4 Implement response parsing (JSON extraction, is_phishing, reasons)
- [x] 3.5 Implement timeout handling (5 second limit)
- [x] 3.6 Implement error handling (network errors, API errors)
- [x] 3.7 Implement mock mode for development testing

## 4. Sync Flow Refactoring

- [x] 4.1 Modify sync_via_pop3 to call phishing detection before save
- [x] 4.2 Modify sync_via_imap to call phishing detection before save
- [x] 4.3 Update save logic to use detection result for is_phishing flag
- [x] 4.4 Add sync progress reporting for phishing detection phase (日志输出)
- [x] 4.5 Implement user_overridden check to skip detection for overridden emails

## 5. User Override Support

- [x] 5.1 Update updatePhishingStatus API to set user_overridden flag
- [x] 5.2 Update move-to-inbox logic to set user_overridden = true
- [x] 5.3 Update frontend ReadingPane handleMoveToInbox to pass user_overridden

## 6. Frontend Cleanup

- [x] 6.1 Remove OpenAI SDK import from aiService.ts (未使用 OpenAI SDK)
- [x] 6.2 Remove complete() function from AIService class (不存在此函数)
- [x] 6.3 Remove phishing detection logic from mailbox.ts sync function
- [x] 6.4 Remove phishingDetectedCount state (no longer needed)
- [x] 6.5 Simplify phishingDetectionService.ts to only config reading (保留用于配置管理)

## 7. Testing

- [ ] 7.1 Test OpenAI API compatibility
- [ ] 7.2 Test DeepSeek API compatibility
- [ ] 7.3 Test Bailian API compatibility
- [ ] 7.4 Test timeout scenario
- [ ] 7.5 Test error scenario
- [ ] 7.6 Test mock mode
- [ ] 7.7 Test user_overridden persistence
- [ ] 7.8 Verify phishing emails never appear in inbox