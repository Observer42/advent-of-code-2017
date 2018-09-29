pub fn solve_captcha_first(input: &str) -> u32 {
    let func = |x| x + 1;
    solve_captcha(input, &func)
}

pub fn solve_captcha_second(input: &str) -> u32 {
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
    fn first_problem_first_part() {
        assert_eq!(solve_captcha_first("1122"), 3);
        assert_eq!(solve_captcha_first("1111"), 4);
        assert_eq!(solve_captcha_first("1234"), 0);
        assert_eq!(solve_captcha_first("91212129"), 9);
    }

    #[test]
    fn first_problem_second_part() {
        assert_eq!(solve_captcha_second("1212"), 6);
        assert_eq!(solve_captcha_second("1221"), 0);
        assert_eq!(solve_captcha_second("123425"), 4);
        assert_eq!(solve_captcha_second("123123"), 12);
        assert_eq!(solve_captcha_second("12131415"), 4);
    }
}