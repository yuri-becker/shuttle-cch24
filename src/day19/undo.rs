use crate::day19::quote::Quote;
use crate::day19::schema::{CITE, UNDO};
use log::warn;
use rocket::data::ByteUnit;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::{put, Data, State};
use sqlx::{query, query_as, PgPool};
use std::ops::Deref;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct QuoteRequest {
    author: String,
    quote: String,
}
#[put("/undo/<id>", data = "<body>")]
pub async fn undo(id: &str, body: Data<'_>, db: &State<PgPool>) -> Result<Quote, Status> {
    let id = Uuid::parse_str(id).map_err(|_| {
        warn!("Could not parse UUID: {}", id);
        Status::BadRequest
    })?;
    let body = body.open(ByteUnit::Megabyte(1));
    let body = body.into_string().await.map_err(|err| {
        warn!("Couldn't read input: {}", err);
        Status::BadRequest
    })?;
    let body = serde_json::from_str::<QuoteRequest>(&body).map_err(|_| Status::BadRequest)?;
    let result = query(UNDO)
        .bind(body.author)
        .bind(body.quote)
        .bind(id)
        .execute(db.deref())
        .await
        .map_err(|err| {
            warn!("Failed to update quote: {}", err);
            Status::InternalServerError
        })?;
    if result.rows_affected() == 0 {
        return Err(Status::NotFound);
    }
    query_as(CITE)
        .bind(id)
        .fetch_one(db.deref())
        .await
        .map_err(|err| {
            warn!("Failed to fetch updated quote: {}", err);
            Status::InternalServerError
        })
}
