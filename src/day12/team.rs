#[derive(Debug, Eq, PartialEq)]
pub enum Team {
    Cookie,
    Milk,
}

impl Team {
    pub fn symbol(&self) -> &'static str {
        match self {
            Team::Cookie => "🍪",
            Team::Milk => "🥛"
        }
    }
}

impl TryFrom<&str> for Team {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cookie" => Ok(Team::Cookie),
            "milk" => Ok(Team::Milk),
            _ => Err(()),
        }
    }
}