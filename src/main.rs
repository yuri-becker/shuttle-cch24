mod requested_content_type;
mod day2;
mod day5;
mod day9;
mod day12;
mod day16;
mod day19;

use rocket::{get, routes};
use rocket::response::Redirect;
use sqlx::Executor;
use crate::day12::Day12;
use crate::day16::Day16;
use crate::day9::Day9;

#[get("/")]
fn index() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
fn seek() -> Redirect {
    Redirect::found("https://www.youtube.com/watch?v=9Gc4QTqslN4")
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pg_pool: sqlx::PgPool
) -> shuttle_rocket::ShuttleRocket {
    pg_pool.execute(day19::schema::SCHEMA)
        .await
        .expect("Failed to create table");
    
    let rocket = rocket::build()
        .manage(Day9::new())
        .manage(Day12::new())
        .manage(Day16::new())
        .manage(pg_pool)
        .mount("/", routes![index, seek])
        .mount("/2", day2::routes())
        .mount("/5", day5::routes())
        .mount("/9", day9::routes())
        .mount("/12", day12::routes())
        .mount("/16", day16::routes())
        .mount("/19", day19::routes());

    Ok(rocket.into())
}
