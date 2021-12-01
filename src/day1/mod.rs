use crate::utils;

pub fn run() {
    let lines = utils::read_lines("./src/day1/input.txt").expect("Unable to read file");
    let mut depth_increases = 0;
    let mut previous_measurement = u64::max_value();
    lines.for_each(|line| {
        if let Ok(measurement_str) = line {
            let measurement: u64 = measurement_str.parse().expect("Unable to parse value");
            if measurement > previous_measurement {
                depth_increases += 1
            }
            previous_measurement = measurement
        }
    });
    println!("Day1 result: {}", depth_increases)
}
