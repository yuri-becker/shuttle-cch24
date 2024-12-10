use rocket::{async_trait, Request};
use rocket::request::{FromRequest, Outcome};
use std::ops::Deref;

pub struct RequestedContentType {
    inner: Option<String>,
}

#[async_trait]
impl<'r> FromRequest<'r> for RequestedContentType {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = request.headers().get_one("Content-Type");
        Outcome::Success(RequestedContentType {
            inner: header.map(|it| it.to_string()),
        })
    }
}

impl Deref for RequestedContentType {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}