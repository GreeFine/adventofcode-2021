use crate::utils;

pub fn run() {
    let lines = utils::read_lines("./src/day1/input.txt").expect("Unable to read file");
    let mut sliding_windows: Vec<u32> = Vec::new();
    lines.enumerate().for_each(|(line_index, line)| {
        let measurement_str = line.expect("Error reading line");
        let measurement: u32 = measurement_str.parse().expect("Unable to parse value");
        sliding_windows.push(0);
        for window_index in 0..3 {
            if window_index > line_index {
                continue;
            };
            let last_index = sliding_windows.len() - 1;
            sliding_windows[last_index - window_index] += measurement;
        }
    });

    let mut last_measurement = u32::max_value();
    let mut depth_increases: u32 = 0;
    for &measurement in &sliding_windows[..sliding_windows.len() - 2] {
        if last_measurement < measurement {
            depth_increases += 1;
        }
        last_measurement = measurement
    }
    println!("Day2 result: {}", depth_increases);
}