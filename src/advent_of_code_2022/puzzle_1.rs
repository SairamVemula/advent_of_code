
use std::{fs::File, io::BufRead, io::BufReader};

pub fn main() {
    let file = match File::open(String::from("input1.txt")) {
        Err(why) => panic!("Can't open {}", why),
        Ok(file) => file,
    };

    let mut lines: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }

    let mut elves: Vec<Vec<i32>> = Vec::new();
    let mut elve: Vec<i32> = Vec::new();

    for line in lines {
        if line.is_empty() {
            elves.push(elve);
            elve = Vec::new();
        } else {
            elve.push(line.parse::<i32>().unwrap());
        }
    }

    let mut cals: Vec<i32> = elves
        .iter()
        .map(|e| e.iter().fold(0, |acc, e| acc + e))
        .collect();
    cals.sort();
    println!("Largested {}", cals.last().unwrap());

    // PART 2
    println!(
        "Sum of largest top three {}",
        cals.get(cals.len() - 1).unwrap()
            + cals.get(cals.len() - 2).unwrap()
            + cals.get(cals.len() - 3).unwrap()
    )
}
