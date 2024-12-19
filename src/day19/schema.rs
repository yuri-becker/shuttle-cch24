use rocket::serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};

pub const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    author TEXT NOT NULL,
    quote TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version INT NOT NULL DEFAULT 1
); ";
pub const RESET: &str = "DELETE FROM quotes;";
pub const CITE: &str = "SELECT * FROM quotes WHERE id=$1";
pub const LIST: &str = "SELECT * FROM quotes ORDER BY created_at ASC LIMIT $1 OFFSET $2;";

pub  const COUNT: &str = "SELECT COUNT(*) AS count FROM quotes;";
pub const REMOVE: &str = "DELETE FROM quotes WHERE id=$1";
pub const UNDO: &str = "UPDATE quotes SET author=$1, quote=$2, version = version + 1 WHERE id=$3";
pub const DRAFT: &str = "INSERT INTO quotes (id, author, quote, created_at, version) \
    VALUES ($1, $2, $3, $4, 1);";

#[derive(Deserialize, Serialize, Debug, Clone, FromRow, Decode)]
pub struct Count {
    pub count: i64,
}