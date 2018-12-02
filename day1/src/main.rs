extern crate clap;
use clap::{Arg, App};

mod device;
use device::Device;

use std::fs::File;
use std::io::prelude::Read;

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

    let mut frequencies = String::new();
    File::open(matches.value_of("input")
                      .unwrap())
         .expect("input file not exists")
         .read_to_string(&mut frequencies)
         .expect("failed to read input file");

    let device: Device = frequencies.split("\n")
                                    .filter_map(|f| f.parse::<i32>().ok())
                                    .fold(Device::new(0), |device, frequency| device.calibrate(frequency));

    println!("Device frequency : {:?}", device.frequency());
}