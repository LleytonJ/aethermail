## ADDED Requirements

### Requirement: IMAP threading header extraction

The system SHALL extract and store RFC 2822 threading headers for IMAP-synced emails.

#### Scenario: Extract Message-ID header
- **WHEN** IMAP email is parsed during sync
- **THEN** system extracts Message-ID header from raw email content
- **AND** system stores message_id_header field in emails table

#### Scenario: Extract In-Reply-To header
- **WHEN** IMAP email is parsed during sync
- **THEN** system extracts In-Reply-To header if present
- **AND** system stores in_reply_to_header field in emails table

#### Scenario: Extract References header
- **WHEN** IMAP email is parsed during sync
- **THEN** system extracts References header if present
- **AND** system stores references_header field in emails table

### Requirement: Thread computation using JWZ algorithm

The system SHALL compute email threads using JWZ threading algorithm in frontend.

#### Scenario: Build threads from emails
- **WHEN** frontend receives emails with threading headers
- **THEN** system runs JWZ algorithm to group emails into threads
- **AND** system generates deterministic thread ID from root Message-ID

#### Scenario: Handle phantom containers
- **WHEN** email references a Message-ID that does not exist locally
- **THEN** system creates phantom container for missing message
- **AND** thread structure remains consistent

### Requirement: Thread storage in database

The system SHALL store thread metadata in SQLite database.

#### Scenario: Create thread record
- **WHEN** frontend computes a new thread
- **THEN** system creates record in threads table
- **AND** system stores thread ID, account ID, subject base, message count

#### Scenario: Assign thread_id to emails
- **WHEN** thread is created or updated
- **THEN** system updates emails.thread_id for all emails in thread
- **AND** system updates threads.last_message_at and message_count

### Requirement: Thread retrieval

The system SHALL provide methods to retrieve threads and thread messages.

#### Scenario: Get threads for account
- **WHEN** frontend requests threads for an account
- **THEN** system returns all threads sorted by last_message_at descending
- **AND** each thread includes subject, snippet, message_count, has_unread

#### Scenario: Get messages in thread
- **WHEN** frontend requests messages for a specific thread
- **THEN** system returns all emails with matching thread_id
- **AND** system sorts by date ascending (chronological order)

### Requirement: Thread-based email list UI

The system SHALL display emails grouped as threads in the UI.

#### Scenario: Display thread card
- **WHEN** thread has multiple messages
- **THEN** UI displays thread card with subject, sender, message count badge
- **AND** UI displays latest message snippet and date

#### Scenario: Expand thread to view messages
- **WHEN** user clicks on thread card
- **THEN** UI expands to show all messages in thread chronologically
- **AND** each message shows sender, date, and body preview

#### Scenario: Single message thread
- **WHEN** thread has only one message
- **THEN** UI displays as regular email item without thread badge