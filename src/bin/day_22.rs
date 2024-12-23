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
    let nums: Vec<i64> = lines_from_file("../input/22.txt")
        .into_iter()
        .map(|a| a.parse::<i64>().unwrap())
        .collect();

    let mut ans1 = 0;

    for num in &nums {
        let mut n = *num;
        for _i in 0..2000 {
            let op1 = mix_prune(n * 64, n);
            let op2 = mix_prune(op1 / 32, op1);
            let op3 = mix_prune(op2 * 2048, op2);
            n = op3;
        }
        ans1 += n;
    }

    // part 2
    let mut ans2 = 0;
    let mut sequences: Vec<Vec<i64>> = vec![];

    for h in -9..10 {
        for j in -9..10 {
            for k in -9..10 {
                for l in -9..10 {
                    sequences.push(vec![h, j, k, l]);
                }
            }
        }
    }

    let mut hm: HashMap<(i64, i64, i64, i64, i64), i64> = HashMap::new();
    let mut hm2: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for num in &nums {
        let mut n = *num;
        let mut p = n % 10;
        let mut cs: VecDeque<i64> = VecDeque::from(vec![10, 10, 10, 10]);
        for _i in 0..2000 {
            let op1 = mix_prune(n * 64, n);
            let op2 = mix_prune(op1 / 32, op1);
            let op3 = mix_prune(op2 * 2048, op2);
            n = op3;
            let c = (n % 10) - p;
            p = n % 10;
            cs.pop_front();
            cs.push_back(c);
            if !hm.contains_key(&(*num, cs[0], cs[1], cs[2], cs[3])) {
                hm.insert((*num, cs[0], cs[1], cs[2], cs[3]), p);
                let new = hm2.get(&(cs[0], cs[1], cs[2], cs[3])).unwrap_or(&0) + p;
                hm2.insert((cs[0], cs[1], cs[2], cs[3]), new);
            }
        }
    }

    for seq in sequences {
        let total = *hm2.get(&(seq[0], seq[1], seq[2], seq[3])).unwrap_or(&0);
        if total > ans2 {
            ans2 = total;
        }
    }

    println!("answer 1: {}", ans1); // 17724064040
    println!("answer 2: {}", ans2); // 1998
}

fn mix_prune(n: i64, prev_n: i64) -> i64 {
    return (n ^ prev_n) % 16777216;
}
