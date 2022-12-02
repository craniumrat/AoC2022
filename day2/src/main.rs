use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_scores() -> [[u32; 3]; 3] {
    let mut scores = [[0u32; 3]; 3];
    scores[0][0] = 4;
    scores[0][1] = 8;
    scores[0][2] = 3;
    scores[1][0] = 1;
    scores[1][1] = 5;
    scores[1][2] = 9;
    scores[2][0] = 7;
    scores[2][1] = 2;
    scores[2][2] = 6;

    scores
}

fn get_index(a: &str) -> usize {
    match a {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!()
    }
}

fn main() {

    let scores = get_scores();
    let mut sum = 0;

    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(pairs) = line {
                println!("{}", pairs);
                let mut parts = pairs.split(' ');
                let first = parts.next().unwrap();
                let second = parts.next().unwrap();

                let first_index = get_index(first);
                let second_index = get_index(second);
                
                println!("{}", scores[first_index][second_index]);
                sum += scores[first_index][second_index];
            }
        }
    }

    println!("{}", sum);
}
