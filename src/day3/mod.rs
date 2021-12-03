use crate::utils;

const NUMBER_OF_BITS: usize = 12;

pub fn pt1_run() {
    let file_content: Vec<String> = utils::get_file_content("./src/day3/input.txt");
    let mut true_bits: [u32; NUMBER_OF_BITS] = [0; NUMBER_OF_BITS];
    file_content.iter().for_each(|line| {
        let value: u32 = u32::from_str_radix(line, 2).expect("Unable to parse binary value");
        for (index, bit) in true_bits.iter_mut().enumerate() {
            if value & (1 << index as u32) > 0 {
                *bit += 1;
            }
        }
    });
    let half_file_content_size = file_content.len() / 2;
    let most_common_bit: u32 = true_bits
        .iter()
        .enumerate()
        .map(|(index, val)| {
            if *val > half_file_content_size as u32 {
                1 << index
            } else {
                0
            }
        })
        .sum();
    let least_common_bit = ((!most_common_bit) << (32 - NUMBER_OF_BITS)) >> (32 - NUMBER_OF_BITS);
    println!(
        "Day3 pt1 result: {}, most_common_bit:{} least_common_bit:{}",
        most_common_bit * least_common_bit,
        most_common_bit,
        least_common_bit
    );
}
