use crate::day19::quote::Quote;
use crate::day19::schema::{CITE, REMOVE};
use log::warn;
use rocket::http::Status;
use rocket::{delete, State};
use sqlx::{query, query_as, PgPool};
use std::ops::Deref;
use uuid::Uuid;

#[delete("/remove/<id>")]
pub async fn remove(id: &str, db: &State<PgPool>) -> Result<Quote, Status> {
    let id = Uuid::parse_str(id).map_err(|_| Status::NotFound)?;
    let quote: Option<Quote> = query_as(CITE)
        .bind(id)
        .fetch_optional(db.deref())
        .await
        .map_err(|err| {
            warn!("Failed to fetch quote: {}", err);
            Status::InternalServerError
        })?;

    let quote = quote.ok_or(Status::NotFound)?;

    query(REMOVE)
        .bind(id)
        .execute(db.deref())
        .await
        .map_err(|err| {
            warn!("Failed to delete quote: {}", err);
            Status::InternalServerError
        })?;
    Ok(quote)
}
