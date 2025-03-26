pub fn run() {
    // A report is consodered safe if
    // 1. The levels are either all increasing or all decreasing.
    // 2. Any two adjacent levels differ by at least one and at most three
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
    read_input()
        .into_iter()
        .filter(|report| is_safe(report))
        .count()
}

fn part_two() -> usize {
    read_input()
        .into_iter()
        .filter(|report| is_safe_dampened(report))
        .count()
        + 1
}

fn read_input() -> Vec<Vec<u64>> {
    let mut reports: Vec<Vec<u64>> = Vec::new();
    for line in include_str!("../input/day2").lines() {
        let numbers = line.split(' ').map(|l| l.parse().unwrap()).collect();
        reports.push(numbers);
    }

    reports
}

fn is_safe(report: &[u64]) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let order = report[0].cmp(&report[1]);
    for window in report.windows(2) {
        let curr = window.first().unwrap();
        let next = window.get(1).unwrap();

        if !(1..=3).contains(&curr.abs_diff(*next)) {
            return false;
        }

        if curr.cmp(next) != order {
            return false;
        }
    }

    true
}

fn is_safe_dampened(report: &[u64]) -> bool {
    if is_safe(&report[1..]) {
        return true;
    }

    let order = report[0].cmp(&report[2]);
    let mut errors = 0;
    let mut prev = report[0];
    for &curr in report.iter().skip(1) {
        if curr == prev || curr.abs_diff(prev) > 3 {
            errors += 1;
            continue;
        }

        if prev.cmp(&curr) != order {
            errors += 1;
            continue;
        }

        prev = curr;
    }

    errors <= 1
}
