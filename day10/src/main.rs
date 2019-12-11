use std::fs;

type Point = (usize, usize);

#[derive(Debug)]
enum Line {
    Slope(Point, (i64, i64)),
    Offset(i64),
}

impl Line {
    pub fn has_point(&self, p: Point) -> bool {
        match self {
            Line::Offset(b) => p.0 as i64 == *b,
            Line::Slope(origin, (dy, dx)) => {
                let x = (p.0 as i64 - origin.0 as i64).abs();
                let y = (p.1 as i64 - origin.1 as i64).abs();

                if *dy == 0 {
                    p.1 == origin.1
                } else {
                    x % dx == 0 && y % dy == 0 && x / dx == y / dy
                }
            }
        }
    }
}

fn gcd(mut m: i64, mut n: i64) -> i64 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }

    n.abs()
}

fn simplify_fraction(f: (i64, i64)) -> (i64, i64) {
    let (a, b) = f;
    let d = gcd(a, b);

    (a / d, b / d)
}

fn linear_equation(a: Point, b: Point) -> Line {
    let dy = b.1 as i64 - a.1 as i64;
    let dx = b.0 as i64 - a.0 as i64;
    if dx == 0 {
        Line::Offset(a.0 as i64)
    } else {
        Line::Slope(a, simplify_fraction((dy, dx)))
    }
}

macro_rules! between {
    ($num:expr, $top:expr, $bottom:expr) => {{
        if $top > $bottom {
            $num <= $top && $num >= $bottom
        } else {
            $num >= $top && $num <= $bottom
        }
    }};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asteroids = get_input()?;

    let best = asteroids.iter().max_by_key(|(x, y)| {
        println!("{},{}", x, y);
        let mut seen = 0;
        for asteroid in &asteroids {
            if *asteroid == (*x, *y) {
                continue;
            }

            let line = linear_equation((*x, *y), *asteroid);
            let blocking = asteroids.iter().any(|a| {
                if a == asteroid || *a == (*x, *y) {
                    return false;
                }

                a != asteroid
                    && line.has_point(*a)
                    && between!(a.0, *x, asteroid.0)
                    && between!(a.1, *y, asteroid.1)
            });

            if !blocking {
                seen += 1;
            }
        }

        println!("{:?} {}", (x, y), seen);

        seen
    });

    println!("{:?}", best);

    Ok(())
}

fn get_input() -> std::io::Result<Vec<(usize, usize)>> {
    let input = fs::read_to_string("input.txt")?;

    Ok(input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| if ch == '#' { Some((x, y)) } else { None })
        })
        .collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_positive_slope() {
        let line = linear_equation((1, 1), (4, 7));

        assert!(line.has_point((2, 3)));
        assert!(line.has_point((3, 5)));
        assert!(!line.has_point((2, 4)));
    }

    #[test]
    fn test_negative_slope() {
        let line = linear_equation((4, 4), (1, 1));

        assert!(line.has_point((2, 2)));
        assert!(line.has_point((3, 3)));
        assert!(!line.has_point((2, 3)));
    }

    #[test]
    fn test_vertical_line() {
        let line = linear_equation((4, 0), (4, 3));

        assert!(line.has_point((4, 2)));
        assert!(!line.has_point((5, 4)));
    }

    #[test]
    fn test_horizontal_line() {
        let line = linear_equation((2, 2), (5, 2));

        assert!(line.has_point((3, 2)));
        assert!(!line.has_point((3, 3)));
    }

    #[test]
    fn test_between() {
        let tests = &[
            ((3, 1, 5), true),
            ((3, 5, 1), true),
            ((4, 3, 1), false),
            ((4, 1, 3), false),
            ((7, 1, 5), false),
            ((2, 0, 3), true),
        ];

        for ((num, top, bottom), result) in tests {
            assert!(between!(num, top, bottom) == *result);
        }
    }
}
