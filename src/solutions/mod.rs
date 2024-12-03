use std::collections::HashMap;

mod day1;
mod day2;

pub fn register_solutions(solutions: &mut HashMap<u32, fn(String)>) {
  solutions.insert(1, day1::run);
  solutions.insert(2, day2::run);
} 