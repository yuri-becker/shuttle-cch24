use crate::day19::schema::RESET;
use log::warn;
use rocket::http::Status;
use rocket::{post, State};
use sqlx::{query, PgPool};
use std::ops::Deref;

#[post("/reset")]
pub async fn reset(db: &State<PgPool>) -> Result<(), Status> {
    query(RESET)
        .execute(db.deref())
        .await
        .map(|_| ())
        .map_err(|err| {
            warn!("Failed to delete quotes: {}", err);
            Status::InternalServerError
        })
}
