use crate::day19::page::{page_to_token, token_to_page, Page};
use crate::day19::quote::Quote;
use crate::day19::schema::{Count, COUNT, LIST};
use log::{info, warn};
use rocket::http::Status;
use rocket::{get, State};
use sqlx::{query_as, PgPool};
use std::ops::Deref;

const PAGE_SIZE: usize = 3;

#[get("/list?<token>")]
pub async fn list(token: Option<String>, db: &State<PgPool>) -> Result<Page, Status> {
    let page = token
        .clone()
        .map(token_to_page)
        .transpose()
        .map_err(|_| Status::BadRequest)?
        .unwrap_or(0);
    info!("Page is {} for token {:?}", page, token);

    let count: Count = query_as(COUNT).fetch_one(db.deref()).await.map_err(|err| {
        warn!("Cound not count Quotes: {}", err);
        Status::InternalServerError
    })?;
    info!("Count is {}", count.count);

    let quotes: Vec<Quote> = query_as(LIST)
        .bind(PAGE_SIZE as i32)
        .bind(page as i64 * PAGE_SIZE as i64)
        .fetch_all(db.deref())
        .await
        .map_err(|err| {
            warn!("Could not query: {}", err);
            Status::InternalServerError
        })?;
    info!("List is {:?}", quotes);

    let items_left = (PAGE_SIZE as i64 * (page as i64 + 1)) < count.count;
    info!("Items left is {}", items_left);

    let response = Page {
        page: page + 1,
        quotes,
        next_token: if items_left { Some(page_to_token(page + 1)) } else { None }
    };
    info!("Response is {:?}", response);
    Ok(response)
}
