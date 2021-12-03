use crate::utils;
use std::str::FromStr;
use strum::EnumString;

#[derive(Default, Debug)]
struct Position {
    horizontal: u32,
    depth: u32,
}

#[derive(Default, Debug)]
struct Submarine {
    position: Position,
    aim: u32,
}

#[derive(EnumString)]
#[allow(non_camel_case_types)]
enum Instructions {
    up,
    down,
    forward,
}

pub fn pt1_run() {
    let lines = utils::read_lines("./src/day2/input.txt").expect("Unable to read file");
    let mut submarine_position = Position::default();
    lines.for_each(|line| {
        let line = line.expect("Unable to read line");
        let instruction_and_value: Vec<&str> = line.split(' ').collect();
        if instruction_and_value.len() != 2 {
            panic!("Instruction is not reconized")
        }
        let instructions = Instructions::from_str(instruction_and_value[0]).expect(&*format!(
            "Unalbe to parse intruction: {}",
            instruction_and_value[0]
        ));
        let value: u32 = instruction_and_value[1].parse().unwrap();
        match instructions {
            Instructions::up => {
                submarine_position.depth -= value;
            }
            Instructions::down => {
                submarine_position.depth += value;
            }
            Instructions::forward => {
                submarine_position.horizontal += value;
            }
        }
    });
    println!(
        "Day2 pt1 result: {}, submarine_position: {:#?}",
        submarine_position.depth * submarine_position.horizontal,
        submarine_position
    );
}

pub fn pt2_run() {
    let lines = utils::read_lines("./src/day2/input.txt").expect("Unable to read file");
    let mut submarine = Submarine::default();
    lines.for_each(|line| {
        let line = line.expect("Unable to read line");
        let instruction_and_value: Vec<&str> = line.split(' ').collect();
        if instruction_and_value.len() != 2 {
            panic!("Instruction is not reconized")
        }
        let instructions = Instructions::from_str(instruction_and_value[0]).expect(&*format!(
            "Unalbe to parse intruction: {}",
            instruction_and_value[0]
        ));
        let value: u32 = instruction_and_value[1].parse().unwrap();
        match instructions {
            Instructions::up => {
                submarine.aim -= value;
            }
            Instructions::down => {
                submarine.aim += value;
            }
            Instructions::forward => {
                submarine.position.horizontal += value;
                submarine.position.depth += value * submarine.aim;
            }
        }
    });
    println!(
        "Day2 pt2 result: {}, submarine_position: {:#?}",
        submarine.position.depth * submarine.position.horizontal,
        submarine.position
    );
}
