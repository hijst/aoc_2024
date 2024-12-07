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
    let lines: Vec<String> = lines_from_file("../input/07.txt");

    let targets: Vec<i64> = lines.iter()
        .map(|line| line.split(": ")
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap())
        .collect();

    let operands: Vec<Vec<i64>> = lines.iter()
        .map(|line| line
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(" ")
            .map(|n| n
                .parse::<i64>()
                .unwrap()
                )
            .collect()
            )
        .collect();

    let tasks: Vec<(i64, Vec<i64>)> = targets.into_iter().zip(operands.clone()).collect();

    let (mut ans, mut ans2) = (0,0);

    for task in tasks {
        let ways = find_ways(&task, 0, task.1[0]);
        let ways2 = find_ways2(&task, 0, task.1[0]);
        if ways {
            ans += task.0;
        }
        if ways2 {
            ans2 += task.0;
        }
    }


    println!("answer 1: {}", ans) ; // 945512582195
    println!("answer 2: {}", ans2); // 271691107779347
}

fn find_ways(task: &(i64, Vec<i64>), i: usize, total: i64) -> bool {
    if i == task.1.len() - 1 {
        if total == task.0 {
            return true;
        } else {
            return false;
        }
    }

    return find_ways(task, i+1, total + task.1[i+1]) || find_ways(task, i+1, total * task.1[i+1]);
}

fn find_ways2(task: &(i64, Vec<i64>), i: usize, total: i64) -> bool {
    if i == task.1.len() - 1 {
        if total == task.0 {
            return true;
        } else {
            return false;
        }
    }

    return find_ways2(task, i+1, total + task.1[i+1]) || find_ways2(task, i+1, total * task.1[i+1]) || find_ways2(task, i+1, concat(total, task.1[i+1]));
}

fn concat(n1: i64, n2: i64) -> i64 {
    (n1.to_string() + &n2.to_string()).parse::<i64>().unwrap()
}
