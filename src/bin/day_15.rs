use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashSet,
    thread::sleep,
    time::Duration,
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
    let path = "../input/15.txt";
    let mut map: Vec<Vec<String>> = lines_from_file(path)
        .into_iter()
        .filter(|l| l.len() > 0 && l.chars().next().unwrap().to_string() == "#")
        .map(|row| row.chars().map(|c| c.to_string()).collect::<Vec<_>>()
        ).collect::<Vec<Vec<_>>>();

    let moves: Vec<(isize, isize)> = lines_from_file(path)
        .into_iter()
        .filter(|l| l.len() > 0 && l.chars().next().unwrap().to_string() != "#")
        .collect::<Vec<_>>()
        .join("")
        .chars()
        .map(|c| match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("Found unexpected character! Namely: {}", c),
        })
        .collect();

    let mut robot_pos: (isize, isize) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| row
            .iter()
            .position(|s| s == "@")
            .map(|j| (i as isize, j as isize))
            ).unwrap();

    for m in &moves {
        let mut next = (robot_pos.0 + m.0, robot_pos.1 + m.1);
        let mut boxes: Vec<(isize, isize)> = vec![];
        loop {
            // illegal move
            if map[next.0 as usize][next.1 as usize] == "#" { break; }
            // legal move
            if map[next.0 as usize][next.1 as usize] == "." {
                // empty the robot's square
                map[robot_pos.0 as usize][robot_pos.1 as usize] = ".".to_string();
                // move the robot
                robot_pos = (robot_pos.0 + m.0, robot_pos.1 + m.1);
                map[robot_pos.0 as usize][robot_pos.1 as usize] = "@".to_string();

                // place the boxes
                for b in boxes {
                    map[b.0 as usize][b.1 as usize] = "O".to_string()
                }
                break;
            }
            // collect boxes
            if map[next.0 as usize][next.1 as usize] == "O" {
                boxes.push((next.0 + m.0, next.1 + m.1));
            }
            // iterate
            next = (next.0 + m.0, next.1 + m.1);
        }

        //println!("{}", &map.clone().into_iter().map(|row| row.join("")).collect::<Vec<_>>().join("\n"));
    }

    let mut m2: Vec<Vec<&str>> = lines_from_file(path)
        .into_iter()
        .filter(|l| l.len() > 0 && l.chars().next().unwrap().to_string() == "#")
        .map(|row| row.chars()
            .map(|c| c.to_string())
            .flat_map(|s| match &s as &str {
                "@" => vec!["@", "."],
                "." => vec![".", "."],
                "O" => vec!["[", "]"],
                "#" => vec!["#", "#"],
                _ => panic!("What the hell")
            })
            .collect::<Vec<_>>()
        ).collect::<Vec<Vec<_>>>();

    robot_pos = m2
        .iter()
        .enumerate()
        .find_map(|(i, row)| row
            .iter()
            .position(|s| *s == "@")
            .map(|j| (i as isize, j as isize))
            ).unwrap();


    for (i, m) in moves.into_iter().enumerate() {
        let mut nexts = vec![(robot_pos.0 + m.0, robot_pos.1 + m.1)];
        let mut lboxes: HashSet<(isize, isize)> = HashSet::new(); //where to place ['s
        let mut rboxes: HashSet<(isize, isize)> = HashSet::new(); //where to place ]'s
        let mut dots: HashSet<(isize, isize)> = HashSet::new(); //where to place .'s
        let mut stop = false; //flag to break in case of illegal move
        loop {
            // queue of positions to check on next iteration
            let mut ns = vec![];

            // check for legal move - then execute it
            let found_end = nexts.iter().all(|n| m2[n.0 as usize][n.1 as usize] == ".");
            if found_end {
                // empty the robot's square
                m2[robot_pos.0 as usize][robot_pos.1 as usize] = ".";

                // first place down the dots
                for dot in dots {
                    m2[dot.0 as usize][dot.1 as usize] = ".";
                }
                // move the robot
                robot_pos = (robot_pos.0 + m.0, robot_pos.1 + m.1);
                m2[robot_pos.0 as usize][robot_pos.1 as usize] = "@";

                // place the boxes
                for lbox in lboxes {
                    m2[lbox.0 as usize][lbox.1 as usize] = "[";
                }
                for rbox in rboxes {
                    m2[rbox.0 as usize][rbox.1 as usize] = "]";
                }
                break;
            }

            // check next squares in front of train of pushed boxes + robot
            for next in nexts {
                // illegal move - abort
                if m2[next.0 as usize][next.1 as usize] == "#" { stop = true; break; }
                // collect boxes
                if m2[next.0 as usize][next.1 as usize] == "[" {
                    lboxes.insert((next.0 + m.0, next.1 + m.1));
                    rboxes.insert((next.0 + m.0, next.1 + m.1 + 1));
                    // not on move to right
                    if m.1 != 1 {
                        ns.push((next.0 + m.0, next.1 + m.1));
                    }
                    ns.push((next.0 + m.0, next.1 + m.1 + 1));
                    dots.insert((next.0, next.1));
                    dots.insert((next.0, next.1 + 1));
                }
                if m2[next.0 as usize][next.1 as usize] == "]" {
                    rboxes.insert((next.0 + m.0, next.1 + m.1));
                    lboxes.insert((next.0 + m.0, next.1 + m.1 - 1));
                    // not on move to left
                    if m.1 != -1 {
                        ns.push((next.0 + m.0, next.1 + m.1));
                    }
                    ns.push((next.0 + m.0, next.1 + m.1 - 1));
                    dots.insert((next.0, next.1));
                    dots.insert((next.0, next.1 - 1));
                }
            }
            if stop { break; }
            nexts = ns.clone();
        }
        //println!("Move {}", i);
        //for c in &m2 {
        //    println!("{:?}", c.join(""));
        //}
    }

    println!("answer 1: {}", score(&map)); // 1527563
    println!("answer 2: {}", score2(&m2)); // 1521635
}

fn score(map: &Vec<Vec<String>>) -> usize {
    let mut score = 0;
    for (i, row) in map.into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if col == "O" {
                score += 100 * i + j;
            }
        }
    }
    return score
}

fn score2(map: &Vec<Vec<&str>>) -> usize {
    let mut score = 0;
    for (i, row) in map.into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if *col == "[" {
                score += 100 * i + j;
            }
        }
    }
    return score
}
