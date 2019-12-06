use std::collections::HashMap;
use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;
    let orbits = contents
        .trim()
        .lines()
        .map(|s| {
            let sides = s.split(')').collect::<Vec<_>>();
            (sides[1], sides[0])
        })
        .collect::<HashMap<_, _>>();

    let mut sum = 0 as usize;
    for orbitee in orbits.values() {
        let mut current = orbitee;
        while let Some(object) = orbits.get(current) {
            sum += 1;
            current = object;
        }
        sum += 1; // account for COM which is not in the hashmap
    }

    println!("{}", sum);

    Ok(())
}
