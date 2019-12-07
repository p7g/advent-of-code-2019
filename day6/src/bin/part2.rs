use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;
    let orbits = contents
        .trim()
        .lines()
        .map(|s| {
            let sides = s
                .split(')')
                .map(String::from)
                .collect::<Vec<_>>();
            (sides[1].clone(), sides[0].clone())
        })
        .collect::<HashMap<_, _>>();

    let mut all_names = orbits
        .keys()
        .map(|k| k.clone())
        .collect::<HashSet<_>>();
    all_names.extend(orbits.values().map(String::clone));

    let mut reverse_orbits: HashMap<_, Vec<String>> = all_names
        .into_iter()
        .map(|n| (n, Vec::new()))
        .collect::<HashMap<_, _>>();

    for (k, v) in orbits.iter() {
        reverse_orbits.get_mut(k).unwrap().push(v.clone());
        reverse_orbits.get_mut(v).unwrap().push(k.clone());
    }

    let mut q: HashSet<String> = HashSet::new();
    let mut dist: HashMap<String, i64> = HashMap::new();
    let mut prev: HashMap<String, Option<String>> = HashMap::new();

    for name in reverse_orbits.keys() {
        let name = name.to_string();
        dist.insert(name.clone(), i64::max_value() - 1);
        prev.insert(name.clone(), None);
        q.insert(name);
    }

    dist.insert(String::from("YOU"), 0);

    while !q.is_empty() {
        let u = q.iter().min_by_key(|n| dist[*n]).unwrap().clone();

        q.remove(&u);

        if let Some(vs) = reverse_orbits.get(&u) {
            for v in vs {
                let alt = dist[&u] + 1;
                if alt < dist[v] {
                    let v = v.to_string();
                    dist.insert(v.clone(), alt);
                    prev.insert(v, Some(u.clone()));
                }
            }
        }
    }

    let mut s = VecDeque::new();
    let mut u = Some(String::from("SAN"));
    while let Some(v) = prev[&u.clone().unwrap()].clone() {
        s.push_front(u.clone());
        u = Some(v);
    }

    // -1 for transfer to SAN, -1 to go from number of nodes to number of edges
    println!("{}", s.len() - 2);

    Ok(())
}
