use clap::{App, Arg, crate_name, crate_authors, crate_version};

use ksp_commnet_simulator::error::{Error, MessageError};
use ksp_commnet_simulator::antenna::{Antennas, Antenna};
use ksp_commnet_simulator::vessel::Vessel;
use ksp_commnet_simulator::distance::distances;

fn main() {
    if let Err(e) = w_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    } 
}

fn w_main() -> Result<(), Error> {
    let matches = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(Arg::with_name("from").short("f").long("from").multiple(true).takes_value(true).default_value("DSN Lv.3"))
        .arg(Arg::with_name("to").short("t").long("to").multiple(true).takes_value(true))
        .arg(Arg::with_name("antennas").long("antennas").help("Print antennas"))
        .get_matches();

    let antennas = Antennas::new();

    if matches.is_present("antennas") {
        println!("Available antennas:");
        antennas.print_all("    ");
        return Ok(());
    }

    let mut from_vessel = Vessel::new();
    for antenna_str in matches.values_of("from").unwrap_or_default() {
        let (a, n) = parse_antenna_arg(&antennas, antenna_str)?;
        from_vessel.add_antenna(a, n);
    }

    let mut to_vessel = Vessel::new();
    for antenna_str in matches.values_of("to").unwrap_or_default() {
        let (a, n) = parse_antenna_arg(&antennas, antenna_str)?;
        to_vessel.add_antenna(a, n);
    }

    println!("From:");
    from_vessel.print("    ");
    println!("To:");
    to_vessel.print("    ");
    println!();

    let range = from_vessel.range_to(&to_vessel);
    println!("Max distance: {}", range);
    println!();

    println!("|          Section          |   @Min   |   @Max   |");
    println!("|:--------------------------|---------:|---------:|");
    for d in distances() {
        println!(
            "| {:<25} | {:>8} | {:>8} |",
            d.section,
            format_strength(range.strength_at(d.min)),
            format_strength(range.strength_at(d.max)),
        );
    }

    Ok(())
}

fn parse_antenna_arg(antennas: &Antennas, s: &str) -> Result<(Antenna, usize), Error> {
    let parts: Vec<&str> = s.split(':').collect();

    match parts.len() {
        1 => {
            let a = antennas.get(parts[0]).ok_or_else(|| MessageError::new(format!("unknown antenna: {}", parts[0])))?;
            Ok((a.clone(), 1))
        }
        2 => {
            let a = antennas.get(parts[0]).ok_or_else(|| MessageError::new(format!("unknown antenna: {}", parts[0])))?;
            let n = parts[1].parse()?;
            Ok((a.clone(), n))
        }
        _ => Err(MessageError::new(format!("antenna specifier should be [<NUMBER_OF_ANTENNA>:]<ANTENNA_NAME>, but {}", s)).into()),
    }
}

fn format_strength(strength: Option<f64>) -> String {
    if let Some(s) = strength {
        format!("{:.1} %", 100.0 * s)
    } else {
        "NA".to_owned()
    }
}