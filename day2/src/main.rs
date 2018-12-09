extern crate clap;

mod package;
mod warehouse;

use std::fs;
use clap::{Arg, App};
use package::Package;
use warehouse::Warehouse;

fn main() {
    let matches = App::new("adventofcode2018_day2")
                    .about("Warehouse checksum")
                    .arg(Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("FILE")
                        .help("input packages file")
                        .takes_value(true))
                    .get_matches();

    let packages_raw = fs::read_to_string(matches.value_of("input").unwrap()).expect("failed to read file");
    let packages = packages_raw.split("\n").map(|id| Package::new(id.to_string()));
    let warehouse = Warehouse::new(packages.collect());
    println!("Checksum : {:?}", warehouse.checksum());
    println!("Prototypes : {:?}", warehouse.prototypes());
}
