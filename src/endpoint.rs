use std::fmt;

use crate::antenna::Antenna;

#[derive(Debug, Default, Clone)]
pub struct Endpoint {
    antennas: Vec<Antenna>,
    is_dsn: bool,
}

impl Endpoint {
    pub fn new() -> Endpoint {
        Endpoint {
            antennas: Vec::new(),
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

        if let Some(existing) = self.antennas.get(0) {
            if existing.is_dsn {
                return;
            }
        }

        for _ in 1..count {
            self.antennas.push(antenna.clone());
        }
        self.antennas.push(antenna);
    }

    pub fn antennas(&self) -> impl Iterator<Item = &Antenna> {
        self.antennas.iter()
    }

    pub fn antenna_counts(&self) -> impl Iterator<Item = (&Antenna, usize)> {
        Counts {
            antennas: self.antennas.as_slice(),
            idx: 0,
        }
    }

    pub fn endpoint_type(&self) -> &str {
        if self.is_dsn {
            "DSN"
        } else {
            "Vessel"
        }
    }

    pub fn is_empty(&self) -> bool {
        self.antennas.is_empty()
    }

    pub fn power(&self) -> f64 {
        let strongest_antenna_power = self.strongest_antenna().power;
        let sum_of_antenna_power: f64 = self.antennas.iter().map(|a| a.power).sum();

        strongest_antenna_power
            * (sum_of_antenna_power / strongest_antenna_power)
                .powf(self.average_combinability_exponent())
    }

    pub fn range_to(&self, other: &Endpoint) -> Range {
        Range {
            distance: (self.power() * other.power()).sqrt(),
        }
    }

    fn strongest_antenna(&self) -> &Antenna {
        self.antennas
            .iter()
            .max_by_key(|a| a.power as u64)
            .expect("vessel shoud have antenna.")
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

#[derive(Debug)]
pub struct Counts<'a> {
    antennas: &'a [Antenna],
    idx: usize,
}

impl<'a> Iterator for Counts<'a> {
    type Item = (&'a Antenna, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.antennas.len() {
            return None;
        }

        let antenna = &self.antennas[self.idx];
        self.idx += 1;
        let mut count = 1;

        while self.idx < self.antennas.len() {
            let a = &self.antennas[self.idx];
            if a.name != antenna.name {
                break;
            }

            self.idx += 1;
            count += 1;
        }

        Some((antenna, count))
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
