use rocket::{post, State};
use sqlx::{query, PgPool};
use rocket::http::Status;
use log::warn;
use std::ops::Deref;
use crate::day19::schema::RESET;

#[post("/reset")]
pub  async fn reset(db: &State<PgPool>) -> Result<(), Status> {
    query(RESET)
        .execute(db.deref())
        .await
        .map(|_| ())
        .map_err(|err| {
            warn!("Failed to delete quotes: {}", err);
            Status::InternalServerError
        })
}