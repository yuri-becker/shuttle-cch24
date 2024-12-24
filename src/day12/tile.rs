use crate::day12::team::Team;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Cookie,
    Milk,
}

impl Tile {
    pub fn char(&self) -> &'static str {
        match self {
            Tile::Empty => "⬛",
            Tile::Cookie => "🍪",
            Tile::Milk => "🥛",
        }
    }

    pub fn team(&self) -> Option<Team> {
        match self {
            Tile::Empty => None,
            Tile::Cookie => Some(Team::Cookie),
            Tile::Milk => Some(Team::Milk),
        }
    }
}

impl From<&Team> for Tile {
    fn from(value: &Team) -> Self {
        match value {
            Team::Cookie => Tile::Cookie,
            Team::Milk => Tile::Milk,
        }
    }
}
