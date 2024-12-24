mod grid;
mod team;
mod tile;

use crate::day12::grid::Grid;
use crate::day12::grid::PlaceError;
use crate::day12::team::Team;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rocket::http::Status;
use rocket::log::private::{info, warn};
use rocket::{get, post, routes, Route, State};
use std::convert::Into;
use std::sync::RwLock;

impl From<PlaceError> for Status {
    fn from(error: PlaceError) -> Self {
        match error {
            PlaceError::InvalidColumn => Status::BadRequest,
            PlaceError::ColumnFull => Status::ServiceUnavailable,
            PlaceError::GameOver => Status::ServiceUnavailable,
        }
    }
}

pub struct Day12 {
    pub grid: RwLock<Grid>,
    pub rand: RwLock<StdRng>,
}

impl Day12 {
    pub fn new() -> Self {
        Day12 {
            grid: RwLock::new(Grid::new()),
            rand: RwLock::new(StdRng::seed_from_u64(2024)),
        }
    }
}

#[get("/board")]
async fn get_board(state: &State<Day12>) -> Result<String, Status> {
    Ok(state
        .grid
        .read()
        .map_err(|_| Status::InternalServerError)?
        .print())
}

#[post("/reset")]
async fn reset(state: &State<Day12>) -> Result<String, Status> {
    let mut grid = state
        .grid
        .write()
        .map_err(|_| Status::InternalServerError)?;
    *grid = Grid::new();
    let mut rand = state
        .rand
        .write()
        .map_err(|_| Status::InternalServerError)?;
    *rand = StdRng::seed_from_u64(2024);
    Ok(grid.print())
}

#[post("/place/<team>/<column>")]
async fn place(team: &str, column: &str, state: &State<Day12>) -> Result<String, (Status, String)> {
    let team = Team::try_from(team).map_err(|_| (Status::BadRequest, "Bad Request".to_string()))?;
    let mut grid = state.grid.write().map_err(|_| {
        warn!("Couldn't acquire write lock for Grid");
        (
            Status::InternalServerError,
            "Internal Server Error".to_string(),
        )
    })?;
    let column = column
        .parse::<usize>()
        .map_err(|_| (Status::BadRequest, "Bad Request".to_string()))?;
    if column == 0 {
        return Err((Status::BadRequest, "Bad Request".to_string()));
    }
    grid.place(column - 1, &team).map_err(|err| {
        let print = grid.print();
        info!("Placing failed due to {:?}:\n{}", err, &print);
        (Status::from(err), print)
    })?;
    let print = grid.print();
    info!("Placing succeeded.:\n{}", print);
    Ok(print)
}

#[get("/random-board")]
fn random_board(state: &State<Day12>) -> Result<String, Status> {
    let mut rand = state
        .rand
        .write()
        .map_err(|_| Status::InternalServerError)?;
    let grid = Grid::generate_random(&mut rand);
    Ok(grid.print())
}

pub fn routes() -> Vec<Route> {
    routes![get_board, reset, place, random_board]
}
