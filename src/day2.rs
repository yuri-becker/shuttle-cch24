use rocket::http::{RawStr, Status};
use rocket::{get, routes, Route};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::BitXor;
use std::str::FromStr;

#[get("/dest?<from>&<key>")]
fn dest(from: &str, key: &str) -> Result<String, Status> {
    let from = Ipv4Addr::from_str(from)
        .map_err(|_| Status::BadRequest)?
        .octets();
    let key = Ipv4Addr::from_str(key)
        .map_err(|_| Status::BadRequest)?
        .octets();

    let mut result: [u8; 4] = [0; 4];
    for i in 0..4 {
        result[i] = from[i].overflowing_add(key[i]).0;
    }
    let result = Ipv4Addr::from(result);
    Ok(result.to_string())
}

#[get("/key?<from>&<to>")]
fn key(from: &str, to: &str) -> Result<String, Status> {
    let from = Ipv4Addr::from_str(from)
        .map_err(|_| Status::BadRequest)?
        .octets();
    let to = Ipv4Addr::from_str(to)
        .map_err(|_| Status::BadRequest)?
        .octets();

    let mut result: [u8; 4] = [0; 4];
    for i in 0..4 {
        result[i] = to[i].overflowing_sub(from[i]).0;
    }
    let result = Ipv4Addr::from(result);
    Ok(result.to_string())
}

#[get("/v6/dest?<from>&<key>")]
fn v6_dest(from: &str, key: &str) -> Result<String, Status> {
        let from = Ipv6Addr::from_str(from)
        .map_err(|_| Status::BadRequest)?
        .octets();
    let key = Ipv6Addr::from_str(key)
        .map_err(|_| Status::BadRequest)?
        .octets();

    let mut result: [u8; 16] = [0; 16];
    for i in 0..16 {
        result[i] = from[i].bitxor(key[i]);
    }
    let result = Ipv6Addr::from(result);
    Ok(result.to_string())
}

#[get("/v6/key?<from>&<to>")]
fn v6_key(from: &str, to: &str) -> Result<String, Status> {
    let from = Ipv6Addr::from_str(from)
        .map_err(|_| Status::BadRequest)?
        .octets();
    let to = Ipv6Addr::from_str(to)
        .map_err(|_| Status::BadRequest)?
        .octets();

    let mut result: [u8; 16] = [0; 16];
    for i in 0..16 {
        result[i] = to[i].bitxor(from[i]);
    }
    let result = Ipv6Addr::from(result);
    Ok(result.to_string())
}

pub fn routes() -> Vec<Route> {
    routes![dest, key, v6_dest, v6_key]
}
