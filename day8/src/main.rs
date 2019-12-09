use std::collections::HashSet;
use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const TRANSPARENT: u32 = 2;

fn get_input() -> Result<Box<[u32]>, std::io::Error> {
    Ok(fs::read_to_string("input.txt")?
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice())
}

fn num_digits(iter: std::slice::Iter<u32>, num: u32) -> i64 {
    iter.fold(0, |acc, n| if *n == num { acc + 1 } else { acc })
}

fn part1(chunks: std::slice::Chunks<u32>) {
    // part 1
    let minzeros = chunks.min_by_key(|c| num_digits(c.iter(), 0)).unwrap();

    let numones = num_digits(minzeros.iter(), 1);
    let numtwos = num_digits(minzeros.iter(), 2);

    println!("Part 1: {}", numones * numtwos);
}

fn part2(chunks: std::slice::Chunks<u32>) {
    let mut screen: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    let mut written = HashSet::new();

    for pixels in chunks {
        for (i, pixel) in pixels.iter().enumerate() {
            let y = i / WIDTH;
            let x = i % WIDTH;

            if !written.contains(&(x, y)) && *pixel != TRANSPARENT {
                written.insert((x, y));
                screen[y][x] = *pixel as u8;
            }
        }
    }

    for row in &screen {
        for column in row {
            if *column == 1 {
                print!("\x1b[47m  \x1b[0m");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let chunks = input.chunks(WIDTH * HEIGHT);

    part1(chunks);

    let input = get_input()?;
    let chunks = input.chunks(WIDTH * HEIGHT);

    part2(chunks);

    Ok(())
}
