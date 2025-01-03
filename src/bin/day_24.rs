use std::{
    collections::HashMap,
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

#[derive(Debug)]
struct Gate {
    in1: String,
    op: String,
    in2: String,
    out: String,
}

fn solve() {
    let lines: Vec<_> = lines_from_file("../input/24.txt");

    let mut values: HashMap<String, i8> = lines
        .iter()
        .filter(|l| l.contains(':'))
        .map(|s| s.split_once(": ").unwrap())
        .map(|(k, v)| (k.to_string(), v.parse::<i8>().unwrap()))
        .collect();

    let mut gates: Vec<Gate> = lines
        .iter()
        .filter(|l| l.contains('-'))
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(k, v)| (k.split(" ").collect::<Vec<_>>(), v))
        .map(|(k, v)| Gate {
            in1: k[0].to_owned(),
            op: k[1].to_owned(),
            in2: k[2].to_owned(),
            out: v.to_owned(),
        })
        .collect();

    let mut changed = 1;

    while changed > 0 {
        changed = 0;
        let mut new_gates: Vec<Gate> = vec![];

        for gate in gates {
            if values.contains_key(&gate.in1) && values.contains_key(&gate.in2) {
                changed += 1;
                match gate.op.as_str() {
                    "AND" => values.insert(
                        gate.out.to_owned(),
                        if (*values.get(&gate.in1).unwrap() == 1 as i8)
                            && (*values.get(&gate.in2).unwrap() == 1 as i8)
                        {
                            1
                        } else {
                            0
                        },
                    ),
                    "OR" => values.insert(
                        gate.out.to_owned(),
                        if (*values.get(&gate.in1).unwrap() == 1 as i8)
                            || (*values.get(&gate.in2).unwrap() == 1 as i8)
                        {
                            1
                        } else {
                            0
                        },
                    ),
                    "XOR" => values.insert(
                        gate.out.to_owned(),
                        if (*values.get(&gate.in1).unwrap() == 1 as i8)
                            != (*values.get(&gate.in2).unwrap() == 1 as i8)
                        {
                            1
                        } else {
                            0
                        },
                    ),
                    _ => {
                        panic!("unknown operation {}", gate.op);
                    }
                };
            } else {
                new_gates.push(gate);
            }
        }
        gates = new_gates;
    }

    let mut zs: Vec<(String, i8)> = values
        .into_iter()
        .filter(|(k, _)| k[0..1] == *"z")
        .map(|(k, v)| (k, v))
        .collect();

    zs.sort_by(|(k, _), (k2, _)| k2.cmp(k));

    let zb = zs
        .iter()
        .map(|(_, v)| v.to_string())
        .collect::<Vec<String>>()
        .join("");

    let ans1 = i64::from_str_radix(&zb, 2).unwrap();

    println!("answer 1: {:?}", ans1); //
    println!("answer 2: {:?}", 0); //
}
