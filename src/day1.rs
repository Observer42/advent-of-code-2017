use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("src/input/01.txt")?;
    let mut digits = String::new();
    file.read_to_string(&mut digits)?;
    let digits = digits.trim();
    let day_1_first = solve_captcha_first(digits);
    println!("day 1 first: {}", day_1_first);
    let day_1_second = solve_captcha_second(digits);
    println!("day 1 second: {}", day_1_second);
    Ok(())
}

fn solve_captcha_first(input: &str) -> u32 {
    let func = |x| x + 1;
    solve_captcha(input, &func)
}

fn solve_captcha_second(input: &str) -> u32 {
    let half = input.len() / 2;
    let func = move |x| x + half;
    solve_captcha(input, &func)
}

fn solve_captcha(input: &str, func: &Fn(usize) -> usize) -> u32 {
    let s = input.as_bytes();
    let len = s.len();
    let mut sum = 0;
    for i in 0..len {
        if s[i] == s[func(i) % len] {
            sum += u32::from(s[i] - b'0');
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{solve_captcha_first, solve_captcha_second};

    #[test]
    fn test_solve_captcha_first() {
        assert_eq!(solve_captcha_first("1122"), 3);
        assert_eq!(solve_captcha_first("1111"), 4);
        assert_eq!(solve_captcha_first("1234"), 0);
        assert_eq!(solve_captcha_first("91212129"), 9);
    }

    #[test]
    fn test_solve_captcha_second() {
        assert_eq!(solve_captcha_second("1212"), 6);
        assert_eq!(solve_captcha_second("1221"), 0);
        assert_eq!(solve_captcha_second("123425"), 4);
        assert_eq!(solve_captcha_second("123123"), 12);
        assert_eq!(solve_captcha_second("12131415"), 4);
    }
}
