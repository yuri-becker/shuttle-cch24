use crate::day19::quote::Quote;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::response::Responder;
use rocket::Request;
use serde::{Deserialize, Serialize};

const DELIMITER: char = 'P';
const TOKEN_LENGTH: usize = 16;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    pub quotes: Vec<Quote>,
    pub page: u32,
    pub next_token: Option<String>,
}

pub fn page_to_token(page: u32) -> String {
    let page = page.to_string();
    let random_length = TOKEN_LENGTH - page.len() - 1;

    let random_part: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .filter(|it| it != &DELIMITER)
        .take(random_length)
        .collect();
    format!("{}{}{}", random_part, DELIMITER, page)
}

pub fn token_to_page(token: String) -> Result<u32, ()> {
    token
        .find(DELIMITER)
        .map(|index| token.split_at(index + 1).1)
        .ok_or(())
        .map(|it| it.parse::<u32>())
        .and_then(|inner| inner.map_err(|_| ()))
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Page {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        serde_json::to_string(&self).unwrap().respond_to(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_page_to_token() {
        let p1 = page_to_token(1);
        assert!(p1.ends_with("P1"));
        assert!(p1.len() == 16);
        assert!(p1.chars().all(|it| it.is_ascii_alphanumeric()));
        let p16 = page_to_token(16);
        assert!(p16.ends_with("P16"));
        assert!(p16.len() == 16);
        assert!(p16.chars().all(|it| it.is_ascii_alphanumeric()));
        let p11111 = page_to_token(11111);
        assert!(p11111.ends_with("P11111"));
        assert!(p11111.len() == 16);
        assert!(p11111.chars().all(|it| it.is_ascii_alphanumeric()));
    }

    #[test]
    pub fn test_token_to_page() {
        let p1 = token_to_page("msdan12jdP1".to_string()).unwrap();
        assert_eq!(p1, 1);
        let p1 = token_to_page("msas2213hddP1293".to_string()).unwrap();
        assert_eq!(p1, 1293);
        let invalid = token_to_page("ashkjdashd".to_string());
        assert!(invalid.is_err());
        let invalid = token_to_page("ashkjdashdP".to_string());
        assert!(invalid.is_err());
    }
}
