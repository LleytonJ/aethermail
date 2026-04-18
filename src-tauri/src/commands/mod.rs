//! Tauri Commands Module
//!
//! Provides commands for frontend-backend communication.

pub mod account;
pub mod ai;
pub mod contact;
pub mod email;

pub use account::*;
pub use ai::*;
pub use contact::*;
pub use email::*;