use std::{collections::VecDeque, fs::File, io::Read};
use regex::Regex;

#[derive(Debug)]
struct Movement {
    elements: usize,
    from: usize,
    to: usize
}

pub fn run() {
    let mut file = File::open("input5.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let chunks: Vec<&str> = content.split("\n\n").collect();
    let _stacks = 
    chunks[0].split("\n").map(|line| 
        line.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>().trim().to_string())
        .collect::<Vec<String>>()
    )
    .collect::<Vec<Vec<String>>>();
    // .last().unwrap();
    let mut stacks = vec![VecDeque::new(); _stacks.last().unwrap().len()];

    for (index,els) in _stacks.iter().enumerate(){
        if index == (_stacks.len() - 1){
            break;
        }
        for (index,el) in els.iter().enumerate() {
            if el != "" {
                stacks[index].push_front(el.chars().nth(1).unwrap());
            }
        }
    }
    
    println!("{:?}",stacks);
    
    let movements  = chunks[1].split("\n").map(|line|{
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let elements: usize = caps[1].parse().unwrap();
        let from: usize = caps[2].parse().unwrap();
        let to: usize = caps[3].parse().unwrap();
        return Movement {
            elements,
            from,
            to,
        };
    }).collect::<Vec<Movement>>();
    // println!("{:?}",movements);
    
    for movement in movements{
        // Part 1
        // let mut turns = 0;
        // while turns < movement.elements {
        //     let el = stacks[movement.from - 1].pop_back().unwrap();
        //     stacks[movement.to -1].push_back(el);
        //     turns+=1;
        // }

        // Part 2
        let stack = stacks[movement.from - 1].clone();
        // let mut stack_to = stacks[movement.to - 1].clone();
        stacks[movement.from - 1] = VecDeque::from(stack.as_slices().0[..stack.len() - movement.elements].to_vec());
        let mut slice = VecDeque::from(stack.as_slices().0[stack.len() - movement.elements..].to_vec());
        stacks[movement.to - 1].append(&mut slice);
        // println!("{:?}",slice);
        // println!("{:?}",stacks[movement.from - 1]);
        // break;
    }
    println!("{:?}",stacks);
    println!("{:?}",stacks.iter().map(|stack| {
        stack.iter().last().unwrap()
    }).collect::<String>());

}
