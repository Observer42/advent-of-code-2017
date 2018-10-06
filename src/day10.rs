use std::fs::File;
use std::io::Read;
use std::io::Result;

const SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];

pub fn solve() -> Result<()> {
    let mut file = File::open("input/10.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let parse_first: Vec<u8> = input
        .split(',')
        .filter_map(|digits| digits.parse::<u8>().ok())
        .collect();

    let input_second = input.trim().to_string();

    println!("day 10 first: {}", solve_first(parse_first, 256));
    println!("day 10 second: {}", solve_second(input_second, 256));
    Ok(())
}

fn solve_first(input: Vec<u8>, list_size: usize) -> u32 {
    let hash_prep = knot_hash_prep(input, list_size, 1);
    u32::from(hash_prep[0]) * u32::from(hash_prep[1])
}

fn solve_second(input: String, list_size: usize) -> u128 {
    knot_hash(input.into_bytes(), list_size, 64)
}

fn knot_hash_prep(mut input: Vec<u8>, list_size: usize, round: u32) -> Vec<u8> {
    input.extend(&SUFFIX);
    let mut list: Vec<u8> = (0..list_size).map(|i| i as u8).collect();
    let mut pos = 0;
    let mut skip = 0;

    for _ in 0..round {
        for &length in &input {
            partial_reverse(&mut list, pos as usize, length as usize);
            pos = (pos + skip + length as usize) % list_size;
            skip += 1;
        }
    }
    list
}

pub fn knot_hash(input: Vec<u8>, list_size: usize, round: u32) -> u128 {
    let hash_prep = knot_hash_prep(input, list_size, round);

    let mut res: u128 = 0;
    for chunk in hash_prep.chunks(16) {
        res <<= 8;
        res |= u128::from(chunk.iter().fold(0u8, |a, b| a ^ b));
    }
    res
    //format!("{:0x}", res)
}

fn partial_reverse(list: &mut [u8], pos: usize, size: usize) {
    let len = list.len();
    for i in 0..(size / 2) {
        list.swap((pos + i) % len, (pos + size - 1 - i) % len);
    }
}

#[cfg(test)]
mod tests {
    use super::solve_second;

    #[test]
    fn test_solve_second() {
        assert_eq!(
            solve_second("".to_string(), 256),
            0xa2582a3a0e66e6e86e3812dcb672a272
        );
        assert_eq!(
            solve_second("AoC 2017".to_string(), 256),
            0x33efeb34ea91902bb2f59c9920caa6cd
        );
        assert_eq!(
            solve_second("1,2,3".to_string(), 256),
            0x3efbe78a8d82f29979031a4aa0b16a9d
        );
        assert_eq!(
            solve_second("1,2,4".to_string(), 256),
            0x63960835bcdc130f0b66d7ff4f6a5a8e
        );
    }
}
