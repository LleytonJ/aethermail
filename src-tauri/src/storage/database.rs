//! Database operations using SQLite

use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::path::PathBuf;

use super::models::*;

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

    /// Create database tables
    async fn init_tables(&self) -> Result<(), String> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS emails (
                id TEXT PRIMARY KEY,
                account_id TEXT NOT NULL,
                uidl TEXT NOT NULL,
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

            CREATE INDEX IF NOT EXISTS idx_emails_account ON emails(account_id);
            CREATE INDEX IF NOT EXISTS idx_emails_date ON emails(date);
            CREATE INDEX IF NOT EXISTS idx_emails_uidl ON emails(account_id, uidl);
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

        Ok(())
    }

    /// Save an email to database
    pub async fn save_email(&self, email: &DbEmail) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO emails
            (id, account_id, uidl, from_address, from_name, to_addresses, cc_addresses,
             subject, body_text, body_html, date, is_read, is_starred, has_attachment,
             is_phishing, phishing_reasons, user_overridden)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&email.id)
        .bind(&email.account_id)
        .bind(&email.uidl)
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
        .execute(&self.pool)
        .await
        .map_err(|e| format!("保存邮件失败: {}", e))?;

        Ok(())
    }

    /// Get all emails for an account (excluding phishing emails)
    pub async fn get_emails(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden
            FROM emails
            WHERE account_id = ? AND is_phishing = 0
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
            })
            .collect();

        Ok(emails)
    }

    /// Get a single email by ID
    pub async fn get_email_by_id(&self, id: &str) -> Result<Option<DbEmail>, String> {
        let row = sqlx::query(
            r#"
            SELECT id, account_id, uidl, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden
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

    /// Get unread count for an account (excluding phishing emails)
    pub async fn get_unread_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND is_read = 0 AND is_phishing = 0")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取未读数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Update phishing status for an email
    pub async fn update_phishing_status(
        &self,
        id: &str,
        is_phishing: bool,
        phishing_reasons: Option<String>,
        user_overridden: Option<bool>,
    ) -> Result<(), String> {
        // 如果提供了 user_overridden，则更新它
        if let Some(overridden) = user_overridden {
            sqlx::query("UPDATE emails SET is_phishing = ?, phishing_reasons = ?, user_overridden = ? WHERE id = ?")
                .bind(is_phishing as i32)
                .bind(phishing_reasons)
                .bind(overridden as i32)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新钓鱼状态失败: {}", e))?;
        } else {
            sqlx::query("UPDATE emails SET is_phishing = ?, phishing_reasons = ? WHERE id = ?")
                .bind(is_phishing as i32)
                .bind(phishing_reasons)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新钓鱼状态失败: {}", e))?;
        }

        Ok(())
    }

    /// Get all phishing emails for an account
    pub async fn get_phishing_emails(&self, account_id: &str) -> Result<Vec<DbEmail>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, uidl, from_address, from_name, to_addresses, cc_addresses,
                   subject, body_text, body_html, date, is_read, is_starred, has_attachment,
                   is_phishing, phishing_reasons, user_overridden
            FROM emails
            WHERE account_id = ? AND is_phishing = 1
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
            })
            .collect();

        Ok(emails)
    }

    /// Get phishing count for an account
    pub async fn get_phishing_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND is_phishing = 1")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取钓鱼邮件数量失败: {}", e))?;

        Ok(row.get("count"))
    }

    /// Get unread phishing count for an account
    pub async fn get_phishing_unread_count(&self, account_id: &str) -> Result<i32, String> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM emails WHERE account_id = ? AND is_phishing = 1 AND is_read = 0")
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("获取钓鱼邮件未读数量失败: {}", e))?;

        Ok(row.get("count"))
    }
}