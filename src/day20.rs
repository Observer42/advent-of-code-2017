use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::ops::{Add, AddAssign, Mul};

pub fn solve() -> Result<()> {
    let mut file = File::open("input/20.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let particles = parse_input(&raw_input);

    println!("day 20 first: {}", solve_first(&particles));
    println!("day 20 second: {}", solve_second(&particles));

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coordinate { x, y, z }
    }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Coordinate) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul<i32> for Coordinate {
    type Output = Coordinate;

    fn mul(self, other: i32) -> Coordinate {
        Coordinate {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[derive(Clone)]
struct Particle {
    p: Coordinate,
    v: Coordinate,
    a: Coordinate,
}

impl Particle {
    fn new(data: &[Vec<i32>]) -> Self {
        let p = Coordinate::new(data[0][0], data[0][1], data[0][2]);
        let v = Coordinate::new(data[1][0], data[1][1], data[1][2]);
        let a = Coordinate::new(data[2][0], data[2][1], data[2][2]);
        Particle { p, v, a }
    }

    fn quadratic_param(&self) -> i32 {
        self.a.manhattan()
    }

    fn linear_param(&self) -> i32 {
        (self.a + self.v).manhattan()
    }

    fn constant_param(&self) -> i32 {
        self.p.manhattan()
    }

    fn long_term_cmp(&self, other: &Particle) -> Ordering {
        //distance: at^2 + (a + v)t + p
        match self.quadratic_param().cmp(&other.quadratic_param()) {
            Ordering::Equal => match self.linear_param().cmp(&other.linear_param()) {
                Ordering::Equal => self.constant_param().cmp(&other.constant_param()),
                other => other,
            },
            other => other,
        }
    }

    fn next(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }
}

fn parse_input(input: &str) -> Vec<Particle> {
    input
        .lines()
        .map(|line| {
            let particle_data: Vec<&str> = line.trim().split(", ").collect();
            let data: Vec<Vec<i32>> = particle_data
                .iter()
                .map(|&coordinate| {
                    coordinate[3..(coordinate.len() - 1)]
                        .split(',')
                        .map(|num| num.trim().parse::<i32>().unwrap())
                        .collect()
                }).collect();
            Particle::new(&data)
        }).collect()
}

fn solve_first(particles: &[Particle]) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| left.long_term_cmp(&right))
        .unwrap()
        .0
}

fn solve_second(particles: &[Particle]) -> usize {
    let mut particles = particles.to_vec();
    for _ in 1..10_000 {
        let mut collision = HashMap::new();
        particles
            .iter()
            .for_each(|particle| *collision.entry(particle.p).or_insert(0) += 1);

        particles.retain(|particle| collision[&particle.p] == 1);
        particles.iter_mut().for_each(|particle| particle.next());

        collision.clear();
    }
    particles.len()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_first, solve_second};

    const INPUT: &str = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
                         p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
                         p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
                         p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

    #[test]
    fn test_solve_first() {
        let particles = parse_input(INPUT);
        assert_eq!(solve_first(&particles), 2);
    }

    #[test]
    fn test_solve_second() {
        let particles = parse_input(INPUT);
        assert_eq!(solve_second(&particles), 1);
    }
}
