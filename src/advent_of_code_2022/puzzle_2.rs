use std::{fs::File, io::BufRead, io::BufReader,collections::HashMap};


enum Points {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

enum SignPoints {
    X = 1,
    Y = 2,
    Z = 3,
}


pub fn main() {

let permutations  = HashMap::from([
    ("A X".to_string(), (Points::Draw as i32) + (SignPoints::X as i32)),
    ("A Y".to_string(), (Points::Win as i32) + (SignPoints::Y as i32)),
    ("A Z".to_string(), (Points::Loose as i32) + (SignPoints::Z as i32)),
    ("B X".to_string(), (Points::Loose  as i32)+ (SignPoints::X as i32)),
    ("B Y".to_string(), (Points::Draw  as i32)+ (SignPoints::Y as i32)),
    ("B Z".to_string(), (Points::Win as i32) + (SignPoints::Z as i32)),
    ("C X".to_string(), (Points::Win as i32) + (SignPoints::X as i32)),
    ("C Y".to_string(), (Points::Loose as i32) + (SignPoints::Y as i32)),
    ("C Z".to_string(), (Points::Draw as i32) + (SignPoints::Z as i32)),
]);

let permutations2  = HashMap::from([
    ("A X".to_string(), (Points::Loose as i32) + (SignPoints::Z as i32)),
    ("A Y".to_string(), (Points::Draw as i32) + (SignPoints::X as i32)),
    ("A Z".to_string(), (Points::Win as i32) + (SignPoints::Y as i32)),
    ("B X".to_string(), (Points::Loose  as i32)+ (SignPoints::X as i32)),
    ("B Y".to_string(), (Points::Draw  as i32)+ (SignPoints::Y as i32)),
    ("B Z".to_string(), (Points::Win as i32) + (SignPoints::Z as i32)),
    ("C X".to_string(), (Points::Loose as i32) + (SignPoints::Y as i32)),
    ("C Y".to_string(), (Points::Draw as i32) + (SignPoints::Z as i32)),
    ("C Z".to_string(), (Points::Win as i32) + (SignPoints::X as i32)),
]);


    let file = match File::open("input2.txt".to_string()) {
        Err(e) => panic!("file can't be opened : {}", e),
        Ok(file) => file,
    };

    let mut total_points = 0;
    let mut total_points2 = 0;

    let reader = BufReader::new(file);

    for line in reader.lines(){
        match line {
            Ok(line) =>{ 
                match permutations.get(&line){
                None => total_points += 0,
                Some(value) =>{
                     total_points += value;
                    },
            }
                match permutations2.get(&line){
                None => total_points2 += 0,
                Some(value) =>{
                     total_points2 += value;
                    },
            }
        },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }

    println!("Total Points : {}",total_points);
    println!("Total Points 2 : {}",total_points2);
}
