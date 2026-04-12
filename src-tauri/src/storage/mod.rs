//! Local Storage Module
//!
//! Provides SQLite database operations for email and account persistence.

mod database;
mod models;

pub use database::*;
pub use models::*;