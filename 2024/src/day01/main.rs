use std::{
    collections::{BinaryHeap, HashMap},
    io::{self, Read},
};

fn part1(input: &str) {
    let (mut a, mut b): (BinaryHeap<_>, BinaryHeap<_>) = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut ints = line.split("   ").map(|i| i.parse::<i32>().unwrap());
            (ints.next().unwrap(), ints.next().unwrap())
        })
        .unzip();

    let mut sum = 0;
    for _ in 0..a.len() {
        sum += a.pop().unwrap().abs_diff(b.pop().unwrap());
    }

    println!("{sum}")
}

fn part2(input: &str) {
    let (a, b): (Vec<_>, Vec<_>) = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut ints = line.split("   ").map(|i| i.parse::<i32>().unwrap());
            (ints.next().unwrap(), ints.next().unwrap())
        })
        .unzip();

    let mut hm = HashMap::<i32, i32>::new();
    for v in &b {
        match hm.get_mut(v) {
            Some(c) => *c += 1,
            None => {
                hm.insert(*v, 1);
            }
        }
    }

    let mut sum = 0;
    for v in &a {
        sum += v * hm.get(v).unwrap_or(&0)
    }

    println!("{sum}")
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
