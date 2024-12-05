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
    let lines = lines_from_file("../input/05.txt");
    let rules_raw: Vec<Vec<i32>> = lines.iter()
        .filter(|s| s.contains('|'))
        .map(|r| r.split("|")
            .map(|n| n.parse::<i32>().unwrap())
            .collect())
        .collect();

    let instructions: Vec<Vec<i32>> = lines.iter()
        .filter(|s| s.contains(','))
        .map(|i| i.split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect())
        .collect();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in &rules_raw {
        rules.entry(rule[1]).and_modify(|rs| rs.push(rule[0])).or_insert(vec![rule[0]]);
    }

    let (mut ans, mut ans2) = (0,0);

    for instruction in &instructions {
        let mut counts = true;
        for (i, page) in instruction.into_iter().enumerate() {
            if !rules.contains_key(&page) || i >= instruction.len() - 1 { continue }
                for r in &instruction[(i+1 as usize)..] {
                    if rules[&page].contains(r) { counts = false; break;}
                }
        }
        if counts { 
            ans = ans + instruction[instruction.len()/2];
        } else {
            ans2 = ans2 + fixed_score(instruction, &rules);
        }
    }

    println!("answer 1: {}", ans); // 5108
    println!("answer 2: {}", ans2); // 7380
}

fn fixed_score(instruction: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut ins = instruction.to_vec();
    let mut i: usize = 0;
    while i < ins.len() {
        for j in (i+1)..ins.len() {
            if rules[&ins[i]].contains(&ins[j]) { 
                let temp = ins[i];
                ins[i] = ins[j];
                ins[j] = temp;
                continue;
            }
        }
        i = i + 1;
    }
    return ins[ins.len()/2]
}
