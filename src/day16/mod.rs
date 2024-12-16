use josekit::jws::alg::rsassa::{RsassaJwsSigner, RsassaJwsVerifier};
use josekit::jws::{JwsHeader, RS256, RS512};
use josekit::jwt::JwtPayload;
use josekit::{jwt, JoseError};
use log::warn;
use rocket::data::ByteUnit;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::{get, post, routes, Data, Route, State};
use serde_json::Value;
const PRIVATE_KEY: &str = include_str!("private.pem");
const PUBLIC_KEY: &str = include_str!("public.pem");
const SANTA_PUBLIC_KEY: &str = include_str!("day16_santa_public_key.pem");
const COOKIE: &str = "gift";
const CLAIM_WRAPPED_GIFT: &str = "wrapped_gift";

pub struct Day16 {
    signer: RsassaJwsSigner,
    verifier: RsassaJwsVerifier,
    santa_verifier: RsassaJwsVerifier,
    santa_verifier_rs512: RsassaJwsVerifier,
}

impl Day16 {
    pub fn new() -> Self {
        Day16 {
            signer: RS256.signer_from_pem(&PRIVATE_KEY).unwrap(),
            verifier: RS256.verifier_from_pem(&PUBLIC_KEY).unwrap(),
            santa_verifier: RS256.verifier_from_pem(&SANTA_PUBLIC_KEY).unwrap(),
            santa_verifier_rs512: RS512.verifier_from_pem(&SANTA_PUBLIC_KEY).unwrap(),
        }
    }
}

#[post("/wrap", data = "<body>")]
async fn wrap(body: Data<'_>, state: &State<Day16>, cookies: &CookieJar<'_>) -> Result<(), Status> {
    let body = body.open(ByteUnit::Megabyte(1));
    let body = body.into_string().await.map_err(|err| {
        warn!("Couldn't read input: {}", err);
        Status::BadRequest
    })?;
    let body = serde_json::from_str::<Value>(&body).map_err(|err| {
        warn!("Couldn't parse input: {}", err);
        Status::BadRequest
    })?;

    let mut header = JwsHeader::new();
    header.set_token_type("JWT");
    let mut payload = JwtPayload::new();
    payload
        .set_claim(CLAIM_WRAPPED_GIFT, Some(body))
        .map_err(|err| {
            warn!("Couldn't set claim payload: {}", err);
            Status::InternalServerError
        })?;

    let jwt = jwt::encode_with_signer(&payload, &header, &state.signer).map_err(|err| {
        warn!("Could not encode JWT: {}", err);
        Status::InternalServerError
    })?;
    cookies.add(Cookie::new(COOKIE, jwt));
    Ok(())
}

#[get("/unwrap")]
async fn unwrap(state: &State<Day16>, cookies: &CookieJar<'_>) -> Result<String, Status> {
    let jwt = cookies.get(COOKIE).ok_or(Status::BadRequest)?.value();

    let (payload, _header) = jwt::decode_with_verifier(jwt, &state.verifier).map_err(|err| {
        warn!("Couldn't verify JWT: {}", err);
        Status::BadRequest
    })?;

    let gift = payload
        .claim(CLAIM_WRAPPED_GIFT)
        .ok_or(Status::BadRequest)?;
    serde_json::to_string(gift).map_err(|err| {
        warn!("Couldn't parse wrapped_gift claim: {}", err);
        Status::BadRequest
    })
}

#[post("/decode", data = "<body>")]
async fn decode(body: Data<'_>, state: &State<Day16>) -> Result<String, Status> {
    let jwt = body.open(ByteUnit::Megabyte(1));
    let jwt = jwt.into_string().await.map_err(|err| {
        warn!("Couldn't read input: {}", err);
        Status::BadRequest
    })?;
    let header = jwt::decode_header(&jwt.value).map_err(|err| {
        warn!("Couldn't decode JWT: {}", err);
        Status::BadRequest
    })?;
    let (payload, _) = jwt::decode_with_verifier(
        &jwt.value,
        match header.claim("alg").and_then(|it| it.as_str()) {
            Some("RS512") => &state.santa_verifier_rs512,
            _ => &state.santa_verifier,
        },
    )
    .map_err(|err| {
        warn!("Couldn't verify JWT: {}", err);
        match err {
            JoseError::InvalidSignature(_) => Status::Unauthorized,
            _ => Status::BadRequest,
        }
    })?;
    serde_json::to_string(&payload.claims_set()).map_err(|err| {
        warn!("Couldn't serialize JWT: {}", err);
        Status::BadRequest
    })
}

pub fn routes() -> Vec<Route> {
    routes![wrap, unwrap, decode]
}
