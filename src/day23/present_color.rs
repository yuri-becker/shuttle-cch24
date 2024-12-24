use std::fmt::Display;
use std::str::FromStr;

pub enum PresentColor {
    Red,
    Blue,
    Purple,
}

impl PresentColor {
    pub fn next(&self) -> Self {
        match self {
            Self::Red => Self::Blue,
            Self::Blue => Self::Purple,
            Self::Purple => Self::Red,
        }
    }
}

impl FromStr for PresentColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(PresentColor::Red),
            "blue" => Ok(PresentColor::Blue),
            "purple" => Ok(PresentColor::Purple),
            _ => Err(()),
        }
    }
}

impl Display for PresentColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PresentColor::Red => f.write_str("red"),
            PresentColor::Blue => f.write_str("blue"),
            PresentColor::Purple => f.write_str("purple"),
        }
    }
}
