use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::io::Result;

struct Process<'a> {
    name: &'a str,
    weight: i32,
    children: Option<Vec<&'a str>>,
}

pub fn solve() -> Result<()> {
    let mut file = File::open("src/input/07.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let processes = parse_input(&input);
    let root = solve_first(&processes);
    println!("day 7 first: {}", root);
    println!("day 7 second: {}", solve_second(processes, root));

    Ok(())
}

fn parse_input(input: &str) -> Vec<Process> {
    input
        .lines()
        .map(|line| {
            let mut name = "";
            let mut weight = 0;
            let mut children = vec![];
            for (i, word) in line.split_whitespace().enumerate() {
                if i == 0 {
                    name = word;
                } else if i == 1 {
                    weight = word[1..word.len() - 1].parse::<i32>().unwrap();
                } else if i > 2 {
                    let child = if word.contains(',') {
                        &word[..word.len() - 1]
                    } else {
                        word
                    };
                    children.push(child);
                }
            }
            Process {
                name,
                weight,
                children: if children.is_empty() {
                    None
                } else {
                    Some(children)
                },
            }
        }).collect()
}

fn solve_first<'a>(processes: &[Process<'a>]) -> &'a str {
    let mut set = HashSet::new();
    processes.iter().for_each(|process| {
        if set.contains(process.name) {
            set.remove(process.name);
        } else {
            set.insert(process.name);
        }

        if let Some(ref children) = process.children {
            for child in children {
                if set.contains(child) {
                    set.remove(child);
                } else {
                    set.insert(child);
                }
            }
        }
    });
    let roots: Vec<_> = set.into_iter().collect();
    roots[0]
}

fn solve_second(processes: Vec<Process>, root: &str) -> i32 {
    let tree = generate_weight_tree(processes, root);
    let mut node = root;
    let mut fix = 0;
    loop {
        if let Some(ref children) = tree[node].1 {
            match children.len() {
                2 => {
                    if fix == 2 * tree[children[0]].0 {
                        node = children[1];
                    } else {
                        node = children[0];
                    }
                    fix /= 2;
                }
                1 => {
                    node = children[0];
                }
                _ => {
                    let (candidate1, weight1, mut count1) =
                        (Some(children[0]), tree[children[0]].0, 1);
                    let (mut candidate2, mut weight2) = (None, 0);
                    for child in children.iter().skip(1) {
                        if tree[child].0 == weight1 {
                            count1 += 1;
                        } else if candidate2.is_none() {
                            candidate2 = Some(*child);
                            weight2 = tree[child].0;
                        }
                    }
                    if candidate2.is_none() {
                        return fix - weight1 * children.len() as i32;
                    } else if count1 == 1 {
                        fix = weight2;
                        node = candidate1.unwrap();
                    } else {
                        fix = weight1;
                        node = candidate2.unwrap();
                    }
                }
            }
        } else {
            return fix;
        }
    }
}

fn generate_weight_tree<'a>(
    processes: Vec<Process<'a>>,
    root: &str,
) -> HashMap<&'a str, (i32, Option<Vec<&'a str>>)> {
    let mut tree = processes
        .into_iter()
        .map(|process| (process.name, (process.weight, process.children)))
        .collect();

    update_weight(&mut tree, root);

    tree
}

fn update_weight(hash: &mut HashMap<&str, (i32, Option<Vec<&str>>)>, root: &str) -> i32 {
    //any better way?
    let children: Vec<_> = hash[root]
        .1
        .iter()
        .flat_map(|child| child.clone())
        .collect();
    let mut sum = hash[root].0;
    for child in children {
        sum += update_weight(hash, child);
    }
    hash.get_mut(root).unwrap().0 = sum;
    sum
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_first, solve_second};

    #[test]
    fn test_solve_both() {
        let input = "pbga (66)
                     xhth (57)
                     ebii (61)
                     havc (66)
                     ktlj (57)
                     fwft (72) -> ktlj, cntj, xhth
                     qoyq (66)
                     padx (45) -> pbga, havc, qoyq
                     tknk (41) -> ugml, padx, fwft
                     jptl (61)
                     ugml (68) -> gyxo, ebii, jptl
                     gyxo (61)
                     cntj (57)";
        let processes = parse_input(&input);
        assert_eq!(solve_first(&processes), "tknk");
        assert_eq!(solve_second(processes, "tknk"), 60);
    }
}
