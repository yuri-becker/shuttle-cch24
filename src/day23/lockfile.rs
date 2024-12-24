use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub source: Option<String>,
    pub checksum: Option<String>,
    pub dependencies: Option<Vec<String>>,
}

impl Package {
    pub fn color(&self) -> Result<Option<String>, &str> {
        self.checksum
            .clone()
            .map(|checksum| {
                checksum
                    .get(0..6)
                    .map(|slice| slice.to_owned())
                    .filter(|slice| slice.chars().all(|c| c.is_ascii_hexdigit()))
                    .ok_or("Checksum does not have 3 hexdigit bytes.")
            })
            .transpose()
            .map(|option| option.map(|it| format!("#{}", it)))
    }
    pub fn top(&self) -> Result<Option<String>, &str> {
        self.checksum
            .clone()
            .map(|checksum| {
                checksum
                    .get(6..8)
                    .map(|slice| slice.to_owned())
                    .ok_or("Checksum does not have 4 bytes.")
            })
            .transpose()
            .and_then(|option| {
                option
                    .map(|it| {
                        u16::from_str_radix(it.as_str(), 16).map_err(|err| {
                            warn!("Could not parse bytes for top: {}", err);
                            "Could not parse bytes for top."
                        })
                    })
                    .transpose()
            })
            .map(|option| option.map(|it| it.to_string()))
    }
    pub fn left(&self) -> Result<Option<String>, &str> {
        self.checksum
            .clone()
            .map(|checksum| {
                checksum
                    .get(8..10)
                    .map(|slice| slice.to_owned())
                    .ok_or("Checksum does not have 5 bytes.")
            })
            .transpose()
            .and_then(|option| {
                option
                    .map(|it| {
                        u16::from_str_radix(it.as_str(), 16).map_err(|err| {
                            warn!("Could not parse bytes for left: {}", err);
                            "Could not parse bytes for left."
                        })
                    })
                    .transpose()
            })
            .map(|option| option.map(|it| it.to_string()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lockfile {
    pub version: Option<u32>,
    pub package: Vec<Package>,
}
