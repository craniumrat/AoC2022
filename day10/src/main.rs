use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct RegisterHistory {
    time_: u32,
    value: i32
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn instruction_to_register_timestamp(instruction: &str, history: &mut Vec<RegisterHistory>) -> io::Result<()> {
    lazy_static! {
        static ref ADDX_REGEX: Regex = Regex::new(r"addx (-?\d+)").unwrap(); 
    }

    let last = history.iter().last().or_else(|| Some(&RegisterHistory{time_: 1, value: 1 }) ).unwrap().clone();

    if instruction == "noop" {
        history.push(RegisterHistory {time_: last.time_ + 1, value: last.value });
        println!("{:?}: {:?}", last.time_ + 1, last.value);
        return Ok(())
    }

    let captures = ADDX_REGEX.captures(instruction).unwrap();
    let value: i32 = captures[1].parse().unwrap();

    history.push(RegisterHistory{ time_: last.time_ + 1, value: last.value });
    history.push(RegisterHistory { time_: last.time_ + 2, value: last.value + value} );

    println!("{:?}: {:?}", last.time_ + 1, last.value);
    println!("{:?}: {:?}", last.time_ + 2, last.value + value);

    Ok(())
}

fn main() -> io::Result<()> {

    let mut timestamp_registers: Vec<RegisterHistory> = vec![];

    //Part 1
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(instruction) = line {
                instruction_to_register_timestamp(&instruction, &mut timestamp_registers)?;
                // println!("{}", instruction);
            }
        }
    }

    let mut sum = 0;

    for register_history in &timestamp_registers {
        if register_history.time_ % 40 == 20 {
            println!("{}: {}. Value: {}", register_history.time_, register_history.value, register_history.value * register_history.time_ as i32);
            sum += register_history.value * register_history.time_ as i32;
            println!("{}", sum);
        }
    }

    Ok(())
}
