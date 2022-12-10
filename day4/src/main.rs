use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Pairs {
    start1: u32,
    end1: u32,
    start2: u32,
    end2: u32
}

fn parse_pairs(line: &str) -> Pairs {
    lazy_static! {
        static ref PAIRS_REGEX: Regex = Regex::new(r"([0-9]+)\-([0-9]+),([0-9]+)\-([0-9]+)").unwrap(); 
    }

    let captures = PAIRS_REGEX.captures(line).unwrap();

    Pairs{start1: str::parse(&captures[1]).unwrap(), end1: str::parse(&captures[2]).unwrap(), start2: str::parse(&captures[3]).unwrap(), end2: str::parse(&captures[4]).unwrap() }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut sum = 0;

    //Part 1
    // if let Ok(lines) = read_lines("input.txt") {
    //     for line in lines {
    //         if let Ok(pairs) = line {
    //             let pair = parse_pairs(&pairs);
    //             if (pair.start1 >= pair.start2 && pair.end1 <= pair.end2) ||
    //                  (pair.start2 >= pair.start1 && pair.end2 <= pair.end1) {
    //                     sum += 1;
    //             }
    //         }
    //     }
    // }

    // //Part 1
    // println!("{}", sum);

    //Part 2
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(pairs) = line {
                let pair = parse_pairs(&pairs);
                if pair.start1 <= pair.end2 && pair.end1 >= pair.start2 {
                        sum += 1;
                }
            }
        }
    }

    //Part 2
    println!("{}", sum);

}
