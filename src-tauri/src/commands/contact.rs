//! Contact management commands

use serde::{Deserialize, Serialize};
use crate::storage::{DbContact};
use crate::commands::email::get_db;

/// Contact data for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub frequency: i32,
    pub last_contacted_at: Option<i64>,
}

impl From<DbContact> for Contact {
    fn from(db: DbContact) -> Self {
        Self {
            id: db.id,
            email: db.email,
            display_name: db.display_name,
            avatar_url: db.avatar_url,
            frequency: db.frequency,
            last_contacted_at: db.last_contacted_at,
        }
    }
}

/// Search contacts for autocomplete
#[tauri::command]
pub async fn search_contacts(query: String, limit: i32) -> Result<Vec<Contact>, String> {
    let db = get_db().await?;
    let contacts = db.search_contacts(&query, limit).await?;
    Ok(contacts.into_iter().map(Contact::from).collect())
}

/// Get all contacts
#[tauri::command]
pub async fn get_all_contacts() -> Result<Vec<Contact>, String> {
    let db = get_db().await?;
    let contacts = db.get_all_contacts().await?;
    Ok(contacts.into_iter().map(Contact::from).collect())
}

/// Get contact by email
#[tauri::command]
pub async fn get_contact_by_email(email: String) -> Result<Option<Contact>, String> {
    let db = get_db().await?;
    let contact = db.get_contact_by_email(&email).await?;
    Ok(contact.map(Contact::from))
}

/// Update contact
#[tauri::command]
pub async fn update_contact(
    id: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
) -> Result<(), String> {
    let db = get_db().await?;
    db.update_contact(&id, display_name.as_deref(), avatar_url.as_deref()).await?;
    Ok(())
}

/// Delete contact
#[tauri::command]
pub async fn delete_contact(id: String) -> Result<(), String> {
    let db = get_db().await?;
    db.delete_contact(&id).await?;
    Ok(())
}