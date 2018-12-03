extern crate clap;
use clap::{Arg, App};

mod device;
use device::Device;

use std::fs;

fn main() {

    let matches = App::new("adventofcode2018_day1")
                          .about("Device calibration")
                          .arg(Arg::with_name("input")
                               .short("i")
                               .long("input")
                               .value_name("FILE")
                               .help("input calibration file")
                               .takes_value(true))
                          .get_matches();

    let frequencies_raw = fs::read_to_string(matches.value_of("input").unwrap()).expect("failed to read file");
    let frequencies = frequencies_raw.split("\n")
                                     .filter_map(|f| f.parse::<i32>().ok());

     let mut device = Device::new();
     while device.first_reached_twice().is_none() {
         frequencies.clone().for_each(|frequency| device.calibrate(frequency));
     }

    println!("Device frequency : {:?}", device.frequency());
    println!("Device frequency reached twice: {:?}", device.first_reached_twice());
}