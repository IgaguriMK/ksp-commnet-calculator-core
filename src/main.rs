use clap::{crate_authors, crate_name, crate_version, App, Arg};

use ksp_commnet_calculator::error::{Error, MessageError};
use ksp_commnet_calculator::model::vessel::EndpointInfo;
use ksp_commnet_calculator::usecase::distance::{Output, Runner};
use ksp_commnet_calculator::util::MetricPrefix;

const INDENT: &str = "    ";

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
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .multiple(true)
                .takes_value(true)
                .default_value("DSN Lv.3"),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("antennas")
                .short("A")
                .long("antennas")
                .help("Print antennas"),
        )
        .get_matches();

    let mut runner = Runner::new();

    if matches.is_present("antennas") {
        runner.antenna_list();
        return Ok(());
    }

    for antenna_str in matches.values_of("from").unwrap_or_default() {
        let (c, n) = split_antenna_arg(antenna_str)?;
        runner.add_from_vessel_antenna(c, n)?;
    }

    for antenna_str in matches.values_of("to").unwrap_or_default() {
        let (c, n) = split_antenna_arg(antenna_str)?;
        runner.add_to_vessel_antenna(c, n)?;
    }

    let output = runner.run()?;
    print_res(&output);

    Ok(())
}

fn split_antenna_arg(s: &str) -> Result<(usize, &str), Error> {
    let parts: Vec<&str> = s.split(':').collect();

    match parts.len() {
        1 => Ok((1, parts[0])),
        2 => {
            let n = parts[1].parse()?;
            Ok((n, parts[0]))
        }
        _ => Err(MessageError::new(format!(
            "antenna specifier should be [<NUMBER_OF_ANTENNA>:]<ANTENNA_NAME>, but {}",
            s
        ))
        .into()),
    }
}

fn print_res(res: &Output) {
    println!("From:");
    print_endpoint(&res.endpoints.from);
    println!("To:");
    print_endpoint(&res.endpoints.to);
    println!();

    println!("Max distance: {}m", MetricPrefix(res.max_distance));
    println!();

    println!("|          Section          |   @Min   |   @Max   |");
    println!("|:--------------------------|---------:|---------:|");
    for strength in &res.signal_strengthes {
        println!(
            "| {:<25} | {:>8} | {:>8} |",
            strength.section,
            format_strength(strength.at_min),
            format_strength(strength.at_max),
        );
    }
}

fn print_endpoint(endpoint: &EndpointInfo) {
    println!("{}:", endpoint.endpoint_type);

    for (c, a) in &endpoint.antennas {
        if *c == 1 {
            println!("{}{}{}", INDENT, INDENT, a.name);
        } else {
            println!("{}{}{}x {}", INDENT, INDENT, *c, a.name);
        }
    }
}

fn format_strength(strength: Option<f64>) -> String {
    if let Some(s) = strength {
        format!("{:.1} %", 100.0 * s)
    } else {
        "NA".to_owned()
    }
}
