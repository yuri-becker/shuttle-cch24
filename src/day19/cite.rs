use rocket::{get, State};
use sqlx::{query_as, PgPool};
use rocket::http::Status;
use uuid::Uuid;
use log::warn;
use std::ops::Deref;
use crate::day19::quote::Quote;
use crate::day19::schema::CITE;

#[get("/cite/<id>")]
pub async fn cite(id: &str, db: &State<PgPool>) -> Result<Quote, Status> {
    let id = Uuid::parse_str(id).map_err(|_| Status::NotFound)?;
    let result: Option<Quote> = query_as(CITE)
        .bind(id)
        .fetch_optional(db.deref())
        .await
        .map_err(|err| {
            warn!("Failed to fetch cite: {}", err);
            Status::InternalServerError
        })?;
    result.ok_or(Status::NotFound)
}