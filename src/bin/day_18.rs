use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::{HashMap,HashSet,VecDeque}
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
    let path = "../input/18.txt";
    let lines = lines_from_file(path)
        .into_iter()
        .map(|row| row.split(",").collect::<Vec<_>>().into_iter().map(|p| p.parse::<usize>().unwrap()).collect()
        ).map(|a: Vec<usize>| (a[0], a[1]));
        
    
    let mut map = HashSet::from_iter(lines.clone().take(1024).collect::<Vec<_>>());
    let fin: (usize, usize) = (70,70);
    let w: isize = fin.0 as isize;

    let mut queue: VecDeque<(usize,usize)> = VecDeque::new();
    let mut dists: HashMap<(usize, usize), usize> = HashMap::new();
    queue.push_back((0,0));
    dists.insert((0,0),0);
    let mut ans1 = 0;
    let mut found = false;

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        let nbs = get_neighbors(cur, w, &map);
        for nb in nbs {
            if nb == fin {
                ans1 = dists.get(&cur).unwrap() + 1;
                found = true;
                break;
            }
            if !dists.contains_key(&nb) {
                queue.push_back(nb);
                dists.insert(nb, dists.get(&cur).unwrap() + 1);
            }
        }
        if found { break };
    }

    // part 2
    let mut ans2: (usize, usize) = (0,0);
    for i in 0..5000 {
        map = HashSet::from_iter(lines.clone().take(i + 1).collect::<Vec<_>>());
        queue = VecDeque::new();
        dists = HashMap::new();
        queue.push_back((0,0));
        dists.insert((0,0),0);
        found = false;

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            let nbs = get_neighbors(cur, w, &map);
            for nb in nbs {
                if nb == fin {
                    found = true;
                    break;
                }
                if !dists.contains_key(&nb) {
                    queue.push_back(nb);
                    dists.insert(nb, dists.get(&cur).unwrap() + 1);
                }
            }
            if found { break };
        }
        if found { continue };
        ans2 = lines.collect::<Vec<_>>()[i];
        break;
    }

    println!("answer 1: {}", ans1); // 1527563
    println!("answer 2: {:?}", ans2); // 1521635
}

fn get_neighbors(cur: (usize,usize), w: isize, map: &HashSet<(usize,usize)> ) -> Vec<(usize,usize)> {
    let mut nbs: Vec<(usize,usize)> = vec![];
    let row = cur.0 as isize;
    let col = cur.1 as isize;
    let dirs: Vec<(isize,isize)> = vec![(-1,0), (1,0), (0,1), (0,-1)];

    for dir in dirs {
        let nr = row + dir.0;
        let nc = col + dir.1;
        if nr >= 0 && nc >= 0 && nr <= w && nc <= w {
            nbs.push((nr as usize, nc as usize));
        } 
    }
    return nbs.into_iter().filter(|nb| !map.contains(&nb)).collect()
} 
