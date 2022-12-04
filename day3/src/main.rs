use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn priority(c: char) -> u32 {
    let mut value = 0;
    
    if c >= 'A' && c <= 'Z' {
        value = c as u32 - 'A' as u32 + 27;
    } else {
        value = c as u32 - 'a' as u32 + 1;
    }

    value
}

fn main() {
    let path = Path::new("input.txt");
    let mut sum = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(value) = line {
                let mut first_map: HashMap<char, usize> = HashMap::new();
                let mut second_map: HashMap<char, usize> = HashMap::new();

                println!("Len: {}", value.len());

                let first_string = &value[0..value.len()/2];
                let second_string = &value[value.len()/2..];

                first_string.chars().for_each(|c| *first_map.entry(c).or_insert(0) += 1);
                second_string.chars().for_each(|c| *second_map.entry(c).or_insert(0) += 1);

                //Now that we have populated both maps.. we can compare keys and find one that is 
                //also present in the other.

                for key in first_map.keys() {
                    if second_map.get(key).is_some() {
                        sum += priority(*key);
                    }
                }
            }
        }

        //Part 1
        println!("Part 1: {}", sum);
    }
}

