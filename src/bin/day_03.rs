use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use regex::Regex;

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
    let lines = lines_from_file("../input/03.txt");

    let mut ans = 0;
    for line in &lines {
        ans = ans + sum_up(line);
    }
    
    let ans2 = sum_up_2(&lines.join(""));

    println!("answer 1: {}", ans); // 184576302
    println!("answer 2: {}", ans2)
}

fn sum_up(s: &String) -> i32 {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    let re2 = Regex::new(r"(\d{1,3},\d{1,3})").unwrap();

    let ans: i32 = re.find_iter(&s)
        .flat_map(|m| { 
            re2.find_iter(m.as_str())
            .map(|s| { s
                .as_str()
                .split(",")
                .collect()
            })
            .map(|p: Vec<&str>| p[0].parse::<i32>().unwrap() * p[1].parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
    .sum();

    return ans;
}

fn sum_up_2(s: &String) -> i32 {
    let re = Regex::new(r"(mul)\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let re2 = Regex::new(r"(\d{1,3},\d{1,3})").unwrap();

    let re_mul = Regex::new(r"(mul)\(\d{1,3},\d{1,3}\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();


    let mut is_on = true;

    let ans: i32 = re.find_iter(&s)
        .map(|mt| mt.as_str())
        .map(|m| match m {
            _ if re_dont.is_match(m) => {
                is_on = false;
                0
            },
            _ if re_do.is_match(m) => {
                is_on = true;
                0
            },
            _ if re_mul.is_match(m) => { re2.find_iter(m)
            .map(|s| { s
                .as_str()
                .split(",")
                .collect()
            })
            .map(|p: Vec<&str>| if is_on { p[0].parse::<i32>().unwrap() * p[1].parse::<i32>().unwrap() } else { 0 })
                .collect::<Vec<i32>>()[0]
            },
            _ => 0
        })
    .sum();
    return ans;
}



