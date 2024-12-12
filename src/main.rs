mod day2;
mod day5;
mod day9;
mod day12;
mod requested_content_type;

use rocket::{get, routes};
use rocket::response::Redirect;
use crate::day12::Day12;
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
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .manage(Day9::new())
        .manage(Day12::new())
        .mount("/", routes![index, seek])
        .mount("/2", day2::routes())
        .mount("/5", day5::routes())
        .mount("/9", day9::routes())
        .mount("/12", day12::routes());

    Ok(rocket.into())
}
