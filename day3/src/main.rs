extern crate clap;

mod point;
mod rectangle;
mod claim;
mod fabric;

use std::fs;
use clap::{Arg, App};
use fabric::Fabric;
use claim::Claim;

fn main() {
    let matches = App::new("adventofcode2018_day3")
                    .about("Fabric claims")
                    .arg(Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("FILE")
                        .help("input claims file")
                        .takes_value(true))
                    .get_matches();

    let claims_raw = fs::read_to_string(matches.value_of("input").unwrap()).expect("failed to read file");
    let claims = claims_raw.split("\n").map(|claim| Claim::from(claim).expect("failed to parse claim"));

    let mut fabric = Fabric::new();
    claims.for_each(|claim| fabric.claim(claim));
    println!("Overlapping claims : {:?}", fabric.overlapping_claim_area());
}
