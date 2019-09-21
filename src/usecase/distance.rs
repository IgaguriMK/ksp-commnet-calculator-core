use crate::error::{Error, MessageError};
use crate::model::antenna::Antennas;
use crate::model::distance::distances;
use crate::model::vessel::{EndpointInfo, Vessel};

#[derive(Debug, Default, Clone)]
pub struct Runner {
    antennas: Antennas,
    from_vessel: Vessel,
    to_vessel: Vessel,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            antennas: Antennas::new(),
            from_vessel: Vessel::new(),
            to_vessel: Vessel::new(),
        }
    }

    pub fn add_from_vessel_antenna(
        &mut self,
        count: usize,
        antenna_name: &str,
    ) -> Result<(), Error> {
        let antenna = self
            .antennas
            .get(antenna_name)
            .cloned()
            .ok_or_else(|| MessageError::new(format!("unknown antenna: {}", antenna_name)))?;
        self.from_vessel.add_antenna(antenna, count);
        Ok(())
    }

    pub fn add_to_vessel_antenna(&mut self, count: usize, antenna_name: &str) -> Result<(), Error> {
        let antenna = self
            .antennas
            .get(antenna_name)
            .cloned()
            .ok_or_else(|| MessageError::new(format!("unknown antenna: {}", antenna_name)))?;
        self.to_vessel.add_antenna(antenna, count);
        Ok(())
    }

    pub fn run(&self) -> Result<Output, Error> {
        let endpoints = Endpoints {
            from: self.from_vessel.info(),
            to: self.to_vessel.info(),
        };

        let range = self.from_vessel.range_to(&self.to_vessel);
        let max_distance = range.max_distance();

        let distances = distances();
        let mut signal_strengthes = Vec::with_capacity(distances.len());
        for d in distances {
            signal_strengthes.push(SignalStrength {
                section: d.section,
                at_min: range.strength_at(d.min),
                at_max: range.strength_at(d.max),
            });
        }

        Ok(Output {
            endpoints,
            max_distance,
            signal_strengthes,
        })
    }

    pub fn antenna_list(&self) {
        println!("Available antennas:");
        self.antennas.print_all("    ");
    }
}

pub struct Output {
    pub endpoints: Endpoints,
    pub max_distance: f64,
    pub signal_strengthes: Vec<SignalStrength>,
}

pub struct Endpoints {
    pub from: EndpointInfo,
    pub to: EndpointInfo,
}

pub struct SignalStrength {
    pub section: String,
    pub at_min: Option<f64>,
    pub at_max: Option<f64>,
}
