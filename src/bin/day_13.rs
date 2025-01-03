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
    let lines: Vec<(i64, i64)> = lines_from_file("../input/13.txt")
        .into_iter()
        .filter(|l| l.len() > 1)
        .map(|l| {
            l.split(",")
                .map(|part| {
                    part.chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<i64>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .map(|score| (score[0].clone(), score[1].clone()))
        .collect::<Vec<_>>();

    let mut ans1 = 0;
    lines.chunks(3).for_each(|chunk| {
        let price = find_cheapest(chunk[0], chunk[1], chunk[2]);
        if price < 10000000 {
            ans1 += price;
        }
    });

    let mut ans2 = 0;

    lines.chunks(3).for_each(|chunk| {
        ans2 += find_cheapest2(chunk[0], chunk[1], chunk[2]);
    });

    println!("answer 1: {}", ans1); // 34787
    println!("answer 2: {}", ans2); // 85644161121698
}

fn find_cheapest(a: (i64, i64), b: (i64, i64), x: (i64, i64)) -> i64 {
    let mut min = 10000000;
    for h in 1..101 {
        for l in 1..101 {
            if a.0 * h + b.0 * l == x.0 && a.1 * h + b.1 * l == x.1 {
                min = h * 3 + l;
            }
        }
    }
    return min;
}

fn find_cheapest2(a: (i64, i64), b: (i64, i64), x: (i64, i64)) -> i64 {
    let mut ta: i64 = 0;
    let mut tb: i64 = 0;
    let diffx = x.0 - x.1;
    let diffa = a.0 - a.1;
    let diffb = b.0 - b.1;
    let c = 10000000000000;

    let correctedx = x.0 + c;

    let lh1 = diffb * a.0;
    let rh1 = diffx * a.0;

    let lh2 = b.0 * diffa;
    let rh2 = correctedx * diffa;

    let lh3 = lh2 - lh1;
    let rh3 = rh2 - rh1;

    if rh3 % lh3 == 0 {
        tb = rh3 / lh3;
        let lh4 = -diffb * tb + diffx;
        if diffa != 0 && lh4 % diffa == 0 {
            ta = lh4 / diffa;
        } else {
            return 0;
        }
    }

    if ta > 0 && tb > 0 {
        return 3 * ta + tb;
    }
    return 0;
}
