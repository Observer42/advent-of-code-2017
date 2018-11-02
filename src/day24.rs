use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/24.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let components = parse_input(&raw_input);

    println!("day 24 first: {}", solve_first(&components));
    println!("day 24 second: {}", solve_second(&components));

    Ok(())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Component(u32, u32);

impl Component {
    fn sum(self) -> u32 {
        self.0 + self.1
    }
}

fn parse_input(input: &str) -> Vec<Component> {
    input
        .lines()
        .filter_map(|line| {
            let mut it = line.split('/').map(|s| s.parse::<u32>().unwrap());

            let first = it.next()?;
            let second = it.next()?;

            Some(Component(first, second))
        }).collect()
}

fn solve_first(input: &[Component]) -> u32 {
    let (mut available_components, port_map) = prepare_input(input);
    let mut res = 0;

    let candidates = &port_map[&0];
    for component in candidates {
        let new_port = if component.0 == 0 {
            component.1
        } else {
            component.0
        };
        available_components.remove(component);
        dfs(
            &mut available_components,
            &port_map,
            &mut res,
            component.sum(),
            new_port,
        );
        available_components.insert(*component);
    }

    res
}

fn solve_second(input: &[Component]) -> u32 {
    let (mut available_components, port_map) = prepare_input(input);
    let mut res = BridgeStrength {
        length: 0,
        strength: 0,
    };

    let candidates = &port_map[&0];
    for component in candidates {
        let new_port = if component.0 == 0 {
            component.1
        } else {
            component.0
        };
        available_components.remove(component);
        let current = BridgeStrength {
            length: 1,
            strength: component.sum(),
        };
        dfs(
            &mut available_components,
            &port_map,
            &mut res,
            current,
            new_port,
        );
        available_components.insert(*component);
    }

    res.strength
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct BridgeStrength {
    length: u32,
    strength: u32,
}

trait NextStrength {
    fn next_strength(&self, strength: u32) -> Self;
}

impl NextStrength for BridgeStrength {
    fn next_strength(&self, strength: u32) -> Self {
        BridgeStrength {
            length: self.length + 1,
            strength: self.strength + strength,
        }
    }
}

impl NextStrength for u32 {
    fn next_strength(&self, strength: u32) -> Self {
        *self + strength
    }
}

fn dfs<T: Copy + Ord + NextStrength + std::fmt::Debug>(
    available_components: &mut HashSet<Component>,
    port_map: &HashMap<u32, Vec<Component>>,
    max_strength: &mut T,
    current: T,
    port: u32,
) -> Option<()> {
    *max_strength = max(*max_strength, current);
    let candidates = port_map.get(&port)?;

    for component in candidates {
        if available_components.contains(component) {
            let new_port = if component.0 == port {
                component.1
            } else {
                component.0
            };
            available_components.remove(component);
            dfs(
                available_components,
                port_map,
                max_strength,
                current.next_strength(component.sum()),
                new_port,
            );
            available_components.insert(*component);
        }
    }

    Some(())
}

fn prepare_input(input: &[Component]) -> (HashSet<Component>, HashMap<u32, Vec<Component>>) {
    let available_components: HashSet<_> = input.iter().cloned().collect();
    let mut port_map = HashMap::new();
    for component in available_components.iter() {
        port_map
            .entry(component.0)
            .or_insert_with(|| vec![])
            .push(*component);

        if component.0 != component.1 {
            port_map
                .entry(component.1)
                .or_insert_with(|| vec![])
                .push(*component);
        }
    }
    (available_components, port_map)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_first, solve_second};

    const INPUT: &str = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";

    #[test]
    fn test_solve_first() {
        assert_eq!(solve_first(&parse_input(INPUT)), 31);
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(solve_second(&parse_input(INPUT)), 19);
    }
}
