use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::types::email::Email;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Email,
    pub password: String,
    pub account_status: AccountStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: Email,
    pub password: String,
    pub account_status: AccountStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Locked,
    Compromised,
    PendingDeletion
}