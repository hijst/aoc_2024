use std::{
    cmp::min,
    collections::{HashMap, HashSet},
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
    let path = "../input/19.txt";
    let patterns: HashSet<String> = lines_from_file(path)
        .get(0)
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    let mxp = patterns
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();

    let designs: Vec<String> = lines_from_file(path).into_iter().skip(2).collect();
    let mut cache: HashMap<String, i64> = HashMap::new();

    let (mut ans1, mut ans2) = (0, 0);

    for design in designs {
        let w = ways(&design, &patterns, mxp, &mut cache);
        if w > 0 {
            ans1 += 1;
        }
        ans2 += w;
    }

    println!("answer 1: {}", ans1); // 340
    println!("answer 2: {}", ans2); // 717561822679428
}

fn ways(s: &str, patterns: &HashSet<String>, mxp: usize, cache: &mut HashMap<String, i64>) -> i64 {
    if let Some(&v) = cache.get(&s.to_string()) {
        return v;
    }

    let mut ws = 0;
    let n = min(s.len(), mxp);
    for i in (1..=n).rev() {
        let ns = &s[0..i];
        if patterns.contains(ns) {
            if i == s.len() {
                ws += 1;
            } else {
                ws += ways(&s[i..], patterns, mxp, cache);
            }
        }
    }

    cache.insert(s.to_string(), ws);
    ws
}
