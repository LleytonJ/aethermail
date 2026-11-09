## 1. Rust Backend - Data Model & Migration

- [x] 1.1 Add threading header fields to DbEmail model (message_id_header, in_reply_to_header, references_header, thread_id)
- [x] 1.2 Create DbThread model struct (id, account_id, subject_base, last_message_at, message_count, has_unread)
- [x] 1.3 Add threads table migration in database.rs init_tables()
- [x] 1.4 Add threading header columns migration to emails table
- [x] 1.5 Create indexes on threads(account_id, last_message_at) and emails(thread_id)

## 2. Rust Backend - Threading Header Parsing

- [x] 2.1 Add message_id, in_reply_to, references fields to ImapMessage struct in imap/types.rs
- [x] 2.2 Update imap/client.rs fetch_email to parse threading headers from raw content using mail-parser
- [x] 2.3 Handle missing Message-ID by generating synthetic ID (imap-{account_id}-{uid})
- [x] 2.4 Handle In-Reply-To header parsing (mail-parser HeaderValue::Text / TextList)
- [x] 2.5 Handle References header parsing (space-separated Message-IDs)

## 3. Rust Backend - Sync Flow & Storage

- [x] 3.1 Update sync_via_imap to store threading headers in DbEmail
- [x] 3.2 Add upsert_thread method to database.rs
- [x] 3.3 Add assign_thread_ids batch update method to database.rs
- [x] 3.4 Add get_threads_for_account method to database.rs
- [x] 3.5 Add get_thread_emails method to database.rs

## 4. Rust Backend - Tauri Commands

- [x] 4.1 Add get_threads Tauri command returning Thread[] for frontend
- [x] 4.2 Add get_thread_emails Tauri command returning Email[] for thread
- [x] 4.3 Add assign_threads Tauri command for frontend to write computed thread_ids
- [x] 4.4 Register new commands in lib.rs invoke_handler

## 5. Frontend - JWZ Threading Algorithm

- [x] 5.1 Create services/threading/threadBuilder.ts (copy from velo project)
- [x] 5.2 Create ThreadableMessage interface matching threadBuilder input
- [x] 5.3 Create ThreadGroup interface matching threadBuilder output
- [x] 5.4 Implement normalizeSubject function (strip Re:/Fwd:/[list] prefixes)
- [x] 5.5 Implement buildThreads function (full JWZ algorithm)
- [ ] 5.6 Add unit tests for threadBuilder

## 6. Frontend - Thread Service & Store

- [x] 6.1 Create services/threading/threadService.ts with API calls
- [x] 6.2 Add threads state to mailbox store (threads array, threadMap)
- [x] 6.3 Add loadThreads action to mailbox store
- [x] 6.4 Add selectThread action to mailbox store
- [x] 6.5 Add threading computation hook (compute threads after email sync)

## 7. Frontend - Thread UI Components

- [x] 7.1 Create ThreadListItem.vue component (shows thread card with message count)
- [x] 7.2 Create ThreadView.vue component (expanded view with all messages)
- [x] 7.3 Update EmailList to conditionally render threads or single emails
- [x] 7.4 Add thread selection and expansion interaction
- [x] 7.5 Style thread badge (message count indicator)

## 8. Integration & Testing

- [ ] 8.1 Test IMAP sync with threading headers extraction
- [ ] 8.2 Test thread computation with reply chain emails
- [ ] 8.3 Test thread storage and retrieval
- [ ] 8.4 Test UI rendering of threads vs single emails
- [ ] 8.5 Test phantom container handling (missing referenced emails)