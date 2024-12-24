use std::fmt::Display;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
pub enum OrnamentState {
    On,
    Off,
}

impl OrnamentState {
    pub fn flip(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

impl FromStr for OrnamentState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(()),
        }
    }
}

impl Display for OrnamentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
        }
    }
}
