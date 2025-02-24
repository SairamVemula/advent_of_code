use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> i32 {
    let file = File::open("input4.txt").unwrap();
    let lines: Vec<((usize, usize), (usize, usize))> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut split = line.split(',').map(|part| {
                let mut split = part.split('-');
                let start: usize = split.next().unwrap().parse().unwrap();
                let end: usize = split.next().unwrap().parse().unwrap();
                (start, end)
            });
            let first = split.next().unwrap();
            let second = split.next().unwrap();
            (first, second)
        })
        .collect();

    let mut count = 0;

    for (first, second) in lines {
        // if (first.0 >= second.0 && first.1 <= second.1) || (second.0 >= first.0 && second.1 <= first.1){
        //     count+=1;
        // }
        let first_range = first.0..=first.1;
        let second_range = second.0..=second.1;
        // println!("(({},{}),({},{}))",first.0,first.1,second.0,second.1);
        if (second_range.contains(&first.0) || second_range.contains(&first.0))
            || (first_range.contains(&second.0) || first_range.contains(&second.0))
        {
            count += 1;
        }
    }
    println!("match = {}", count);
    return count;
}
