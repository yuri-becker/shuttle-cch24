use crate::day12::team::Team;
use crate::day12::tile::Tile;
use rand::rngs::StdRng;
use rand::Rng;
use rocket::log::private::info;
use std::ops::Deref;

const ROWS: usize = 4;
const COLS: usize = 4;

pub struct Grid([[Tile; COLS]; ROWS]);

impl Grid {
    pub fn new() -> Grid {
        Grid([[Tile::Empty; COLS]; ROWS])
    }

    fn determine_winner(&self) -> Option<Winner> {
        for row in 0..ROWS {
            let first_in_row = self[row][0];
            let all_tiles_in_row_same = self[row][1..]
                .iter()
                .all(|other_tile| *other_tile == first_in_row);
            if all_tiles_in_row_same {
                info!("All tiles in row {} are {:?}", row, first_in_row);
                // if tile is Empty, Winner will None (at least one Empty tile means no game end)
                return first_in_row.team().map(Winner::Team);
            }
        }
        for col in 0..COLS {
            let first_in_col = self[0][col];
            let all_tiles_in_col_same = (1..ROWS)
                .map(|row| &self[row][col])
                .all(|other_tile| *other_tile == first_in_col);
            if all_tiles_in_col_same {
                info!("All tiles in col {} are {:?}", col, first_in_col);
                return first_in_col.team().map(Winner::Team);
            }
        }
        {
            let first_in_diag = self[0][0];
            let all_tiles_in_diag_same = (1..ROWS)
                .map(|i| &self[i][i])
                .all(|other_tile| *other_tile == first_in_diag);
            if all_tiles_in_diag_same {
                info!("All tiles in diag are {:?}", first_in_diag);
                return first_in_diag.team().map(Winner::Team);
            }
        }
        {
            let first_in_antidiag = self[0][COLS - 1];
            let all_tiles_in_antidiag_same = (1..ROWS)
                .map(|i| &self[i][COLS - i - 1])
                .all(|other_tile| *other_tile == first_in_antidiag);
            if all_tiles_in_antidiag_same {
                info!("All tiles in antidiag are {:?}", first_in_antidiag);
                return first_in_antidiag.team().map(Winner::Team);
            }
        }
        let empty_tiles_left = self
            .iter()
            .flat_map(|row| row.iter())
            .any(|tile| *tile == Tile::Empty);
        if empty_tiles_left {
            None
        } else {
            Some(Winner::Tie)
        }
    }

    pub fn print(&self) -> String {
        self.iter()
            .map(|row| {
                format!(
                    "⬜{}⬜",
                    row.iter().map(|cell| cell.char()).collect::<String>()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
            .to_string()
            + "\n⬜⬜⬜⬜⬜⬜\n"
            + &self
            .determine_winner()
            .map(|winner| winner.message())
            .unwrap_or("".to_string())
    }

    pub fn place(&mut self, column: usize, team: &Team) -> Result<(), PlaceError> {
        if column >= COLS {
            return Err(PlaceError::InvalidColumn);
        }

        let game_ended = self.determine_winner().is_some();
        if game_ended {
            return Err(PlaceError::GameOver);
        }

        for row in (0..ROWS).rev() {
            let cell = self[row][column];
            if cell == Tile::Empty {
                self.0[row][column] = Tile::from(team);
                return Ok(());
            }
        }
        Err(PlaceError::ColumnFull)
    }

    pub fn generate_random(rng: &mut StdRng) -> Grid {
        let mut grid = Grid::new();
        for row in 0..ROWS {
            for col in 0..COLS {
                let tile = rng.gen::<bool>();
                let tile = match tile {
                    false => Tile::Milk,
                    true => Tile::Cookie
                };
                grid.0[row][col] = tile;
            }
        }
        grid
    }
}

impl Deref for Grid {
    type Target = [[Tile; 4]; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Winner {
    Team(Team),
    Tie,
}

impl Winner {
    pub fn message(&self) -> String {
        match self {
            Winner::Team(team) => format!("{} wins!\n", team.symbol()),
            Winner::Tie => "No winner.\n".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum PlaceError {
    InvalidColumn,
    ColumnFull,
    GameOver,
}
