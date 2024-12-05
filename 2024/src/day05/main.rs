use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn part1(input: &str) {
    let mut orderings: HashMap<usize, HashSet<usize>> = HashMap::new();

    input
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut ints = l.split('|').map(|i| i.parse::<usize>().unwrap());
            (ints.next().unwrap(), ints.next().unwrap())
        })
        .for_each(|(a, b)| {
            orderings.entry(a).or_default().insert(b);
        });

    let sum: usize = input
        .split('\n')
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|pages| {
            let invalid = pages
                .iter()
                .enumerate()
                .rev()
                .filter(|(_, v)| orderings.contains_key(v))
                .any(|(i, v)| pages.iter().take(i).any(|k| orderings[v].contains(k)));

            if invalid {
                0
            } else {
                pages[pages.len() / 2]
            }
        })
        .sum();

    println!("{sum}");
}

fn part2(input: &str) {
    let mut orderings: HashMap<usize, HashSet<usize>> = HashMap::new();

    input
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut ints = l.split('|').map(|i| i.parse::<usize>().unwrap());
            (ints.next().unwrap(), ints.next().unwrap())
        })
        .for_each(|(a, b)| {
            orderings.entry(a).or_default().insert(b);
        });

    let sum: usize = input
        .split('\n')
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|pages| {
            pages
                .iter()
                .enumerate()
                .rev()
                .filter(|(_, v)| orderings.contains_key(v))
                .any(|(i, v)| pages.iter().take(i).any(|k| orderings[v].contains(k)))
        })
        .map(|mut pages| {
            let mut modified = false;

            loop {
                modified = false;
                let swaps = pages
                    .iter()
                    .enumerate()
                    .rev()
                    .filter(|(_, v)| orderings.contains_key(v))
                    .filter_map(|(i, v)| {
                        let first = pages
                            .iter()
                            .enumerate()
                            .take(i)
                            .find(|(_, k)| orderings[v].contains(k))?;
                        Some((first.0, i))
                    })
                    .collect::<Vec<_>>();

                for (a, b) in swaps {
                    modified = true;
                    pages.swap(a, b)
                }

                if !modified {
                    break;
                }
            }

            pages[pages.len() / 2]
        })
        .sum();

    println!("{sum}");
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
