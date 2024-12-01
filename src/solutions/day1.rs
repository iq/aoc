use std::collections::HashMap;

pub fn run(input: String) {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut spl = line.split_whitespace();
            let left = spl.next().unwrap().parse::<i32>().unwrap();
            let right = spl.next().unwrap().parse::<i32>().unwrap();
            (left, right)
        })
        .unzip();

    println!("Part 1 {}", part_one(&mut left, &mut right));
    println!("Part 2 {}", part_two(left, right));
}

fn part_one(left: &mut [i32], right: &mut [i32]) -> i32 {
    left.sort();
    right.sort();

    left.iter_mut()
        .zip(right)
        .map(|(k, v)| (*k - *v).abs())
        .sum()
}

fn part_two(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut right_pairs: HashMap<i32, i32> = HashMap::new();

    for r in right {
        *right_pairs.entry(r).or_insert(0) += 1;
    }

    left.iter()
        .map(|l| right_pairs.get(l).unwrap_or(&0) * l)
        .sum()
}
