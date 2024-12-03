pub fn run(input: String) {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("Part 1 {}", part_one(&reports));
    println!("Part 2 {}", part_two(reports));
}

fn part_one(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| is_safe(r)).count()
}

fn part_two(reports: Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|r| {
            if is_safe(r) {
                return true;
            }
            for i in 0..r.len() {
                let mut without_level = (*r).clone();
                without_level.remove(i);
                if is_safe(&without_level) {
                    return true;
                }
            }

            false
        })
        .count()
}

fn is_safe(report: &[i32]) -> bool {
    let increasing = report[1] > report[0];
    report.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];

        if increasing && diff <= 0 || !increasing && diff >= 0 {
            return false;
        }

        let abs = diff.abs();
        (1..=3).contains(&abs)
    })
}
