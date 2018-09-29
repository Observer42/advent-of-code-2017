use std::io::Result;

mod day1;
mod day2;
mod day3;

fn main() -> Result<()>{
    day1::solve()?;
    day2::solve()?;
    day3::solve();
    Ok(())
}
