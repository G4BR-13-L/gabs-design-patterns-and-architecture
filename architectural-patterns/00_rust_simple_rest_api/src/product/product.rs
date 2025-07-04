use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Produto {
    pub uuid: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub created_at: chrono::NaiveDateTime,
    pub active: bool,
    pub available: bool,
}
