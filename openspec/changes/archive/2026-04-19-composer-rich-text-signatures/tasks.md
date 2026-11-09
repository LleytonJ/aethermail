## 1. Dependencies Setup

- [x] 1.1 Install TipTap Vue 3 packages: `@tiptap/vue-3`, `@tiptap/starter-kit`, `@tiptap/extension-image`, `@tiptap/extension-placeholder`
- [x] 1.2 Verify TipTap installation and create minimal test component

## 2. Database Schema

- [x] 2.1 Add `signature_html` column to accounts table (TEXT type, default empty string)

## 3. Signature Service

- [x] 3.1 Create `signatureService.ts` with signature read/write operations
- [x] 3.2 Implement `getSignature(accountId)` function - returns signature HTML for account
- [x] 3.3 Implement `updateSignature(accountId, signatureHtml)` function
- [x] 3.4 Integrate signature save into existing account save flow

## 4. Rich Text Editor Component

- [x] 4.1 Create `RichTextEditor.vue` component with TipTap integration
- [x] 4.2 Implement editor toolbar (bold, italic, lists, link, image buttons)
- [x] 4.3 Add placeholder extension support
- [x] 4.4 Add image extension support (inline, base64)
- [x] 4.5 Implement content sync with parent component (v-model pattern)
- [x] 4.6 Add minHeight and editable props support
- [ ] 4.7 Add unit tests for RichTextEditor component

## 5. Account Edit Form - Signature Section

- [x] 5.1 Add signature section to `EditAccountForm.vue` below server settings
- [x] 5.2 Add signature enable/disable toggle
- [x] 5.3 Integrate RichTextEditor in signature section
- [x] 5.4 Load existing signature_html when editing account
- [x] 5.5 Save signature_html along with account on form submit
- [x] 5.6 Disable editor when toggle is off
- [ ] 5.7 Add unit tests for signature section in EditAccountForm

## 6. Composer Integration

- [x] 6.1 Replace textarea in ComposeEmail.vue with RichTextEditor
- [x] 6.2 Add signature toggle switch to composer toolbar
- [x] 6.3 Load account signature on composer open
- [x] 6.4 Hide toggle if account has no signature
- [x] 6.5 Default toggle to enabled when signature exists
- [x] 6.6 Update email body validation for HTML content
- [x] 6.7 Add includeSignature state to composeStore
- [ ] 6.8 Add signature preview display below email body (optional)
- [ ] 6.9 Update reply/forward to use rich text editor

## 7. Email Service Update

- [x] 7.1 Update `sendEmail()` to support HTML body parameter
- [x] 7.2 Implement HTML MIME email construction in SMTP sending
- [x] 7.3 Append signature to HTML body when toggle is enabled
- [x] 7.4 Add separator div between body and signature
- [ ] 7.5 Add signature HTML sanitization before sending
- [ ] 7.6 Add unit tests for HTML email sending with signature

## 8. Integration Testing

- [ ] 8.1 Test full flow: set signature in account → compose → send with signature
- [ ] 8.2 Test signature toggle on/off behavior
- [ ] 8.3 Test compose without signature (empty signature_html)
- [ ] 8.4 Test reply/forward with signature
- [ ] 8.5 Test signature isolation between accounts
- [ ] 8.6 Verify signature displays correctly in sent email (check via email client)

## 9. Polish

- [ ] 9.1 Add error handling for signature operations
- [ ] 9.2 Add loading states for signature operations
- [ ] 9.3 Ensure proper HTML sanitization (DOMPurify or similar)