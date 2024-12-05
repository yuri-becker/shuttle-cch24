mod day2;
mod day5;

use rocket::{get, routes};
use rocket::response::Redirect;


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
        .mount("/", routes![index, seek])
        .mount("/2", day2::routes())
        .mount("/5", day5::routes());

    Ok(rocket.into())
}
