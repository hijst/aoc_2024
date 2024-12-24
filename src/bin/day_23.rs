use std::{
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
    let edges: Vec<(String, String)> = lines_from_file("../input/23.txt")
        .into_iter()
        .map(|a| a.split("-").map(|s| s.to_owned()).collect::<Vec<_>>())
        .map(|p| (p[0].to_owned(), p[1].to_owned()))
        .collect();

    let mut adjs: HashMap<String, HashSet<String>> = HashMap::new();
    let mut cycles: HashSet<String> = HashSet::new();

    for edge in &edges {
        adjs.entry(edge.0.to_owned())
            .or_insert(HashSet::new())
            .insert(edge.1.to_owned());

        adjs.entry(edge.1.to_owned())
            .or_insert(HashSet::new())
            .insert(edge.0.to_owned());
    }

    for (n, nbs) in &adjs {
        for nb in nbs {
            for nb2 in nbs {
                if *nb != *nb2 && adjs[nb].contains(nb2) && adjs[nb2].contains(nb) {
                    let mut key: Vec<&String> = vec![n, nb, nb2];
                    key.sort();
                    cycles.insert(
                        key.clone()
                            .into_iter()
                            .map(|s| s.to_owned())
                            .collect::<Vec<_>>()
                            .join(","),
                    );
                }
            }
        }
    }

    let mut ans1 = 0;

    for cycle in &cycles {
        if cycle[0..1] == *"t" || cycle[3..4] == *"t" || cycle[6..7] == *"t" {
            ans1 += 1;
        }
    }

    // part 2
    loop {
        //hash set of cycles of size n + 1 each loop (we build on last iteration and try to add 1)
        let mut new_cycles = HashSet::new();
        for cycle in &cycles {
            let cycle_nodes = unhash(cycle);
            let rep = cycle_nodes.iter().collect::<Vec<_>>()[0];
            for nb in adjs.get(rep).unwrap() {
                if !cycle_nodes.contains(nb) {
                    let mut found = true;
                    for cn in &cycle_nodes {
                        if !adjs.get(cn).unwrap().contains(nb) {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        let mut new_cycle_nodes =
                            cycle_nodes.to_owned().into_iter().collect::<Vec<_>>();
                        new_cycle_nodes.push(nb.to_owned());
                        new_cycles.insert(hash(&new_cycle_nodes));
                        break;
                    }
                }
            }
        }
        cycles = new_cycles.clone();

        if cycles.len() == 1 {
            break;
        }
    }

    println!("answer 1: {}", ans1); // 1314
    println!("answer 2: {}", cycles.iter().next().unwrap()); // bg,bu,ce,ga,hw,jw,nf,nt,ox,tj,uu,vk,wp
}

fn hash(v: &Vec<String>) -> String {
    let mut m: Vec<String> = v.to_vec();
    m.sort();
    return m.join(",");
}

fn unhash(s: &String) -> HashSet<String> {
    s.split(",").map(|s| s.to_owned()).collect::<HashSet<_>>()
}
