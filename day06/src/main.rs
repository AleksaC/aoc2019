use std::collections::{HashMap, HashSet};

fn main() {
    let orbits: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|x| x.split(")").collect())
        .collect();

    let mut res = orbits.len() as u32;

    let mut prec: HashMap<&str, &str> = HashMap::new();
    let mut objects: HashSet<&str> = HashSet::new();

    for orbit in orbits {
        prec.insert(orbit[1], orbit[0]);
        objects.insert(orbit[1]);
    }

    for object in objects {
        let mut curr = prec.get(object);
        let mut num_indirect = 0;
        loop {
            match curr {
                Some(predecessor) => {
                    curr = prec.get(predecessor);
                    num_indirect += 1
                }
                None => {
                    num_indirect -= 1;
                    res += num_indirect;
                    break;
                }
            }
        }
    }

    println!("Part one: {}", res);

    let mut visited: HashMap<&str, u32> = HashMap::new();

    let mut you = prec.get("YOU");

    let mut num_transfers = 0;

    loop {
        match you {
            Some(predecessor) => {
                visited.insert(predecessor, num_transfers);
                you = prec.get(predecessor);
                num_transfers += 1;
            }
            None => { break; }
        }
    }

    let mut san = prec.get("SAN");

    num_transfers = 0;

    loop {
        match san {
            Some(predecessor) => {
                match visited.get(san.unwrap()) {
                    Some(transfers_from) => {
                        println!("Part two: {}", transfers_from + num_transfers);
                        break;
                    }
                    None => {}
                }
                san = prec.get(predecessor);
                num_transfers += 1;
            }
            None => { break; }
        }
    }
}
