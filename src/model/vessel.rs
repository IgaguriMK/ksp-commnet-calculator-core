use std::collections::BTreeMap;
use std::fmt;

use crate::model::antenna::Antenna;

#[derive(Debug, Default, Clone)]
pub struct Vessel {
    antennas: Vec<Antenna>,
    is_dsn: bool,
}

impl Vessel {
    pub fn new() -> Vessel {
        Vessel {
            antennas: vec![Antenna::command_module()],
            is_dsn: false,
        }
    }

    pub fn add_antenna(&mut self, antenna: Antenna, count: usize) {
        if self.is_dsn {
            return;
        }

        if antenna.is_dsn {
            self.is_dsn = true;
            self.antennas = vec![antenna];
            return;
        }

        for _ in 1..count {
            self.antennas.push(antenna.clone());
        }
        self.antennas.push(antenna);
    }

    pub fn info(&self) -> EndpointInfo {
        let endpoint_type = if self.is_dsn { "DSN" } else { "Vessel" };

        let mut counts = BTreeMap::<String, (usize, Antenna)>::new();
        for a in &self.antennas {
            counts
                .entry(a.name.clone())
                .and_modify(|(c, _)| *c += 1)
                .or_insert((1, a.clone()));
        }

        let antennas = counts.into_iter().map(|v| v.1).collect();

        EndpointInfo {
            endpoint_type,
            antennas,
        }
    }

    pub fn range_to(&self, other: &Vessel) -> Range {
        Range {
            distance: (self.power() * other.power()).sqrt(),
        }
    }

    pub fn power(&self) -> f64 {
        let strongest_antenna_power = self.strongest_antenna().power;
        let sum_of_antenna_power: f64 = self.antennas.iter().map(|a| a.power).sum();

        strongest_antenna_power
            * (sum_of_antenna_power / strongest_antenna_power)
                .powf(self.average_combinability_exponent())
    }

    fn strongest_antenna(&self) -> &Antenna {
        self.antennas
            .iter()
            .max_by_key(|a| a.power as u64)
            .expect("vessel shoud have default command module antenna.")
    }

    fn average_combinability_exponent(&self) -> f64 {
        let mut u = 0f64;
        let mut d = 0f64;
        for a in &self.antennas {
            u += a.power * a.combine_exp;
            d += a.power;
        }
        u / d
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    distance: f64,
}

impl Range {
    pub fn max_distance(self) -> f64 {
        self.distance
    }

    pub fn strength_at(self, dist: f64) -> Option<f64> {
        let r = 1.0 - dist / self.distance;
        if r <= 0.0 {
            None
        } else if r > 1.0 {
            Some(1.0)
        } else {
            Some(-2.0 * r.powi(3) + 3.0 * r.powi(2))
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = self.distance;
        if d >= 1_000_000_000.0 {
            write!(f, "{:.2} Gm", d / 1_000_000_000.0)
        } else if d >= 1_000_000.0 {
            write!(f, "{:.2} Mm", d / 1_000_000.0)
        } else if d >= 1_000.0 {
            write!(f, "{:.2} km", d / 1_000.0)
        } else {
            write!(f, "{:.2} m", d)
        }
    }
}

#[derive(Debug, Clone)]
pub struct EndpointInfo {
    pub endpoint_type: &'static str,
    pub antennas: Vec<(usize, Antenna)>,
}
