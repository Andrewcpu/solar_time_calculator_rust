use std::env;
use chrono::{NaiveDate, DateTime, Utc, Datelike, NaiveTime, Timelike};
use std::f64::consts::PI;
use chrono_tz::Tz;

// Function to calculate the fractional year in radians (γ)
fn fractional_year(date: NaiveDate, time: NaiveTime, is_leap_year: bool) -> f64 {
    let day_of_year = date.ordinal0() as f64;
    let fraction_of_day = (time.num_seconds_from_midnight() as f64) / (24.0 * 60.0 * 60.0); // Fraction of the day
    let denominator = if is_leap_year { 366.0 } else { 365.0 };
    2.0 * PI / denominator * (day_of_year + 1.0 + fraction_of_day)
}
// Function to calculate the equation of time (eqtime)
fn equation_of_time(gamma: f64) -> f64 {
    229.18 * (0.000075 + 0.001868 * gamma.cos() - 0.032077 * gamma.sin()
        - 0.014615 * (2.0 * gamma).cos() - 0.040849 * (2.0 * gamma).sin())
}

// Function to calculate the solar declination (decl)
fn solar_declination(gamma: f64) -> f64 {
    0.006918 - 0.399912 * gamma.cos() + 0.070257 * gamma.sin()
        - 0.006758 * (2.0 * gamma).cos() + 0.000907 * (2.0 * gamma).sin()
        - 0.002697 * (3.0 * gamma).cos() + 0.00148 * (3.0 * gamma).sin()
}

// [Previous functions: fractional_year, equation_of_time, solar_declination]
// Function to calculate sunrise and sunset times
fn sunrise_sunset(date_str: &str, time_str: &str, latitude: f64, longitude: f64) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let time = NaiveTime::parse_from_str(time_str, "%H:%M:%S").unwrap();
    let is_leap_year = date.year() % 4 == 0 && (date.year() % 100 != 0 || date.year() % 400 == 0);
    let gamma = fractional_year(date, time, is_leap_year);
    let eqtime = equation_of_time(gamma);
    let decl = solar_declination(gamma);
    let lat_rad = latitude.to_radians();
    let zenith = 90.833_f64.to_radians(); // Solar zenith angle for sunrise/sunset
    let cos_hour_angle = (zenith.cos() - (decl.sin() * lat_rad.sin())) / (decl.cos() * lat_rad.cos());
    let hour_angle = cos_hour_angle.acos();

    // Convert hour angle to hours
    let hour_angle_hours = hour_angle * (180.0 / PI) / 15.0;

    // Calculate sunrise and sunset times in minutes from midnight
    let hour_angle_hours_60: f64 = hour_angle_hours * 60.0;
    let pre_calc = 720.0 - 4.0 * longitude - eqtime;
    let sunrise_minutes = pre_calc - hour_angle_hours_60;
    let sunset_minutes = pre_calc + hour_angle_hours_60;

    // Convert minutes to hours and minutes
    let sunrise_hours = (sunrise_minutes / 60.0).floor() as u32;
    let sunrise_mins = (sunrise_minutes % 60.0) as u32;
    let sunset_hours = (sunset_minutes / 60.0).floor() as u32;
    let sunset_mins = (sunset_minutes % 60.0) as u32;
    let sunrise_time = date.and_hms_opt(sunrise_hours, sunrise_mins, 0)
        .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
    let sunset_time = date.and_hms_opt(sunset_hours, sunset_mins, 0)

        .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));

    // Check if both sunrise and sunset times are valid
    match (sunrise_time, sunset_time) {
        (Some(sunrise), Some(sunset)) => Some((sunrise, sunset)),
        _ => None // Return None if either time is invalid
    }
}

// Main function to demonstrate the usage
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("{}", args[0]);
        println!("Usage: program <date> <time> <latitude> <longitude>");
        return;
    }

    let date_str = &args[1];
    let time_str = &args[2];
    let latitude: f64 = args[3].parse().expect("Invalid latitude");
    let longitude: f64 = args[4].parse().expect("Invalid longitude");

    let tz: Tz = "America/New_York".parse().unwrap(); // Change as needed

    match sunrise_sunset(date_str, time_str, latitude, longitude) {
        Some((sunrise_time, sunset_time)) => {
            println!("▲ {}   ▼ {}", sunrise_time.with_timezone(&tz), sunset_time.with_timezone(&tz));
        }
        None => {
            println!("Something went wrong while calculating the times.");
        }
    }
}

