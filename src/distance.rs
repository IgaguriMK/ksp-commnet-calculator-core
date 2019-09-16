use serde::Deserialize;
use serde_yaml::from_reader;

pub fn distances() -> Vec<Distance> {
    from_reader(&include_bytes!("../resources/distances.yaml")[..]).unwrap()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Distance {
    pub section: String,
    pub min: f64,
    pub max: f64,
}