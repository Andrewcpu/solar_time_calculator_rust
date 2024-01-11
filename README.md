# Sunrise and Sunset Calculator

This Rust application calculates the sunrise and sunset times for a given date, time, latitude, and longitude. It is particularly useful for astronomy enthusiasts, photographers, hikers, or anyone interested in observing the natural phenomena of sunrise and sunset.

## Features

- Calculates sunrise and sunset times based on the specified date, time, latitude, and longitude.
- Utilizes the fractional year in radians to determine the position of the sun.
- Implements the equation of time to account for the Earth's elliptical orbit.
- Calculates solar declination for accurate results.
- Adjusts times to the specified timezone.

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [the official Rust website](https://www.rust-lang.org/).
2. Clone this repository to your local machine.
3. Navigate to the project directory and build the project using Cargo:

```
cargo build --release
```

## Usage

To use the application, run it with the following arguments:
```
./solar <date> <time> <latitude> <longitude> [time_zone]
```


- `date`: The date for which you want to calculate sunrise and sunset times (format: YYYY-MM-DD).
- `time`: The local time to base the calculations on (format: HH:MM:SS).
- `latitude`: The latitude of the location in decimal degrees.
- `longitude`: The longitude of the location in decimal degrees.
- `timezone`: IANA timezone format, optional. Defaults to "America/New_York"

Example:

```
./solar 2024-01-11 12:00:00 40.7128 -74.0060 "America/New_York"
```

This command calculates the sunrise and sunset times for New York City on January 11, 2024, at noon.

## License
I dont care. Use it for whatever, however you please.
