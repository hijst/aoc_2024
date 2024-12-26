use std::{
    collections::{HashMap, VecDeque},
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
    let nums: Vec<i64> = lines_from_file("../input/t24.txt")
        .into_iter()
        .map(|a| a.parse::<i64>().unwrap())
        .collect();

    println!("answer 1: {}", 1); // 17724064040
    println!("answer 2: {}", 2); // 1998
}
