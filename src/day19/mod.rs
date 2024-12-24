use rocket::{routes, Route};
mod cite;
mod draft;
mod list;
mod page;
mod quote;
mod remove;
mod reset;
pub mod schema;
mod undo;

pub fn routes() -> Vec<Route> {
    routes![
        reset::reset,
        cite::cite,
        remove::remove,
        undo::undo,
        draft::draft,
        list::list
    ]
}
