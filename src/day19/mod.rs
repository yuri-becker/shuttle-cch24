use rocket::{routes, Route};
mod quote;
pub mod schema;
mod page;
mod reset;
mod cite;
mod remove;
mod draft;
mod undo;
mod list;

pub fn routes() -> Vec<Route> {
    routes![reset::reset, cite::cite, remove::remove, undo::undo, draft::draft, list::list]
}
