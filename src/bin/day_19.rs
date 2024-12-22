use regex::Regex;
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
    let lines = lines_from_file("../input/19.txt");
    let mut patterns: Vec<&str> = lines.get(0).unwrap().split(", ").collect::<Vec<_>>();
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    let patterns_joined = patterns
        .into_iter()
        .filter(|s| s.len() == 1 || s.contains("w"))
        .collect::<Vec<_>>()
        .join("|");

    let designs: Vec<String> = lines.into_iter().skip(2).collect();

    let re = Regex::new(&format!(r"(?:{})*", patterns_joined)).unwrap();

    let mut ans1 = 0;

    for design in &designs {
        let Some(caps) = re.captures(&design) else {
            continue;
        };
        println!("caps {}", &caps[0]);
        if caps[0] == *design {
            ans1 += 1
        }
    }

    println!("{}", patterns_joined);
    println!("{:?}", &designs);

    println!("answer 1: {}", ans1); // 1527563
    println!("answer 2: {}", 2); // 1521635
}
