## ADDED Requirements

### Requirement: Threading header storage during IMAP sync

The system SHALL store RFC 2822 threading headers when syncing IMAP emails.

#### Scenario: Store Message-ID from parsed email
- **WHEN** IMAP email is parsed by mail-parser during sync
- **THEN** system calls message.message_id() to extract Message-ID
- **AND** system stores value in emails.message_id_header column

#### Scenario: Store In-Reply-To from parsed email
- **WHEN** IMAP email contains In-Reply-To header
- **THEN** system calls message.in_reply_to() to extract header value
- **AND** system stores value in emails.in_reply_to_header column

#### Scenario: Store References from parsed email
- **WHEN** IMAP email contains References header
- **THEN** system calls message.references() to extract header value
- **AND** system stores value in emails.references_header column

#### Scenario: Handle missing threading headers
- **WHEN** email has no Message-ID header
- **THEN** system generates synthetic Message-ID from account_id and uid
- **AND** system stores synthetic ID in message_id_header column