use std::collections::HashMap;

pub mod day1;

pub fn register_solutions(solutions: &mut HashMap<u32, fn(String)>) {
  solutions.insert(1, day1::run);
} 