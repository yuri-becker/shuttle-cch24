use crate::requested_content_type::RequestedContentType;
use leaky_bucket::RateLimiter;
use rocket::data::ByteUnit;
use rocket::http::Status;
use rocket::{post, routes, Data, Route, State};
use serde::{Deserialize, Serialize};
use std::sync::{PoisonError, RwLock, RwLockWriteGuard};
use std::time::Duration;

const BAD_REQUEST: (Status, &str) = (Status::BadRequest, "Bad Request");
const INTERNAL_ERROR: (Status, &str) = (Status::InternalServerError, "Internal Server Error");
const PINTS_PER_LITER: f32 = 1.759_754_1;
const GALLONS_PER_LITER: f32 = 0.264_172_05;

pub struct Day9 {
    rate_limiter: RwLock<RateLimiter>,
}

impl Day9 {
    pub fn new() -> Self {
        Self {
            rate_limiter: RwLock::new(Self::build_rate_limiter()),
        }
    }

    pub fn refill_rate_limiter(
        &self,
    ) -> Result<(), PoisonError<RwLockWriteGuard<'_, RateLimiter>>> {
        let mut write = self.rate_limiter.write()?;
        *write = Self::build_rate_limiter();
        Ok(())
    }

    fn build_rate_limiter() -> RateLimiter {
        RateLimiter::builder()
            .max(5)
            .initial(5)
            .refill(1)
            .interval(Duration::from_secs(1))
            .build()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Conversion {
    Gallons(f32),
    Liters(f32),
    Litres(f32),
    Pints(f32),
}

impl Conversion {
    fn convert(self) -> Self {
        match self {
            Self::Gallons(gallons) => Self::Liters(gallons / GALLONS_PER_LITER),
            Self::Liters(liters) => Self::Gallons(liters * GALLONS_PER_LITER),
            Self::Pints(pints) => Self::Litres(pints / PINTS_PER_LITER),
            Self::Litres(litres) => Self::Pints(litres * PINTS_PER_LITER),
        }
    }
}

#[post("/milk", data = "<body>")]
pub async fn milk(
    state: &State<Day9>,
    content_type: RequestedContentType,
    body: Data<'_>,
) -> Result<String, (Status, &'static str)> {
    if !state
        .rate_limiter
        .read()
        .map_err(|_| INTERNAL_ERROR)?
        .try_acquire(1)
    {
        return Err((Status::TooManyRequests, "No milk available\n"));
    }
    match content_type.as_deref() {
        Some("application/json") => {
            let body = body.open(ByteUnit::Megabyte(1));
            let body = body.into_string().await.map_err(|_| BAD_REQUEST)?;
            let body = serde_json::from_str::<Conversion>(&body).map_err(|_| BAD_REQUEST)?;
            Ok(serde_json::to_string(&body.convert()).unwrap())
        }
        _ => Ok("Milk withdrawn\n".to_string()),
    }
}

#[post("/refill")]
pub fn refill(state: &State<Day9>) -> Result<(), (Status, &'static str)> {
    state.refill_rate_limiter().map_err(|_| INTERNAL_ERROR)?;
    Ok(())
}

pub fn routes() -> Vec<Route> {
    routes![milk, refill]
}
