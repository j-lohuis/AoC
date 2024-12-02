use std::io::{self, Read};

fn check_report(report: &[i32]) -> bool {
    let asc = report[0] < report[1];

    for win in report.windows(2) {
        if win[0] < win[1] && !asc {
            return false;
        }
        if win[0] > win[1] && asc {
            return false;
        }

        match win[0].abs_diff(win[1]) {
            1..=3 => {}
            _ => return false,
        }
    }

    true
}

fn part1(input: &str) {
    let valids: usize = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|report| check_report(&report) as usize)
        .sum();

    println!("{valids}")
}

fn part2(input: &str) {
    let valids: usize = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|report| {
            let safe = if check_report(&report) {
                true
            } else {
                (0..report.len()).any(|skip| {
                    let mut new_report = vec![];
                    for (i, e) in report.iter().enumerate() {
                        if i != skip {
                            new_report.push(*e)
                        }
                    }

                    check_report(&new_report)
                })
            };

            safe as usize
        })
        .sum();

    println!("{valids}")
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
