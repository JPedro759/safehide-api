use mongodb::bson::oid::ObjectId;
use crate::types::email::Email;

pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: Email,
    pub password: String,
}