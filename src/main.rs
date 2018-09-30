use std::io::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<()> {
    day1::solve()?;
    day2::solve()?;
    day3::solve();
    day4::solve()?;
    day5::solve()?;
    day6::solve()?;
    Ok(())
}
