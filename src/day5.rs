use crate::requested_content_type::RequestedContentType;
use cargo_manifest::Manifest;
use rocket::data::ByteUnit;
use rocket::form::validate::Contains;
use rocket::http::Status;
use rocket::log::private::warn;
use rocket::{post, routes, Data, Route};
use shuttle_runtime::__internals::serde_json;
use std::str::FromStr;

static NO_CONTENT: (Status, &str) = (Status::NoContent, "No Content");
static UNSUPPORTED_MEDIA: (Status, &str) = (Status::UnsupportedMediaType, "Unsupported Media Type");
static INVALID_MANIFEST: (Status, &str) = (Status::BadRequest, "Invalid manifest");

#[post("/manifest", data = "<body>")]
async fn manifest(
    body: Data<'_>,
    content_type: RequestedContentType,
) -> Result<String, (Status, &'static str)> {
    if content_type.is_none() {
        return Err(UNSUPPORTED_MEDIA);
    }
    let body = body.open(ByteUnit::Megabyte(1));
    let body = body.into_string().await.map_err(|err| {
        warn!("Couldn't read input: {}", err);
        INVALID_MANIFEST
    })?;

    let manifest: Result<Manifest, (Status, &'static str)> =
        match content_type.clone().unwrap().as_str() {
            "application/json" => serde_json::from_str::<Manifest>(body.as_str()).map_err(|err| {
                warn!("Couldn't parse Manifest as JSON: {}", err);
                INVALID_MANIFEST
            }),
            "application/yaml" => serde_yaml::from_str::<Manifest>(body.as_str()).map_err(|err| {
                warn!("Couldn't parse Manifest as YAML: {}", err);
                INVALID_MANIFEST
            }),
            "application/toml" => Manifest::from_str(body.as_str()).map_err(|err| {
                warn!("Couldn't parse Manifest as TOML: {}", err);
                INVALID_MANIFEST
            }),
            _ => Err(UNSUPPORTED_MEDIA),
        };
    let package = manifest?.package.ok_or(NO_CONTENT)?;

    let keywords = package
        .keywords
        .map(|it| it.as_local())
        .unwrap_or_default()
        .unwrap_or_default();

    if !keywords.contains("Christmas 2024".to_string()) {
        return Err((Status::BadRequest, "Magic keyword not provided"));
    }

    let metadata = package.metadata.ok_or(NO_CONTENT)?;

    let orders = metadata
        .as_table()
        .ok_or(NO_CONTENT)?
        .get("orders")
        .ok_or(NO_CONTENT)?
        .as_array()
        .ok_or(NO_CONTENT)?;

    let valid_items = orders
        .iter()
        .filter_map(|order| {
            let order = order.as_table()?;
            let item = order.get("item")?;
            let item = item.as_str()?;
            let quantity = order.get("quantity")?;
            let quantity = quantity.as_integer()?;
            Some(format!("{}: {}", item, quantity))
        })
        .collect::<Vec<String>>();

    if valid_items.is_empty() {
        return Err(NO_CONTENT);
    }
    Ok(valid_items.join("\n"))
}

pub fn routes() -> Vec<Route> {
    routes![manifest]
}
