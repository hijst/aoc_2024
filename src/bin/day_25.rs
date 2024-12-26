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
    let locks_keys = lines_from_file("../input/25.txt")
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|b| !b.is_empty())
        .collect::<Vec<_>>()
        .chunks(7)
        .map(|g| g.to_owned())
        .collect::<Vec<_>>();

    let locks = locks_keys
        .clone()
        .into_iter()
        .filter(|l| l[0].clone().into_iter().all(|c| c == "#".to_owned()))
        .collect::<Vec<_>>();

    let keys = locks_keys
        .clone()
        .into_iter()
        .filter(|l| !l[0].clone().into_iter().all(|c| c == "#".to_owned()))
        .collect::<Vec<_>>();

    let mut ans1 = 0;

    for lock in &locks {
        for key in &keys {
            let mut ct = 0;
            for c in 0..5 {
                if height(lock, c as usize) + height(key, c as usize) > 7 {
                    continue;
                }
                ct += 1;
            }
            if ct == 5 {
                ans1 += 1;
            }
        }
    }

    println!("answer 1: {}", ans1); // 17724064040
    println!("answer 2: {}", 2); // 1998
}

fn height(item: &Vec<Vec<String>>, col: usize) -> i32 {
    let mut res = 0;
    for row in item {
        if row[col] == "#" {
            res += 1;
        }
    }
    return res;
}
