//! Database operations using SQLite

use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::path::PathBuf;

use super::models::{DbAccount, DbAttachment, DbContact, DbDraftAttachment, DbEmail, DbThread, DraftUpdates, EmailFolder};

/// Database manager
#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Initialize database with given path
    pub async fn new(db_path: &PathBuf) -> Result<Self, String> {
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

        let pool = SqlitePool::connect(&db_url)
            .await
            .map_err(|e| format!("连接数据库失败: {}", e))?;

        let db = Self { pool };
        db.init_tables().await?;

        Ok(db)
    }

    /// 获取数据库连接池（用于debug命令）
    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Create database tables
    async fn init_tables(&self) -> Result<(), String> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS emails (
                id TEXT PRIMARY KEY,
                account_id TEXT NOT NULL,
                uidl TEXT NOT NULL,
                folder TEXT NOT NULL DEFAULT 'inbox',
                draft_id TEXT,
                from_address TEXT,
                from_name TEXT,
                to_addresses TEXT,
                cc_addresses TEXT,
                subject TEXT,
                body_text TEXT,
                body_html TEXT,
                date INTEGER NOT NULL,
                is_read INTEGER NOT NULL DEFAULT 0,
                is_starred INTEGER NOT NULL DEFAULT 0,
                has_attachment INTEGER NOT NULL DEFAULT 0,
                is_phishing INTEGER NOT NULL DEFAULT 0,
                phishing_reasons TEXT,
                user_overridden INTEGER NOT NULL DEFAULT 0,
                UNIQUE(account_id, uidl)
            );

            CREATE TABLE IF NOT EXISTS attachments (
                id TEXT PRIMARY KEY,
                email_id TEXT NOT NULL,
                filename TEXT NOT NULL,
                mime_type TEXT NOT NULL,
                size INTEGER NOT NULL,
                content_id TEXT,
                imap_section TEXT,
                is_downloaded INTEGER NOT NULL DEFAULT 0,
                local_path TEXT,
                FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                display_name TEXT,
                avatar_url TEXT,
                frequency INTEGER NOT NULL DEFAULT 1,
                last_contacted_at INTEGER,
                created_at INTEGER DEFAULT (unixepoch()),
                updated_at INTEGER DEFAULT (unixepoch())
            );

            CREATE TABLE IF NOT EXISTS draft_attachments (
                id TEXT PRIMARY KEY,
                email_id TEXT NOT NULL,  -- 引用草稿邮件的 id（emails.id）
                filename TEXT NOT NULL,
                content_type TEXT,
                file_path TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (unixepoch()),
                FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE
            );

            -- Threads table for email conversation grouping
            CREATE TABLE IF NOT EXISTS threads (
                id TEXT PRIMARY KEY,
                account_id TEXT NOT NULL,
                subject_base TEXT,
                last_message_at INTEGER NOT NULL,
                message_count INTEGER NOT NULL DEFAULT 1,
                has_unread INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_emails_account ON emails(account_id);
            CREATE INDEX IF NOT EXISTS idx_emails_date ON emails(date);
            CREATE INDEX IF NOT EXISTS idx_emails_uidl ON emails(account_id, uidl);
            CREATE INDEX IF NOT EXISTS idx_contacts_email ON contacts(email);
            CREATE INDEX IF NOT EXISTS idx_contacts_frequency ON contacts(frequency DESC);
            CREATE INDEX IF NOT EXISTS idx_draft_attachments_email ON draft_attachments(email_id);
            CREATE INDEX IF NOT EXISTS idx_threads_account ON threads(account_id, last_message_at DESC);
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("创建表失败: {}", e))?;

        // 运行迁移：检查并添加新字段（兼容存量数据）
        self.run_migrations().await?;

        Ok(())
    }

    /// 运行数据库迁移，确保存量数据兼容
    async fn run_migrations(&self) -> Result<(), String> {
        // 检查 emails 表字段
        let email_columns: Vec<String> = sqlx::query("PRAGMA table_info(emails)")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("获取表结构失败: {}", e))?
            .into_iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();

        // 如果 is_phishing 字段不存在，添加它
        if !email_columns.contains(&"is_phishing".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN is_phishing INTEGER NOT NULL DEFAULT 0")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 is_phishing 字段失败: {}", e))?;

            // 创建钓鱼邮件索引（字段添加后）
            sqlx::query("CREATE INDEX IF NOT EXISTS idx_emails_phishing ON emails(is_phishing)")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("创建钓鱼索引失败: {}", e))?;
        }

        // 如果 phishing_reasons 字段不存在，添加它
        if !email_columns.contains(&"phishing_reasons".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN phishing_reasons TEXT")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 phishing_reasons 字段失败: {}", e))?;
        }

        // 如果 user_overridden 字段不存在，添加它
        if !email_columns.contains(&"user_overridden".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN user_overridden INTEGER NOT NULL DEFAULT 0")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 user_overridden 字段失败: {}", e))?;
        }

        // 如果是新数据库（字段已存在于 CREATE TABLE），确保索引存在
        if email_columns.contains(&"is_phishing".to_string()) {
            // 检查索引是否存在
            let indexes: Vec<String> = sqlx::query("SELECT name FROM sqlite_master WHERE type='index' AND tbl_name='emails'")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("获取索引列表失败: {}", e))?
                .into_iter()
                .map(|row| row.get::<String, _>("name"))
                .collect();

            if !indexes.contains(&"idx_emails_phishing".to_string()) {
                sqlx::query("CREATE INDEX idx_emails_phishing ON emails(is_phishing)")
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("创建钓鱼索引失败: {}", e))?;
            }
        }

        // 检查 attachments 表字段
        let att_columns: Vec<String> = sqlx::query("PRAGMA table_info(attachments)")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("获取 attachments 表结构失败: {}", e))?
            .into_iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();

        // 附件懒加载迁移：如果存在 content 列但不存在新字段，需要迁移
        if att_columns.contains(&"content".to_string()) {
            // 添加新字段（如果不存在）
            if !att_columns.contains(&"imap_section".to_string()) {
                sqlx::query("ALTER TABLE attachments ADD COLUMN imap_section TEXT")
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("添加 imap_section 字段失败: {}", e))?;
            }
            if !att_columns.contains(&"is_downloaded".to_string()) {
                sqlx::query("ALTER TABLE attachments ADD COLUMN is_downloaded INTEGER NOT NULL DEFAULT 0")
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("添加 is_downloaded 字段失败: {}", e))?;
            }
            if !att_columns.contains(&"local_path".to_string()) {
                sqlx::query("ALTER TABLE attachments ADD COLUMN local_path TEXT")
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("添加 local_path 字段失败: {}", e))?;
            }

            // 注意：不删除 content 列，SQLite 不支持 DROP COLUMN
            // 但新代码将不再使用 content 字段
            // 存量数据迁移将在 Task 1.3 中实现（需要 app_handle 获取缓存路径）
        }

        // 回收站功能迁移：添加 deleted_at 字段
        if !email_columns.contains(&"deleted_at".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN deleted_at INTEGER DEFAULT NULL")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 deleted_at 字段失败: {}", e))?;

            // 创建 deleted_at 索引（用于回收站查询）
            sqlx::query("CREATE INDEX IF NOT EXISTS idx_emails_deleted_at ON emails(deleted_at)")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("创建 deleted_at 索引失败: {}", e))?;
        }

        // 已发送/草稿功能迁移：添加 folder 字段
        if !email_columns.contains(&"folder".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN folder TEXT NOT NULL DEFAULT 'inbox'")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 folder 字段失败: {}", e))?;

            // 存量钓鱼邮件迁移到 phishing folder
            sqlx::query("UPDATE emails SET folder = 'phishing' WHERE is_phishing = 1")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("迁移钓鱼邮件 folder 失败: {}", e))?;
        }

        // 已发送/草稿功能迁移：添加 draft_id 字段
        if !email_columns.contains(&"draft_id".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN draft_id TEXT")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 draft_id 字段失败: {}", e))?;
        }

        // 确保 folder 和 draft_id 索引存在（新数据库或已迁移的数据库都需要）
        let indexes: Vec<String> = sqlx::query("SELECT name FROM sqlite_master WHERE type='index' AND tbl_name='emails'")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("获取索引列表失败: {}", e))?
            .into_iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();

        if !indexes.contains(&"idx_emails_folder".to_string()) {
            sqlx::query("CREATE INDEX idx_emails_folder ON emails(account_id, folder)")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("创建 folder 索引失败: {}", e))?;
        }

        if !indexes.contains(&"idx_emails_draft_id".to_string()) {
            sqlx::query("CREATE INDEX idx_emails_draft_id ON emails(draft_id)")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("创建 draft_id 索引失败: {}", e))?;
        }

        // Threading 功能迁移：添加 threading header 字段
        if !email_columns.contains(&"message_id_header".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN message_id_header TEXT")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 message_id_header 字段失败: {}", e))?;
        }
        if !email_columns.contains(&"in_reply_to_header".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN in_reply_to_header TEXT")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 in_reply_to_header 字段失败: {}", e))?;
        }
        if !email_columns.contains(&"references_header".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN references_header TEXT")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 references_header 字段失败: {}", e))?;
        }
        if !email_columns.contains(&"thread_id".to_string()) {
            sqlx::query("ALTER TABLE emails ADD COLUMN thread_id TEXT")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("添加 thread_id 字段失败: {}", e))?;
        }

        // Threading 功能迁移：创建 thread_id 索引
        if !indexes.contains(&"idx_emails_thread".to_string()) {
            sqlx::query("CREATE INDEX idx_emails_thread ON emails(thread_id)")
                .execute(&self.pool)
                .await
                .map_err(|e| format!("创建 thread_id 索引失败: {}", e))?;
        }

        // 联系人功能迁移：检查 contacts 表是否存在
        let tables: Vec<String> = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("获取表列表失败: {}", e))?
            .into_iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();

        if !tables.contains(&"contacts".to_string()) {
            sqlx::query(
                r#"
                CREATE TABLE contacts (
                    id TEXT PRIMARY KEY,
                    email TEXT NOT NULL UNIQUE,
                    display_name TEXT,
                    avatar_url TEXT,
                    frequency INTEGER NOT NULL DEFAULT 1,
                    last_contacted_at INTEGER,
                    created_at INTEGER DEFAULT (unixepoch()),
                    updated_at INTEGER DEFAULT (unixepoch())
                );
                CREATE INDEX idx_contacts_email ON contacts(email);
                CREATE INDEX idx_contacts_frequency ON contacts(frequency DESC);
                "#,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| format!("创建 contacts 表失败: {}", e))?;
        }

        // Threading 功能迁移：检查 threads 表是否存在
        if !tables.contains(&"threads".to_string()) {
            sqlx::query(
                r#"
                CREATE TABLE threads (
                    id TEXT PRIMARY KEY,
                    account_id TEXT NOT NULL,
                    subject_base TEXT,
                    last_message_at INTEGER NOT NULL,
                    message_count INTEGER NOT NULL DEFAULT 1,
                    has_unread INTEGER NOT NULL DEFAULT 0
                );
                CREATE INDEX idx_threads_account ON threads(account_id, last_message_at DESC);
                "#,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| format!("创建 threads 表失败: {}", e))?;
            log::info!("threads 表创建完成（迁移）");
        }

        // 草稿附件功能迁移：检查 draft_attachments 表是否存在
        if !tables.contains(&"draft_attachments".to_string()) {
            sqlx::query(
                r#"
                CREATE TABLE draft_attachments (
                    id TEXT PRIMARY KEY,
                    email_id TEXT NOT NULL,
                    filename TEXT NOT NULL,
                    content_type TEXT,
                    file_path TEXT,
                    content TEXT,
                    file_size INTEGER NOT NULL,
                    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
                    FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE
                );
                CREATE INDEX idx_draft_attachments_email ON draft_attachments(email_id);
                "#,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| format!("创建 draft_attachments 表失败: {}", e))?;
            log::info!("draft_attachments 表创建完成（包含 content 字段）");
        } else {
            // 检查 draft_attachments 表结构是否需要迁移
            // PRAGMA table_info 返回: cid, name, type, notnull, dflt_value, pk
            let draft_att_info: Vec<(String, i32)> = sqlx::query("PRAGMA table_info(draft_attachments)")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("获取 draft_attachments 表结构失败: {}", e))?
                .into_iter()
                .map(|row| {
                    let name: String = row.get("name");
                    let notnull: i32 = row.get("notnull");
                    (name, notnull)
                })
                .collect();

            let draft_att_columns: Vec<String> = draft_att_info.iter().map(|(name, _)| name.clone()).collect();
            log::info!("draft_attachments 表结构: {:?}", draft_att_info);

            // 检查 file_path 是否有 NOT NULL 约束 (notnull=1)
            let file_path_notnull = draft_att_info.iter()
                .find(|(name, _)| name == "file_path")
                .map(|(_, notnull)| *notnull)
                .unwrap_or(0);

            // 检查是否需要迁移：draft_id -> email_id 或 file_path 有 NOT NULL 约束
            let needs_rebuild = (draft_att_columns.contains(&"draft_id".to_string()) && !draft_att_columns.contains(&"email_id".to_string()))
                || file_path_notnull == 1;

            // 用于后续检查 content 字段
            let mut draft_att_columns_after = draft_att_columns.clone();

            if needs_rebuild {
                log::info!("重建 draft_attachments 表（直接删除旧表，不迁移数据）");

                // 临时禁用外键检查
                sqlx::query("PRAGMA foreign_keys = OFF")
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("禁用外键检查失败: {}", e))?;

                // 直接删除旧表和索引
                sqlx::query("DROP INDEX IF EXISTS idx_draft_attachments_email")
                    .execute(&self.pool)
                    .await
                    .ok(); // 忽略错误

                sqlx::query("DROP TABLE IF EXISTS draft_attachments")
                    .execute(&self.pool)
                    .await
                    .ok(); // 忽略错误

                // 删除可能残留的临时表
                sqlx::query("DROP TABLE IF EXISTS draft_attachments_new")
                    .execute(&self.pool)
                    .await
                    .ok();

                // 创建新表（正确的结构）
                sqlx::query(
                    r#"
                    CREATE TABLE draft_attachments (
                        id TEXT PRIMARY KEY,
                        email_id TEXT NOT NULL,
                        filename TEXT NOT NULL,
                        content_type TEXT,
                        file_path TEXT,
                        content TEXT,
                        file_size INTEGER NOT NULL,
                        created_at INTEGER NOT NULL DEFAULT (unixepoch()),
                        FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE
                    )
                    "#,
                )
                .execute(&self.pool)
                .await
                .map_err(|e| format!("创建 draft_attachments 表失败: {}", e))?;

                // 创建索引
                sqlx::query("CREATE INDEX idx_draft_attachments_email ON draft_attachments(email_id)")
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("创建索引失败: {}", e))?;

                // 重新启用外键检查
                sqlx::query("PRAGMA foreign_keys = ON")
                    .execute(&self.pool)
                    .await
                    .ok();

                log::info!("draft_attachments 表重建完成");

                // 更新列名列表
                draft_att_columns_after = vec!["id", "email_id", "filename", "content_type", "file_path", "content", "file_size", "created_at"]
                    .iter().map(|s| s.to_string()).collect();
            }

            // 添加 content 字段（用于直接存储 Base64 附件内容）
            if draft_att_columns_after.contains(&"email_id".to_string()) && !draft_att_columns_after.contains(&"content".to_string()) {
                log::info!("为 draft_attachments 表添加 content 字段");
                sqlx::query("ALTER TABLE draft_attachments ADD COLUMN content TEXT")
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("添加 content 字段失败: {}", e))?;
                log::info!("draft_attachments content 字段添加完成");
            }
        }

        Ok(())
    }

    /// Save an email to database
    pub async fn save_email(&self, email: &DbEmail) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO emails
            (id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
             subject, body_text, body_html, date, is_read, is_starred, has_attachment,
             is_phishing, phishing_reasons, user_overridden, deleted_at,
             message_id_header, in_reply_to_header, references_header, thread_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&email.id)
        .bind(&email.account_id)
        .bind(&email.uidl)
        .bind(&email.folder)
        .bind(&email.draft_id)
        .bind(&email.from_address)
        .bind(&email.from_name)
        .bind(&email.to_addresses)
        .bind(&email.cc_addresses)
        .bind(&email.subject)
        .bind(&email.body_text)
        .bind(&email.body_html)
        .bind(email.date)
        .bind(email.is_read as i32)
        .bind(email.is_starred as i32)
        .bind(email.has_attachment as i32)
        .bind(email.is_phishing as i32)
        .bind(&email.phishing_reasons)
        .bind(email.user_overridden as i32)
        .bind(email.deleted_at)
        .bind(&email.message_id_header)
        .bind(&email.in_reply_to_header)
        .bind(&email.references_header)
        .bind(&email.thread_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("保存邮件失败: {}", e))?;

        // 自动创建/更新发件人联系人（仅收件箱邮件）
        if email.folder == "inbox" {
            if let Some(from_addr) = &email.from_address {
                if !from_addr.is_empty() {
                    self.upsert_contact(from_addr, email.from_name.as_deref()).await?;
                }
            }
        }

        Ok(())
    }

    /// Get all emails for an account (inbox folder, excluding deleted emails)
    pub async fn get_emails(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE account_id = ? AND folder = 'inbox' AND deleted_at IS NULL
            ORDER BY date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取邮件列表失败: {}", e))?;

        let emails = rows
            .into_iter()
            .map(|row| DbEmail {
                id: row.get("id"),
                account_id: row.get("account_id"),
                uidl: row.get("uidl"),
                folder: row.get("folder"),
                draft_id: row.get("draft_id"),
                from_address: row.get("from_address"),
                from_name: row.get("from_name"),
                to_addresses: row.get("to_addresses"),
                cc_addresses: row.get("cc_addresses"),
                subject: row.get("subject"),
                body_text: row.get("body_text"),
                body_html: row.get("body_html"),
                date: row.get("date"),
                is_read: row.get::<i32, _>("is_read") == 1,
                is_starred: row.get::<i32, _>("is_starred") == 1,
                has_attachment: row.get::<i32, _>("has_attachment") == 1,
                is_phishing: row.get::<i32, _>("is_phishing") == 1,
                phishing_reasons: row.get("phishing_reasons"),
                user_overridden: row.get::<i32, _>("user_overridden") == 1,
                deleted_at: row.get("deleted_at"),
                message_id_header: row.get("message_id_header"),
                in_reply_to_header: row.get("in_reply_to_header"),
                references_header: row.get("references_header"),
                thread_id: row.get("thread_id"),
            })
            .collect();

        Ok(emails)
    }

    /// Get a single email by ID
    pub async fn get_email_by_id(&self, id: &str) -> Result<Option<DbEmail>, String> {
        let row = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("获取邮件失败: {}", e))?;

        Ok(row.map(|row| DbEmail {
            id: row.get("id"),
            account_id: row.get("account_id"),
            uidl: row.get("uidl"),
            folder: row.get("folder"),
            draft_id: row.get("draft_id"),
            from_address: row.get("from_address"),
            from_name: row.get("from_name"),
            to_addresses: row.get("to_addresses"),
            cc_addresses: row.get("cc_addresses"),
            subject: row.get("subject"),
            body_text: row.get("body_text"),
            body_html: row.get("body_html"),
            date: row.get("date"),
            is_read: row.get::<i32, _>("is_read") == 1,
            is_starred: row.get::<i32, _>("is_starred") == 1,
            has_attachment: row.get::<i32, _>("has_attachment") == 1,
            is_phishing: row.get::<i32, _>("is_phishing") == 1,
            phishing_reasons: row.get("phishing_reasons"),
            user_overridden: row.get::<i32, _>("user_overridden") == 1,
            deleted_at: row.get("deleted_at"),
            message_id_header: row.get("message_id_header"),
            in_reply_to_header: row.get("in_reply_to_header"),
            references_header: row.get("references_header"),
            thread_id: row.get("thread_id"),
        }))
    }

    /// Update email read status
    pub async fn mark_email_read(&self, id: &str, is_read: bool) -> Result<(), String> {
        sqlx::query("UPDATE emails SET is_read = ? WHERE id = ?")
            .bind(is_read as i32)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("更新已读状态失败: {}", e))?;

        Ok(())
    }

    /// Toggle email star status
    pub async fn toggle_email_star(&self, id: &str, is_starred: bool) -> Result<(), String> {
        sqlx::query("UPDATE emails SET is_starred = ? WHERE id = ?")
            .bind(is_starred as i32)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("更新星标状态失败: {}", e))?;

        Ok(())
    }

    /// Get existing UIDLs for an account
    pub async fn get_existing_uidls(&self, account_id: &str) -> Result<Vec<String>, String> {
        let rows = sqlx::query("SELECT uidl FROM emails WHERE account_id = ?")
            .bind(account_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("获取 UIDL 列表失败: {}", e))?;

        let uidls: Vec<String> = rows.into_iter().map(|row| row.get("uidl")).collect();

        Ok(uidls)
    }

    /// Delete all emails for an account
    pub async fn delete_emails_by_account(&self, account_id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM emails WHERE account_id = ?")
            .bind(account_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除账户邮件失败: {}", e))?;

        Ok(())
    }

    /// Save attachment
    pub async fn save_attachment(&self, attachment: &DbAttachment) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO attachments
            (id, email_id, filename, mime_type, size, content_id, imap_section, is_downloaded, local_path)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&attachment.id)
        .bind(&attachment.email_id)
        .bind(&attachment.filename)
        .bind(&attachment.mime_type)
        .bind(attachment.size)
        .bind(&attachment.content_id)
        .bind(&attachment.imap_section)
        .bind(attachment.is_downloaded as i32)
        .bind(&attachment.local_path)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("保存附件失败: {}", e))?;

        Ok(())
    }

    /// Get attachments for an email
    pub async fn get_attachments(&self, email_id: &str) -> Result<Vec<DbAttachment>, String> {
        let rows = sqlx::query(
            "SELECT id, email_id, filename, mime_type, size, content_id, imap_section, is_downloaded, local_path FROM attachments WHERE email_id = ?"
        )
        .bind(email_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取附件失败: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|row| DbAttachment {
                id: row.get("id"),
                email_id: row.get("email_id"),
                filename: row.get("filename"),
                mime_type: row.get("mime_type"),
                size: row.get("size"),
                content_id: row.get("content_id"),
                imap_section: row.get("imap_section"),
                is_downloaded: row.get::<i32, _>("is_downloaded") == 1,
                local_path: row.get("local_path"),
            })
            .collect())
    }

    /// Get attachment by ID
    pub async fn get_attachment_by_id(&self, id: &str) -> Result<Option<DbAttachment>, String> {
        let row = sqlx::query(
            "SELECT id, email_id, filename, mime_type, size, content_id, imap_section, is_downloaded, local_path FROM attachments WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("获取附件失败: {}", e))?;

        Ok(row.map(|row| DbAttachment {
            id: row.get("id"),
            email_id: row.get("email_id"),
            filename: row.get("filename"),
            mime_type: row.get("mime_type"),
            size: row.get("size"),
            content_id: row.get("content_id"),
            imap_section: row.get("imap_section"),
            is_downloaded: row.get::<i32, _>("is_downloaded") == 1,
            local_path: row.get("local_path"),
        }))
    }

    /// Update attachment downloaded status and local path
    pub async fn update_attachment_downloaded(&self, id: &str, local_path: &str) -> Result<(), String> {
        sqlx::query("UPDATE attachments SET is_downloaded = 1, local_path = ? WHERE id = ?")
            .bind(local_path)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("更新附件下载状态失败: {}", e))?;

        Ok(())
    }

    /// Mark all attachments as not downloaded (for cache clear)
    pub async fn mark_all_attachments_not_downloaded(&self) -> Result<(), String> {
        sqlx::query("UPDATE attachments SET is_downloaded = 0, local_path = NULL")
            .execute(&self.pool)
            .await
            .map_err(|e| format!("重置附件下载状态失败: {}", e))?;

        Ok(())
    }

    /// Get unread count for an account (inbox folder)
    pub async fn get_unread_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND folder = 'inbox' AND is_read = 0 AND deleted_at IS NULL")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取未读数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Update phishing status for an email
    /// Also updates the folder: 'inbox' for non-phishing, 'phishing' for phishing emails
    pub async fn update_phishing_status(
        &self,
        id: &str,
        is_phishing: bool,
        phishing_reasons: Option<String>,
        user_overridden: Option<bool>,
    ) -> Result<(), String> {
        // 根据 is_phishing 状态决定 folder
        let folder = if is_phishing { "phishing" } else { "inbox" };

        // 如果提供了 user_overridden，则更新它
        if let Some(overridden) = user_overridden {
            sqlx::query("UPDATE emails SET is_phishing = ?, phishing_reasons = ?, user_overridden = ?, folder = ? WHERE id = ?")
                .bind(is_phishing as i32)
                .bind(phishing_reasons)
                .bind(overridden as i32)
                .bind(folder)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新钓鱼状态失败: {}", e))?;
        } else {
            sqlx::query("UPDATE emails SET is_phishing = ?, phishing_reasons = ?, folder = ? WHERE id = ?")
                .bind(is_phishing as i32)
                .bind(phishing_reasons)
                .bind(folder)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新钓鱼状态失败: {}", e))?;
        }

        Ok(())
    }

    /// Get all phishing emails for an account (phishing folder)
    pub async fn get_phishing_emails(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE account_id = ? AND folder = 'phishing' AND deleted_at IS NULL
            ORDER BY date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取钓鱼邮件列表失败: {}", e))?;

        let emails = rows
            .into_iter()
            .map(|row| DbEmail {
                id: row.get("id"),
                account_id: row.get("account_id"),
                uidl: row.get("uidl"),
                folder: row.get("folder"),
                draft_id: row.get("draft_id"),
                from_address: row.get("from_address"),
                from_name: row.get("from_name"),
                to_addresses: row.get("to_addresses"),
                cc_addresses: row.get("cc_addresses"),
                subject: row.get("subject"),
                body_text: row.get("body_text"),
                body_html: row.get("body_html"),
                date: row.get("date"),
                is_read: row.get::<i32, _>("is_read") == 1,
                is_starred: row.get::<i32, _>("is_starred") == 1,
                has_attachment: row.get::<i32, _>("has_attachment") == 1,
                is_phishing: row.get::<i32, _>("is_phishing") == 1,
                phishing_reasons: row.get("phishing_reasons"),
                user_overridden: row.get::<i32, _>("user_overridden") == 1,
                deleted_at: row.get("deleted_at"),
                message_id_header: row.get("message_id_header"),
                in_reply_to_header: row.get("in_reply_to_header"),
                references_header: row.get("references_header"),
                thread_id: row.get("thread_id"),
            })
            .collect();

        Ok(emails)
    }

    /// Get phishing count for an account (phishing folder)
    pub async fn get_phishing_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND folder = 'phishing' AND deleted_at IS NULL")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取钓鱼邮件数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Get unread phishing count for an account (phishing folder)
    pub async fn get_phishing_unread_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND folder = 'phishing' AND is_read = 0 AND deleted_at IS NULL")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取钓鱼邮件未读数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Get all starred emails for an account (inbox and sent folders)
    pub async fn get_starred_emails(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE account_id = ? AND is_starred = 1 AND folder IN ('inbox', 'sent') AND deleted_at IS NULL
            ORDER BY date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取星标邮件列表失败: {}", e))?;

        let emails = rows
            .into_iter()
            .map(|row| DbEmail {
                id: row.get("id"),
                account_id: row.get("account_id"),
                uidl: row.get("uidl"),
                folder: row.get("folder"),
                draft_id: row.get("draft_id"),
                from_address: row.get("from_address"),
                from_name: row.get("from_name"),
                to_addresses: row.get("to_addresses"),
                cc_addresses: row.get("cc_addresses"),
                subject: row.get("subject"),
                body_text: row.get("body_text"),
                body_html: row.get("body_html"),
                date: row.get("date"),
                is_read: row.get::<i32, _>("is_read") == 1,
                is_starred: row.get::<i32, _>("is_starred") == 1,
                has_attachment: row.get::<i32, _>("has_attachment") == 1,
                is_phishing: row.get::<i32, _>("is_phishing") == 1,
                phishing_reasons: row.get("phishing_reasons"),
                user_overridden: row.get::<i32, _>("user_overridden") == 1,
                deleted_at: row.get("deleted_at"),
                message_id_header: row.get("message_id_header"),
                in_reply_to_header: row.get("in_reply_to_header"),
                references_header: row.get("references_header"),
                thread_id: row.get("thread_id"),
            })
            .collect();

        Ok(emails)
    }

    /// Get starred count for an account (inbox and sent folders)
    pub async fn get_starred_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND is_starred = 1 AND folder IN ('inbox', 'sent') AND deleted_at IS NULL")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取星标邮件数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Delete a single email by ID
    /// Attachments are automatically deleted via CASCADE
    pub async fn delete_email(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM emails WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除邮件失败: {}", e))?;

        Ok(())
    }

    // ==================== 回收站功能方法 ====================

    /// Move email to trash (soft delete)
    pub async fn move_to_trash(&self, id: &str) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query("UPDATE emails SET deleted_at = ? WHERE id = ?")
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("移动邮件到回收站失败: {}", e))?;

        Ok(())
    }

    /// Get trash emails for an account
    pub async fn get_trash_emails(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE account_id = ? AND deleted_at IS NOT NULL
            ORDER BY deleted_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取回收站邮件列表失败: {}", e))?;

        let emails = rows
            .into_iter()
            .map(|row| DbEmail {
                id: row.get("id"),
                account_id: row.get("account_id"),
                uidl: row.get("uidl"),
                folder: row.get("folder"),
                draft_id: row.get("draft_id"),
                from_address: row.get("from_address"),
                from_name: row.get("from_name"),
                to_addresses: row.get("to_addresses"),
                cc_addresses: row.get("cc_addresses"),
                subject: row.get("subject"),
                body_text: row.get("body_text"),
                body_html: row.get("body_html"),
                date: row.get("date"),
                is_read: row.get::<i32, _>("is_read") == 1,
                is_starred: row.get::<i32, _>("is_starred") == 1,
                has_attachment: row.get::<i32, _>("has_attachment") == 1,
                is_phishing: row.get::<i32, _>("is_phishing") == 1,
                phishing_reasons: row.get("phishing_reasons"),
                user_overridden: row.get::<i32, _>("user_overridden") == 1,
                deleted_at: row.get("deleted_at"),
                message_id_header: row.get("message_id_header"),
                in_reply_to_header: row.get("in_reply_to_header"),
                references_header: row.get("references_header"),
                thread_id: row.get("thread_id"),
            })
            .collect();

        Ok(emails)
    }

    /// Get trash count for an account
    pub async fn get_trash_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND deleted_at IS NOT NULL")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取回收站邮件数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Restore email from trash to inbox or phishing folder
    pub async fn restore_email(&self, id: &str, is_phishing: bool) -> Result<(), String> {
        // 根据 is_phishing 状态决定 folder
        let folder = if is_phishing { "phishing" } else { "inbox" };

        sqlx::query("UPDATE emails SET deleted_at = NULL, is_phishing = ?, folder = ? WHERE id = ?")
            .bind(is_phishing as i32)
            .bind(folder)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("恢复邮件失败: {}", e))?;

        Ok(())
    }

    /// Permanently delete email from trash
    pub async fn permanent_delete(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM emails WHERE id = ? AND deleted_at IS NOT NULL")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("永久删除邮件失败: {}", e))?;

        Ok(())
    }

    /// Cleanup emails deleted more than 30 days ago
    pub async fn cleanup_expired_trash(&self) -> Result<u64, String> {
        let threshold = chrono::Utc::now().timestamp() - (30 * 24 * 60 * 60); // 30 days ago
        let result = sqlx::query("DELETE FROM emails WHERE deleted_at IS NOT NULL AND deleted_at < ?")
            .bind(threshold)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("清理过期回收站邮件失败: {}", e))?;

        Ok(result.rows_affected())
    }

    // ==================== 已发送/草稿功能方法 ====================

    /// Get sent emails for an account (sent folder)
    pub async fn get_sent_emails(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE account_id = ? AND folder = 'sent' AND deleted_at IS NULL
            ORDER BY date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取已发送邮件列表失败: {}", e))?;

        let emails = rows
            .into_iter()
            .map(|row| DbEmail {
                id: row.get("id"),
                account_id: row.get("account_id"),
                uidl: row.get("uidl"),
                folder: row.get("folder"),
                draft_id: row.get("draft_id"),
                from_address: row.get("from_address"),
                from_name: row.get("from_name"),
                to_addresses: row.get("to_addresses"),
                cc_addresses: row.get("cc_addresses"),
                subject: row.get("subject"),
                body_text: row.get("body_text"),
                body_html: row.get("body_html"),
                date: row.get("date"),
                is_read: row.get::<i32, _>("is_read") == 1,
                is_starred: row.get::<i32, _>("is_starred") == 1,
                has_attachment: row.get::<i32, _>("has_attachment") == 1,
                is_phishing: row.get::<i32, _>("is_phishing") == 1,
                phishing_reasons: row.get("phishing_reasons"),
                user_overridden: row.get::<i32, _>("user_overridden") == 1,
                deleted_at: row.get("deleted_at"),
                message_id_header: row.get("message_id_header"),
                in_reply_to_header: row.get("in_reply_to_header"),
                references_header: row.get("references_header"),
                thread_id: row.get("thread_id"),
            })
            .collect();

        Ok(emails)
    }

    /// Get draft emails for an account (drafts folder)
    pub async fn get_draft_emails(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE account_id = ? AND folder = 'drafts' AND deleted_at IS NULL
            ORDER BY date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取草稿邮件列表失败: {}", e))?;

        let emails = rows
            .into_iter()
            .map(|row| DbEmail {
                id: row.get("id"),
                account_id: row.get("account_id"),
                uidl: row.get("uidl"),
                folder: row.get("folder"),
                draft_id: row.get("draft_id"),
                from_address: row.get("from_address"),
                from_name: row.get("from_name"),
                to_addresses: row.get("to_addresses"),
                cc_addresses: row.get("cc_addresses"),
                subject: row.get("subject"),
                body_text: row.get("body_text"),
                body_html: row.get("body_html"),
                date: row.get("date"),
                is_read: row.get::<i32, _>("is_read") == 1,
                is_starred: row.get::<i32, _>("is_starred") == 1,
                has_attachment: row.get::<i32, _>("has_attachment") == 1,
                is_phishing: row.get::<i32, _>("is_phishing") == 1,
                phishing_reasons: row.get("phishing_reasons"),
                user_overridden: row.get::<i32, _>("user_overridden") == 1,
                deleted_at: row.get("deleted_at"),
                message_id_header: row.get("message_id_header"),
                in_reply_to_header: row.get("in_reply_to_header"),
                references_header: row.get("references_header"),
                thread_id: row.get("thread_id"),
            })
            .collect();

        Ok(emails)
    }

    /// Get sent count for an account (sent folder)
    pub async fn get_sent_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND folder = 'sent' AND deleted_at IS NULL")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取已发送邮件数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Get draft count for an account (drafts folder)
    pub async fn get_draft_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND folder = 'drafts' AND deleted_at IS NULL")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取草稿邮件数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Create a draft email
    pub async fn create_draft(&self, draft: &DbEmail) -> Result<String, String> {
        let draft_id = draft.draft_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            INSERT INTO emails
            (id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
             subject, body_text, body_html, date, is_read, is_starred, has_attachment,
             is_phishing, phishing_reasons, user_overridden, deleted_at)
            VALUES (?, ?, ?, 'drafts', ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 0, 0, 0, NULL, 0, NULL)
            "#,
        )
        .bind(&draft.id)
        .bind(&draft.account_id)
        .bind(&format!("draft-{}", draft_id))
        .bind(&draft_id)
        .bind(&draft.from_address)
        .bind(&draft.from_name)
        .bind(&draft.to_addresses)
        .bind(&draft.cc_addresses)
        .bind(&draft.subject)
        .bind(&draft.body_text)
        .bind(&draft.body_html)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("创建草稿失败: {}", e))?;

        Ok(draft_id)
    }

    /// Update a draft email by draft_id
    pub async fn update_draft(&self, draft_id: &str, updates: &DraftUpdates) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            UPDATE emails SET
                to_addresses = ?,
                cc_addresses = ?,
                subject = ?,
                body_text = ?,
                body_html = ?,
                has_attachment = ?,
                date = ?
            WHERE draft_id = ? AND folder = 'drafts'
            "#,
        )
        .bind(&updates.to_addresses)
        .bind(&updates.cc_addresses)
        .bind(&updates.subject)
        .bind(&updates.body_text)
        .bind(&updates.body_html)
        .bind(updates.has_attachment as i32)
        .bind(now)
        .bind(draft_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("更新草稿失败: {}", e))?;

        Ok(())
    }

    /// Delete a draft by draft_id
    pub async fn delete_draft(&self, draft_id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM emails WHERE draft_id = ? AND folder = 'drafts'")
            .bind(draft_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除草稿失败: {}", e))?;

        Ok(())
    }

    /// Mark draft as sent - move to sent folder and clear draft_id
    pub async fn mark_draft_as_sent(&self, draft_id: &str, sent_uidl: &str) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            UPDATE emails SET
                folder = 'sent',
                draft_id = NULL,
                uidl = ?,
                date = ?
            WHERE draft_id = ? AND folder = 'drafts'
            "#,
        )
        .bind(sent_uidl)
        .bind(now)
        .bind(draft_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("标记草稿已发送失败: {}", e))?;

        Ok(())
    }

    /// Get draft by draft_id
    pub async fn get_draft_by_id(&self, draft_id: &str) -> Result<Option<DbEmail>, String> {
        let row = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE draft_id = ? AND folder = 'drafts'
            "#,
        )
        .bind(draft_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("获取草稿失败: {}", e))?;

        Ok(row.map(|row| DbEmail {
            id: row.get("id"),
            account_id: row.get("account_id"),
            uidl: row.get("uidl"),
            folder: row.get("folder"),
            draft_id: row.get("draft_id"),
            from_address: row.get("from_address"),
            from_name: row.get("from_name"),
            to_addresses: row.get("to_addresses"),
            cc_addresses: row.get("cc_addresses"),
            subject: row.get("subject"),
            body_text: row.get("body_text"),
            body_html: row.get("body_html"),
            date: row.get("date"),
            is_read: row.get::<i32, _>("is_read") == 1,
            is_starred: row.get::<i32, _>("is_starred") == 1,
            has_attachment: row.get::<i32, _>("has_attachment") == 1,
            is_phishing: row.get::<i32, _>("is_phishing") == 1,
            phishing_reasons: row.get("phishing_reasons"),
            user_overridden: row.get::<i32, _>("user_overridden") == 1,
            deleted_at: row.get("deleted_at"),
            message_id_header: row.get("message_id_header"),
            in_reply_to_header: row.get("in_reply_to_header"),
            references_header: row.get("references_header"),
            thread_id: row.get("thread_id"),
        }))
    }

    // ==================== 联系人功能方法 ====================

    /// Upsert a contact - creates new or updates frequency if exists
    pub async fn upsert_contact(&self, email: &str, display_name: Option<&str>) -> Result<(), String> {
        let normalized_email = email.to_lowercase();
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            INSERT INTO contacts (id, email, display_name, last_contacted_at, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(email) DO UPDATE SET
                display_name = COALESCE(?, display_name),
                frequency = frequency + 1,
                last_contacted_at = ?,
                updated_at = ?
            "#,
        )
        .bind(&id)
        .bind(&normalized_email)
        .bind(display_name)
        .bind(now)
        .bind(now)
        .bind(now)
        .bind(display_name)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("保存联系人失败: {}", e))?;

        Ok(())
    }

    /// Search contacts by email or name prefix for autocomplete
    pub async fn search_contacts(&self, query: &str, limit: i32) -> Result<Vec<DbContact>, String> {
        let pattern = format!("%{}%", query.to_lowercase());
        let rows = sqlx::query(
            r#"
            SELECT id, email, display_name, avatar_url, frequency, last_contacted_at, created_at, updated_at
            FROM contacts
            WHERE email LIKE ? OR display_name LIKE ?
            ORDER BY frequency DESC, display_name ASC
            LIMIT ?
            "#,
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("搜索联系人失败: {}", e))?;

        let contacts = rows
            .into_iter()
            .map(|row| DbContact {
                id: row.get("id"),
                email: row.get("email"),
                display_name: row.get("display_name"),
                avatar_url: row.get("avatar_url"),
                frequency: row.get("frequency"),
                last_contacted_at: row.get("last_contacted_at"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(contacts)
    }

    /// Get all contacts ordered by frequency
    pub async fn get_all_contacts(&self) -> Result<Vec<DbContact>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, email, display_name, avatar_url, frequency, last_contacted_at, created_at, updated_at
            FROM contacts
            ORDER BY frequency DESC, display_name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取联系人列表失败: {}", e))?;

        let contacts = rows
            .into_iter()
            .map(|row| DbContact {
                id: row.get("id"),
                email: row.get("email"),
                display_name: row.get("display_name"),
                avatar_url: row.get("avatar_url"),
                frequency: row.get("frequency"),
                last_contacted_at: row.get("last_contacted_at"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(contacts)
    }

    /// Get contact by email
    pub async fn get_contact_by_email(&self, email: &str) -> Result<Option<DbContact>, String> {
        let normalized_email = email.to_lowercase();
        let row = sqlx::query(
            r#"
            SELECT id, email, display_name, avatar_url, frequency, last_contacted_at, created_at, updated_at
            FROM contacts
            WHERE email = ?
            LIMIT 1
            "#,
        )
        .bind(&normalized_email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("获取联系人失败: {}", e))?;

        Ok(row.map(|row| DbContact {
            id: row.get("id"),
            email: row.get("email"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            frequency: row.get("frequency"),
            last_contacted_at: row.get("last_contacted_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
    }

    /// Update contact info
    pub async fn update_contact(&self, id: &str, display_name: Option<&str>, avatar_url: Option<&str>) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query(
            r#"
            UPDATE contacts SET
                display_name = ?,
                avatar_url = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(display_name)
        .bind(avatar_url)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("更新联系人失败: {}", e))?;

        Ok(())
    }

    /// Delete contact by ID
    pub async fn delete_contact(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM contacts WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除联系人失败: {}", e))?;

        Ok(())
    }

    // ==================== 草稿附件功能方法 ====================

    /// Save a draft attachment
    pub async fn save_draft_attachment(&self, attachment: &DbDraftAttachment) -> Result<(), String> {
        log::info!("[save_draft_attachment] 开始保存: id={}, email_id={}, filename={}, content长度={}",
            attachment.id, attachment.email_id, attachment.filename, attachment.content.as_ref().map(|c| c.len()).unwrap_or(0));

        let result = sqlx::query(
            r#"
            INSERT OR REPLACE INTO draft_attachments
            (id, email_id, filename, content_type, file_path, content, file_size, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&attachment.id)
        .bind(&attachment.email_id)
        .bind(&attachment.filename)
        .bind(&attachment.content_type)
        .bind(&attachment.file_path)
        .bind(&attachment.content)
        .bind(attachment.file_size)
        .bind(attachment.created_at)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                log::info!("[save_draft_attachment] 保存成功: id={}", attachment.id);
                Ok(())
            }
            Err(e) => {
                log::error!("[save_draft_attachment] 保存失败: {}", e);
                Err(format!("保存草稿附件失败: {}", e))
            }
        }
    }

    /// Get all attachments for a draft by email_id (草稿邮件的 id)
    pub async fn get_draft_attachments(&self, email_id: &str) -> Result<Vec<DbDraftAttachment>, String> {
        let rows = sqlx::query(
            "SELECT id, email_id, filename, content_type, file_path, content, file_size, created_at FROM draft_attachments WHERE email_id = ? ORDER BY created_at ASC"
        )
        .bind(email_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取草稿附件失败: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|row| DbDraftAttachment {
                id: row.get("id"),
                email_id: row.get("email_id"),
                filename: row.get("filename"),
                content_type: row.get("content_type"),
                file_path: row.get("file_path"),
                content: row.get("content"),
                file_size: row.get("file_size"),
                created_at: row.get("created_at"),
            })
            .collect())
    }

    /// Delete a draft attachment by ID
    pub async fn delete_draft_attachment(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM draft_attachments WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除草稿附件失败: {}", e))?;

        Ok(())
    }

    /// Delete all attachments for a draft by email_id
    pub async fn delete_draft_attachments(&self, email_id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM draft_attachments WHERE email_id = ?")
            .bind(email_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除草稿附件失败: {}", e))?;

        Ok(())
    }

    // ==================== 邮件会话功能方法 ====================

    /// Upsert a thread - creates new or updates if exists
    pub async fn upsert_thread(&self, thread: &DbThread) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO threads (id, account_id, subject_base, last_message_at, message_count, has_unread)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                subject_base = COALESCE(?, subject_base),
                last_message_at = ?,
                message_count = ?,
                has_unread = ?
            "#,
        )
        .bind(&thread.id)
        .bind(&thread.account_id)
        .bind(&thread.subject_base)
        .bind(thread.last_message_at)
        .bind(thread.message_count)
        .bind(thread.has_unread as i32)
        .bind(&thread.subject_base)
        .bind(thread.last_message_at)
        .bind(thread.message_count)
        .bind(thread.has_unread as i32)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("保存会话失败: {}", e))?;

        Ok(())
    }

    /// Batch update thread_id for emails (called after frontend computes threads)
    pub async fn assign_thread_ids(&self, assignments: &[(&str, &str)]) -> Result<(), String> {
        for (email_id, thread_id) in assignments {
            sqlx::query("UPDATE emails SET thread_id = ? WHERE id = ?")
                .bind(thread_id)
                .bind(email_id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新邮件会话 ID 失败: {}", e))?;
        }

        Ok(())
    }

    /// Get all threads for an account
    pub async fn get_threads_for_account(&self, account_id: &str) -> Result<Vec<DbThread>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, subject_base, last_message_at, message_count, has_unread
            FROM threads
            WHERE account_id = ?
            ORDER BY last_message_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取会话列表失败: {}", e))?;

        let threads = rows
            .into_iter()
            .map(|row| DbThread {
                id: row.get("id"),
                account_id: row.get("account_id"),
                subject_base: row.get("subject_base"),
                last_message_at: row.get("last_message_at"),
                message_count: row.get("message_count"),
                has_unread: row.get::<i32, _>("has_unread") == 1,
            })
            .collect();

        Ok(threads)
    }

    /// Get all emails in a thread by thread_id
    pub async fn get_thread_emails(&self, thread_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE thread_id = ? AND deleted_at IS NULL
            ORDER BY date ASC
            "#,
        )
        .bind(thread_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取会话邮件失败: {}", e))?;

        let emails = rows
            .into_iter()
            .map(|row| DbEmail {
                id: row.get("id"),
                account_id: row.get("account_id"),
                uidl: row.get("uidl"),
                folder: row.get("folder"),
                draft_id: row.get("draft_id"),
                from_address: row.get("from_address"),
                from_name: row.get("from_name"),
                to_addresses: row.get("to_addresses"),
                cc_addresses: row.get("cc_addresses"),
                subject: row.get("subject"),
                body_text: row.get("body_text"),
                body_html: row.get("body_html"),
                date: row.get("date"),
                is_read: row.get::<i32, _>("is_read") == 1,
                is_starred: row.get::<i32, _>("is_starred") == 1,
                has_attachment: row.get::<i32, _>("has_attachment") == 1,
                is_phishing: row.get::<i32, _>("is_phishing") == 1,
                phishing_reasons: row.get("phishing_reasons"),
                user_overridden: row.get::<i32, _>("user_overridden") == 1,
                deleted_at: row.get("deleted_at"),
                message_id_header: row.get("message_id_header"),
                in_reply_to_header: row.get("in_reply_to_header"),
                references_header: row.get("references_header"),
                thread_id: row.get("thread_id"),
            })
            .collect();

        Ok(emails)
    }

    /// Get emails with threading headers for computation (all folders, no thread_id filter)
    pub async fn get_emails_for_threading(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, folder, draft_id, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden, deleted_at,
                   message_id_header, in_reply_to_header, references_header, thread_id
            FROM emails
            WHERE account_id = ? AND folder = 'inbox' AND deleted_at IS NULL
            ORDER BY date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取邮件列表失败: {}", e))?;

        let emails = rows
            .into_iter()
            .map(|row| DbEmail {
                id: row.get("id"),
                account_id: row.get("account_id"),
                uidl: row.get("uidl"),
                folder: row.get("folder"),
                draft_id: row.get("draft_id"),
                from_address: row.get("from_address"),
                from_name: row.get("from_name"),
                to_addresses: row.get("to_addresses"),
                cc_addresses: row.get("cc_addresses"),
                subject: row.get("subject"),
                body_text: row.get("body_text"),
                body_html: row.get("body_html"),
                date: row.get("date"),
                is_read: row.get::<i32, _>("is_read") == 1,
                is_starred: row.get::<i32, _>("is_starred") == 1,
                has_attachment: row.get::<i32, _>("has_attachment") == 1,
                is_phishing: row.get::<i32, _>("is_phishing") == 1,
                phishing_reasons: row.get("phishing_reasons"),
                user_overridden: row.get::<i32, _>("user_overridden") == 1,
                deleted_at: row.get("deleted_at"),
                message_id_header: row.get("message_id_header"),
                in_reply_to_header: row.get("in_reply_to_header"),
                references_header: row.get("references_header"),
                thread_id: row.get("thread_id"),
            })
            .collect();

        Ok(emails)
    }
}