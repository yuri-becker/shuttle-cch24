use chrono::{DateTime, Local};
use rocket::response::Responder;
use rocket::Request;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, FromRow, Decode)]
pub struct Quote {
    pub id: Uuid,
    pub author: String,
    pub quote: String,
    pub created_at: DateTime<Local>,
    pub version: i32,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Quote {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        serde_json::to_string(&self).unwrap().respond_to(request)
    }
}
