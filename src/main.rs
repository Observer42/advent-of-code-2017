use std::io::Result;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<()>{
    day1::solve()?;
    day2::solve()?;
    day3::solve();
    day4::solve()?;
    Ok(())
}
