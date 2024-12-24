use crate::day19::quote::Quote;
use crate::day19::schema::{CITE, DRAFT};
use chrono::Local;
use log::warn;
use rocket::data::ByteUnit;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::{post, Data, State};
use sqlx::{query, query_as, PgPool};
use std::ops::Deref;
use uuid::Uuid;
#[derive(Serialize, Deserialize)]
struct DraftRequest {
    author: String,
    quote: String,
}

#[post("/draft", data = "<body>")]
pub async fn draft(body: Data<'_>, db: &State<PgPool>) -> Result<(Status, Quote), Status> {
    let body = body.open(ByteUnit::Megabyte(1));
    let body = body.into_string().await.map_err(|err| {
        warn!("Couldn't read input: {}", err);
        Status::BadRequest
    })?;
    let body = serde_json::from_str::<DraftRequest>(&body).map_err(|_| Status::BadRequest)?;

    let uuid = Uuid::now_v7();
    query(DRAFT)
        .bind(uuid)
        .bind(body.author)
        .bind(body.quote)
        .bind(Local::now())
        .bind(0)
        .execute(db.deref())
        .await
        .map_err(|err| {
            warn!("Failed to insert quote: {}", err);
            Status::InternalServerError
        })?;
    let quote: Quote = query_as(CITE)
        .bind(uuid)
        .fetch_one(db.deref())
        .await
        .map_err(|err| {
            warn!("Failed to fetch inserted quote: {}", err);
            Status::InternalServerError
        })?;
    Ok((Status::Created, quote))
}
