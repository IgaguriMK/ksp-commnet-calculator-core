use std::collections::BTreeMap;
use std::io::Read;

use serde::Deserialize;
use serde_yaml::from_reader;

#[derive(Debug, Default, Clone)]
pub struct Antennas {
    aliaces: BTreeMap<String, String>,
    dict: BTreeMap<String, Antenna>,
}

impl Antennas {
    pub fn new() -> Antennas {
        let mut res = Antennas {
            aliaces: BTreeMap::new(),
            dict: BTreeMap::new(),
        };
        res.load(&include_bytes!("../resources/antennas.yaml")[..])
            .unwrap();
        res
    }

    pub fn load<R: Read>(&mut self, r: R) -> Result<(), Error> {
        let antennas: Vec<Antenna> = from_reader(r)?;
        for a in antennas {
            for alias in &a.aliases {
                self.aliaces.insert(alias.clone(), a.name.clone());
            }
            let n = a.name.clone();
            self.aliaces.insert(n.clone(), n.clone());
            self.dict.insert(n, a);
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Antenna> {
        if let Some(name) = self.aliaces.get(name) {
            self.dict.get(name)
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Antenna> {
        self.dict.values()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Antenna {
    pub name: String,
    pub aliases: Vec<String>,
    pub power: f64,
    pub combine: bool,
    #[serde(default = "default_combine_exp")]
    pub combine_exp: f64,
    pub relay: bool,
    #[serde(default)]
    pub is_dsn: bool,
}

fn default_combine_exp() -> f64 {
    0.75
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to decode anntenas file: {0}")]
    Decode(#[from] serde_yaml::Error),
}
