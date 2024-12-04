use std::io::{self, Read};

fn part1(input: &str) {
    let mut board: Vec<Vec<u8>> = (0..3)
        .map(|_| "")
        .chain(input.split('\n'))
        .chain((0..2).map(|_| ""))
        .map(|line| {
            if line.is_empty() {
                Vec::new()
            } else {
                (0..3).chain(line.bytes()).chain(0..3).collect()
            }
        })
        .collect();

    board[0] = vec![0; board[3].len()];
    board[1] = vec![0; board[3].len()];
    board[2] = vec![0; board[3].len()];

    let l = board.len();
    board[l - 1] = vec![0; board[3].len()];
    board[l - 2] = vec![0; board[3].len()];
    board[l - 3] = vec![0; board[3].len()];

    let mut count = 0usize;
    for y in 3..l - 3 {
        for x in 3..board[y].len() - 3 {
            if board[y][x] == b'X' {
                let mas = (b'M', b'A', b'S');
                if (board[y][x - 1], board[y][x - 2], board[y][x - 3]) == mas {
                    count += 1;
                }
                if (board[y][x + 1], board[y][x + 2], board[y][x + 3]) == mas {
                    count += 1;
                }
                if (board[y - 1][x], board[y - 2][x], board[y - 3][x]) == mas {
                    count += 1;
                }
                if (board[y + 1][x], board[y + 2][x], board[y + 3][x]) == mas {
                    count += 1;
                }
                if (
                    board[y - 1][x - 1],
                    board[y - 2][x - 2],
                    board[y - 3][x - 3],
                ) == mas
                {
                    count += 1;
                }
                if (
                    board[y + 1][x + 1],
                    board[y + 2][x + 2],
                    board[y + 3][x + 3],
                ) == mas
                {
                    count += 1;
                }
                if (
                    board[y - 1][x + 1],
                    board[y - 2][x + 2],
                    board[y - 3][x + 3],
                ) == mas
                {
                    count += 1;
                }
                if (
                    board[y + 1][x - 1],
                    board[y + 2][x - 2],
                    board[y + 3][x - 3],
                ) == mas
                {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}

fn part2(input: &str) {
    let board: Vec<Vec<u8>> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            if line.is_empty() {
                Vec::new()
            } else {
                line.bytes().collect()
            }
        })
        .collect();

    let mut count = 0usize;
    for y in 1..board.len() - 1 {
        for x in 1..board[y].len() - 1 {
            if board[y][x] == b'A' {
                let ul_m = board[y - 1][x - 1] == b'M';
                let ul_s = board[y - 1][x - 1] == b'S';
                let ur_m = board[y - 1][x + 1] == b'M';
                let ur_s = board[y - 1][x + 1] == b'S';
                let dl_m = board[y + 1][x - 1] == b'M';
                let dl_s = board[y + 1][x - 1] == b'S';
                let dr_m = board[y + 1][x + 1] == b'M';
                let dr_s = board[y + 1][x + 1] == b'S';

                let matches = matches!(
                    (ul_m, ul_s, dr_m, dr_s),
                    (true, false, false, true) | (false, true, true, false)
                ) && matches!(
                    (ur_m, ur_s, dl_m, dl_s),
                    (true, false, false, true) | (false, true, true, false)
                );
                count += matches as usize;
            }
        }
    }

    println!("{count}");
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
