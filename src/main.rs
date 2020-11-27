extern crate astro;
extern crate chrono;
extern crate structopt;

use std::io;
use structopt::StructOpt;
use astro::time::*;
use astro::planet::*;
use astro::pluto::heliocent_pos;
use chrono::{Datelike, Timelike, Utc};

#[derive(StructOpt)]
struct Cli {
    // Input string is a planet
    planet: String,
}

fn main() {
    

    // Get command line argument
    let args = Cli::from_args();

    // Use Chrono to get current date and time in UTC
    let now = Utc::now();

    // Use 'now' to establish the structures necessary for our 'astro' library
    // Cast all the data to the required type.
    let day_of_month = DayOfMonth{
        day:now.day() as u8,
        hr: now.hour() as u8,
        min: now.minute() as u8,
        sec: now.second() as f64,
        time_zone: 0.0

    };

    let my_date = Date {
        year:now.year() as i16,
        month:now.month() as u8,
        decimal_day:decimal_day(&day_of_month),
        cal_type:CalType::Gregorian
    };


    // Use our Date struct to get our Julian Day that we need
    let julian_day: f64 = julian_day(&my_date);

    // User option will be stored here
    let mut option = String::new();

    // Create a planet variable using the `Planet` enum
    let planet: Planet;

    // Read user option into 'option'
    io::stdin().read_line(&mut option).expect("Failed to read in option");
   
    // Make it lowercase :)
    args.planet.to_ascii_lowercase(); 

    // Check which options was chosen
    if args.planet == "mercury" {
        planet = Planet::Mercury;
    } else if args.planet == "venus" {
        planet = Planet::Venus;
    } else if args.planet == "earth"{
        planet = Planet::Earth;
    } else if args.planet == "mars" {
        planet = Planet::Mars;
    } else if args.planet == "jupiter" {
        planet = Planet::Jupiter;
    } else if args.planet == "saturn" {
        planet = Planet::Saturn;
    } else if args.planet == "uranus" {
        planet = Planet::Uranus;
    } else if args.planet == "neptune" {
        planet = Planet::Neptune;
    } else if args.planet == "pluto"{
        let plut_coords = heliocent_pos(julian_day);
        println!("You have discovered the 9th Planet, Pluto!\n");println!("\tLongitude: {}", plut_coords.0);
        println!("\tLatitude: {}", plut_coords.1);
        println!("\tDistance: {} Astronomical Unit(s)   # 1 AU = 9.296e+7 Miles", plut_coords.2);
        return;
    }else {
        println!("Not a valid planet, bruh.");
        return;
    }
    
    // Get a tuple which stores our heliocentric coordinates of the user specified planet
    let coords = heliocent_coords(&planet, julian_day);

    // Print the data
    println!("On {}/{}/{} at  {}:{}.{} Utc  {}, in relation to the sun,  was at...\n", my_date.month, day_of_month.day, my_date.year, day_of_month.hr, day_of_month.min, day_of_month.sec, args.planet);
    println!("\tLongitude: {}", coords.0);
    println!("\tLatitude: {}", coords.1);
    println!("\tDistance: {} Astronomical Unit(s)   # 1 AU = 9.296e+7 Miles", coords.2);
}
