use regex::Regex;
use std::io::{self, Read};

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();

        sum += a * b;
    }

    println!("{sum}")
}

fn part2(input: &str) {
    let mut input = input;
    let mul_re = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"^do\(\)").unwrap();
    let dont_re = Regex::new(r"^don't\(\)").unwrap();

    let mut sum = 0;
    let mut active = true;
    let len = input.len();
    for _ in 0..len {
        if let Some(m) = mul_re.captures(input) {
            if active {
                let a = m[1].parse::<usize>().unwrap();
                let b = m[2].parse::<usize>().unwrap();

                sum += a * b;
            }
        }

        if do_re.is_match(input) {
            active = true;
        }
        if dont_re.is_match(input) {
            active = false;
        }

        input = &input[1..];
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
