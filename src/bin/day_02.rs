use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    solve();
    let elapsed = now.elapsed();
    println!("took {:.2?}", elapsed);
}

fn solve() {
    let lines = lines_from_file("../input/02.txt");

    let reports = lines.iter()
        .map(|l| l.split_whitespace()
            .map(|s| s.parse::<i32>().expect("Could not convert {}"))
            .collect::<Vec<i32>>());

    let ans = reports.clone()
        .filter(|report| is_valid(report))
        .count();

    let ans2 = reports.clone()
        .filter(|report| is_valid(report) || has_valid_permutation(report))
        .count();


    println!("answer 1: {}", ans); // 390
    println!("answer 2: {}", ans2); // 439
}

fn has_valid_permutation(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let permutation = report.clone()
            .iter()
            .enumerate()
            .filter_map(
                |(ix, e)| 
                if i != ix { Some(e.clone()) } 
                else { None }
                )
            .collect::<Vec<i32>>();
        
        if is_valid(&permutation) {
            return true
        }
    }

    return false
}

fn is_valid(report: &Vec<i32>) -> bool {
    if report[0] > report[1] {
        let rev = report.clone().into_iter().rev().collect();
        return is_monotonic(&rev);
    }

    return is_monotonic(report);
}

fn is_monotonic(report: &Vec<i32>) -> bool {
    for (i, e) in report.iter().enumerate() {
        if i > 0 {
            if (e - report[i - 1]) < 1 || (e - report[i - 1]) > 3 {
                return false;
            }
        }
    }
    return true;
}
