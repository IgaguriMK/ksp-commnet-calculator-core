use serde::Deserialize;
use serde_yaml::from_reader;

use crate::endpoint::Range;

#[derive(Debug, Clone, Deserialize)]
pub struct Distance {
    pub section: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct Distances {
    dists: Vec<Distance>,
}

impl Distances {
    pub fn new() -> Distances {
        let dists = from_reader(&include_bytes!("../resources/distances.yaml")[..]).unwrap();
        Distances { dists }
    }

    pub fn get_strengthes(&self, range: Range) -> Vec<Strength> {
        self.dists
            .iter()
            .map(|d| Strength {
                section: d.section.as_str(),
                at_min: range.strength_at(d.min),
                at_max: range.strength_at(d.max),
            })
            .collect()
    }
}

impl Default for Distances {
    fn default() -> Distances {
        Distances::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Strength<'a> {
    pub section: &'a str,
    pub at_min: Option<f64>,
    pub at_max: Option<f64>,
}
