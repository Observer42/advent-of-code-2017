use std::fs::File;
use std::io::Read;
use std::io::Result;

mod day1;

fn main() -> Result<()> {
    let mut file = File::open("src/input/01.txt")?;
    let mut digits = String::new();
    file.read_to_string(&mut digits)?;
    let digits = digits.trim();
    let day_1_first = day1::solve_captcha_first(digits);
    println!("day1 first: {}", day_1_first);
    let day_1_second = day1::solve_captcha_second(digits);
    println!("day1 second: {}", day_1_second);
    Ok(())
}

