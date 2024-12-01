mod solutions;

use std::collections::HashMap;
use std::fs;

fn main() {
    let mut solutions: HashMap<u32, fn(String)> = HashMap::new();
    solutions::register_solutions(&mut solutions);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Missing day argument");
        return;
    }

    let Ok(day) = args[1].parse::<u32>() else {
        eprintln!("Invalid day");
        std::process::exit(1);
    };

    match solutions.get(&day) {
        Some(solution) => {
            let input = fs::read_to_string(format!("input/{}.txt", day))
                .expect("Failed to read input file");
            solution(input);
        },
        None => eprintln!("Solution for day {day} not implemented yet"),
    }
}
