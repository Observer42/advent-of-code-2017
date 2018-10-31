use ndarray::{Array2, Axis};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;

//ref: https://github.com/sciyoshi/advent-of-rust-2017/blob/master/src/day21/mod.rs

pub fn solve() -> Result<()> {
    let mut file = File::open("input/21.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let transform = parse_input(&raw_input);

    println!("day 21 first: {}", solve_first(&transform));
    println!("day 21 second: {}", solve_second(&transform));

    Ok(())
}

fn parse_input(input: &str) -> HashMap<Array2<bool>, Array2<bool>> {
    let mut res = HashMap::new();

    input.lines().for_each(|line| {
        let squares: Vec<_> = line.split(" => ").collect();
        let mut key = parse_square(squares[0]);
        let val = parse_square(squares[1]);
        for _ in 0..4 {
            res.entry(key.clone()).or_insert(val.clone());
            key.swap_axes(0, 1);
            res.entry(key.clone()).or_insert(val.clone());
            key.invert_axis(Axis(0));
        }
    });
    res
}

fn parse_square(input: &str) -> Array2<bool> {
    let data: Vec<_> = input
        .as_bytes()
        .iter()
        .filter_map(|b| match b {
            b'.' => Some(false),
            b'#' => Some(true),
            _ => None,
        }).collect();

    let size = (data.len() as f64).sqrt() as usize;
    Array2::from_shape_vec((size, size), data).unwrap()
}

fn next(mat: &Array2<bool>, transform: &HashMap<Array2<bool>, Array2<bool>>) -> Array2<bool> {
    let len = mat.len_of(Axis(0));
    let step = if len % 2 == 0 { 2 } else { 3 };
    let times = len / step;
    let new_len = times * (step + 1);
    let mut new_mat = Array2::default((new_len, new_len));

    for i in 0..times {
        for j in 0..times {
            let new_step = step + 1;
            let slice = mat.slice(s![
                (i * step)..((i + 1) * step),
                (j * step)..((j + 1) * step)
            ]);

            if let Some(val) = transform.get(&slice.to_owned()) {
                new_mat
                    .slice_mut(s![
                        (i * new_step)..((i + 1) * new_step),
                        (j * new_step)..((j + 1) * new_step)
                    ]).assign(val);
            }
        }
    }

    new_mat
}

fn solve_general(times: usize, transform: &HashMap<Array2<bool>, Array2<bool>>) -> usize {
    let mut image = parse_square(".#./..#/###");

    for _ in 0..times {
        image = next(&image, transform);
    }

    image.as_slice().unwrap().iter().filter(|val| **val).count()
}

fn solve_first(transform: &HashMap<Array2<bool>, Array2<bool>>) -> usize {
    solve_general(5, transform)
}

fn solve_second(transform: &HashMap<Array2<bool>, Array2<bool>>) -> usize {
    solve_general(18, transform)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_general};

    #[test]
    fn test_solve_general() {
        let transform = parse_input(".#./#../### => ####/####/####/####");
        assert_eq!(16, solve_general(1, &transform));
    }
}
