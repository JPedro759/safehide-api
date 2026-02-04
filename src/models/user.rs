use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::types::email::Email;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: Email,
    pub password: String,
    pub created_at: DateTime<Utc>,
}