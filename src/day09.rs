use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/09.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let (first, second) = solve_both(&input);
    println!("day 9 first: {}", first);
    println!("day 9 second: {}", second);
    Ok(())
}

fn solve_both(input: &str) -> (i32, i32) {
    let bytes = input.as_bytes();
    let mut garbage = false;
    let mut ignore = false;
    let mut cur = 1;
    let mut sum = 0;
    let mut garbage_bytes = 0;
    for byte in bytes {
        if ignore {
            ignore = false;
            continue;
        }
        if garbage {
            match byte {
                b'!' => ignore = true,
                b'>' => garbage = false,
                _ => {
                    garbage_bytes += 1;
                }
            }
        } else {
            match byte {
                b'<' => garbage = true,
                b'{' => {
                    sum += cur;
                    cur += 1;
                }
                b'}' => {
                    cur -= 1;
                }
                _ => (),
            }
        }
    }
    (sum, garbage_bytes)
}

#[cfg(test)]
mod tests {
    use super::solve_both;

    #[test]
    fn test_solve_both() {
        assert_eq!(solve_both("{}"), (1, 0));
        assert_eq!(solve_both("{{{}}}"), (6, 0));
        assert_eq!(solve_both("{{{},{},{{}}}}"), (16, 0));
        assert_eq!(solve_both("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
        assert_eq!(solve_both("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
        assert_eq!(solve_both("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
        assert_eq!(solve_both("{<!!!>>}"), (1, 0));
        assert_eq!(solve_both("{<{o'i!a,<{i<a>}"), (1, 10));
    }
}
