use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap,
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
    let nums: Vec<i64> = lines_from_file("../input/11.txt")
        .into_iter()
        .flat_map(|l| { l
            .split(" ")
            .flat_map(|s| s
                .parse::<i64>().ok()
                )
            .collect::<Vec<i64>>()
        }).collect::<Vec<i64>>();

    let mut dp: HashMap<(i64, i64), i64> = HashMap::new();

    let (ans, ans2) = nums
        .iter()
        .fold((0,0), |(a1, a2), num| 
            (a1 + resolve(*num, 25, &mut dp), a2 + resolve(*num, 75, &mut dp))
            );

    println!("answer 1: {}", ans); //203457 
    println!("answer 2: {}", ans2); //241394363462435
}

fn resolve(num: i64, left: i64, dp: &mut HashMap<(i64,i64), i64>) -> i64 {
    if left == 0 {
        return 1;
    }

    if dp.contains_key(&(num,left)) {
        return dp[&(num,left)];
    }

    let res = next_nums(num).into_iter().map(|n| resolve(n, left - 1, dp)).sum();
    dp.entry((num,left)).or_insert(res);
    return res
}

fn next_nums(num: i64) -> Vec<i64> {
    if num == 0 {
        return vec![1];
    }
    if num.to_string().len() % 2 == 0 {
        return split_num(num.to_string());
    }
    return vec![num * 2024];
}

fn split_num(num: String) -> Vec<i64> {
    let char_len = num.len();
    let (first_part, second_part) = num.split_at(char_len / 2);
    return vec![first_part.parse::<i64>().unwrap(), second_part.parse::<i64>().unwrap()];
}
