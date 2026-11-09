## ADDED Requirements

### Requirement: Threads table schema

The system SHALL create threads table for storing thread metadata.

#### Scenario: Database migration adds threads table
- **WHEN** application starts with new version
- **THEN** system creates threads table with columns: id, account_id, subject_base, last_message_at, message_count, has_unread
- **AND** system creates primary key on id
- **AND** system creates index on (account_id, last_message_at DESC)

### Requirement: Emails table threading columns

The system SHALL add threading-related columns to emails table.

#### Scenario: Add thread_id column
- **WHEN** database migration runs
- **THEN** system adds thread_id column (TEXT, nullable)
- **AND** system creates index on thread_id

#### Scenario: Add threading header columns
- **WHEN** database migration runs
- **THEN** system adds message_id_header column (TEXT, nullable)
- **AND** system adds in_reply_to_header column (TEXT, nullable)
- **AND** system adds references_header column (TEXT, nullable)

### Requirement: Thread upsert operation

The system SHALL provide method to create or update thread records.

#### Scenario: Upsert new thread
- **WHEN** frontend computes new thread group
- **THEN** system inserts new record into threads table
- **AND** system sets id from computed thread ID

#### Scenario: Upsert existing thread
- **WHEN** frontend updates existing thread (new message added)
- **THEN** system updates message_count and last_message_at
- **AND** system updates has_unread based on email read status

### Requirement: Email thread assignment

The system SHALL provide method to assign thread_id to emails.

#### Scenario: Batch assign thread_id
- **WHEN** frontend calls assign_thread_id method
- **THEN** system updates emails.thread_id for specified email IDs
- **AND** system uses single batch UPDATE for efficiency

### Requirement: Thread retrieval methods

The system SHALL provide methods to query threads and thread messages.

#### Scenario: Get threads by account
- **WHEN** frontend calls get_threads method with account_id
- **THEN** system returns threads ordered by last_message_at DESC
- **AND** each thread includes computed has_unread flag

#### Scenario: Get emails by thread
- **WHEN** frontend calls get_thread_emails method with thread_id
- **THEN** system returns emails ordered by date ASC
- **AND** system includes full email details (body, attachments)