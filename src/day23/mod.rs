use crate::day23::lockfile::Lockfile;
use crate::day23::ornament_state::OrnamentState;
use log::{info, warn};
use present_color::PresentColor;
use rocket::form::Form;
use rocket::http::Status;
use rocket::{form, get, post, routes, FromForm, Route};
use std::str::FromStr;

mod lockfile;
mod ornament_state;
mod present_color;

#[get("/star")]
fn star() -> &'static str {
    "<div id=\"star\" class=\"lit\"></div>"
}

#[get("/present/<color>")]
fn color(color: &str) -> Result<String, Status> {
    let color = PresentColor::from_str(color).map_err(|_| Status::ImATeapot)?;
    Ok(format!(
        r#"<div class="present {}" hx-get="/23/present/{}" hx-swap="outerHTML">
    <div class="ribbon"></div>
    <div class="ribbon"></div>
    <div class="ribbon"></div>
    <div class="ribbon"></div>
</div>"#,
        color,
        color.next()
    ))
}

#[get("/ornament/<state>/<n>")]
fn ornament(state: &str, n: &str) -> Result<String, Status> {
    let state = OrnamentState::from_str(state).map_err(|_| Status::ImATeapot)?;
    let n = html_escape::encode_safe(n);

    let response = format!(
        r#"<div class="ornament{}" id="ornament{}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{}/{}" hx-swap="outerHTML"></div>"#,
        if state == OrnamentState::On {
            " on"
        } else {
            ""
        },
        n,
        state.flip(),
        n
    );
    info!("Responding with {}", response);
    Ok(response)
}

#[derive(FromForm)]
struct LockfileForm {
    lockfile: String,
}

#[post("/lockfile", data = "<form>")]
fn post_lockfile(form: form::Result<Form<LockfileForm>>) -> Result<String, Status> {
    if let Err(err) = form {
        warn!("Could not parse form: {}", err);
        return Err(Status::BadRequest);
    }
    let lockfile = toml::from_str::<Lockfile>(&form.unwrap().lockfile).map_err(|err| {
        warn!("Could not parse lockfile: {}", err);
        Status::BadRequest
    })?;
    let response = lockfile
        .package
        .iter()
        .map(|package| {
            let color = package.color()?;
            let top = package.top()?;
            let left = package.left()?;

            Ok(match (color, top, left) {
                (Some(color), Some(top), Some(left)) => format!(
                    r#"<div style="background-color:{};top:{}px;left:{}px;"></div>{}"#,
                    color, top, left, "\n"
                ),
                _ => "".to_string(),
            })
        })
        .collect::<Result<String, &str>>()
        .map_err(|err| {
            warn!("Could not interpret lockfile: {}", err);
            Status::UnprocessableEntity
        })?;

    info!("Responding with {}", response);
    Ok(response)
}

pub fn routes() -> Vec<Route> {
    routes![star, color, ornament, post_lockfile]
}
