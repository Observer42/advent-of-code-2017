use std::io::Result;

mod day1;
mod day2;

fn main() -> Result<()>{
    day1::solve()?;
    day2::solve()?;
    Ok(())
}
