mod day12;
mod day16;
mod day19;
mod day2;
mod day23;
mod day5;
mod day9;
mod requested_content_type;

use crate::day12::Day12;
use crate::day16::Day16;
use crate::day9::Day9;
use log::warn;
use rocket::data::{ByteUnit, Limits};
use rocket::fs::{relative, NamedFile};
use rocket::response::Redirect;
use rocket::{get, routes, Config};
use sqlx::Executor;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
fn seek() -> Redirect {
    Redirect::found("https://www.youtube.com/watch?v=9Gc4QTqslN4")
}

#[get("/<path..>")]
pub async fn static_files(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("assets")).join(path);
    NamedFile::open(path.clone())
        .await
        .inspect_err(|err| {
            warn!("Could not get file at path {:?}: {:?}", path, err);
        })
        .ok()
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pg_pool: sqlx::PgPool,
) -> shuttle_rocket::ShuttleRocket {
    pg_pool
        .execute(day19::schema::SCHEMA)
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
        .mount("/19", day19::routes())
        .mount("/23", day23::routes())
        .mount("/assets", routes![static_files])
        .configure(Config {
            limits: Limits::default()
                .limit("form", ByteUnit::Megabyte(5))
                .limit("string", ByteUnit::Megabyte(5)),
            ..Default::default()
        });

    Ok(rocket.into())
}
