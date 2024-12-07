use std::io::{self, Read};

fn part1(input: &str) {
    let sum: u64 = input
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (result, calc) = l.split_once(':').unwrap();
            let calc = calc.trim();

            let calc = calc
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (result.parse::<u64>().unwrap(), calc)
        })
        .filter(|calc| {
            let len = calc.1.len() as u32;
            (0..(2usize.pow(len))).any(|pat| {
                let mut result = calc.1[0];
                for bit in 0..len - 1 {
                    if ((pat >> bit) & 1) != 0 {
                        result *= calc.1[(bit + 1) as usize];
                    } else {
                        result += calc.1[(bit + 1) as usize];
                    }
                }

                result == calc.0
            })
        })
        .map(|calc| calc.0)
        .sum();

    println!("{sum}")
}

fn next_pow10(n: u64) -> u64 {
    if n < 10 {
        return 10;
    }
    if n < 100 {
        return 100;
    }
    if n < 1000 {
        return 1000;
    }
    if n < 10000 {
        return 10000;
    }
    if n < 100000 {
        return 100000;
    }
    if n < 1000000 {
        return 1000000;
    }
    if n < 10000000 {
        return 10000000;
    }
    if n < 100000000 {
        return 100000000;
    }
    if n < 1000000000 {
        return 1000000000;
    }
    if n < 10000000000 {
        return 10000000000;
    }
    if n < 100000000000 {
        return 100000000000;
    }
    if n < 1000000000000 {
        return 1000000000000;
    }
    if n < 10000000000000 {
        return 10000000000000;
    }
    if n < 100000000000000 {
        return 100000000000000;
    }
    if n < 1000000000000000 {
        return 1000000000000000;
    }
    if n < 10000000000000000 {
        return 10000000000000000;
    }
    if n < 100000000000000000 {
        return 100000000000000000;
    }
    if n < 1000000000000000000 {
        return 1000000000000000000;
    }
    if n < 10000000000000000000 {
        return 10000000000000000000;
    }

    unreachable!()
}

fn part2(input: &str) {
    let sum: u64 = input
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (result, calc) = l.split_once(':').unwrap();
            let calc = calc.trim();

            let calc = calc
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (result.parse::<u64>().unwrap(), calc)
        })
        .filter(|calc| {
            let len = calc.1.len() as u32;
            (0..(3usize.pow(len))).any(|mut pat| {
                let mut result = calc.1[0];
                for op in 0..len - 1 {
                    let rhs = calc.1[(op + 1) as usize];
                    let calc = pat % 3;
                    pat /= 3;
                    match calc {
                        0 => result *= rhs,
                        1 => result += rhs,
                        2 => result = result * next_pow10(rhs) + rhs,
                        _ => unreachable!(),
                    }
                }

                result == calc.0
            })
        })
        .map(|calc| calc.0)
        .sum();

    println!("{sum}")
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
