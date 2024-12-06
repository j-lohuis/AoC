use std::{
    collections::HashSet,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn rot90(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn movement(&self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
        }
    }
}

fn walk(
    board: &[Vec<u8>],
    start: (isize, isize),
    dims: (isize, isize),
    dir: Dir,
    mut cb: impl FnMut(isize, isize, Dir) -> bool,
) {
    let (height, width) = dims;
    let (mut y, mut x) = start;
    let mut dir = dir;
    loop {
        let (dy, dx) = dir.movement();
        let next = (y + dy, x + dx);

        if next.0 < 0 || next.0 >= height || next.1 < 0 || next.1 >= width {
            break;
        }

        if board[next.0 as usize][next.1 as usize] == b'#' {
            dir = dir.rot90();
        } else {
            (y, x) = next;
            if cb(y, x, dir) {
                break;
            }
        }
    }
}

fn part1(input: &str) {
    let board: Vec<Vec<u8>> = input
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|line| {
            if line.is_empty() {
                Vec::new()
            } else {
                line.bytes().collect()
            }
        })
        .collect();

    let (height, width) = (board.len(), board[0].len());
    let mut visited = vec![vec![false; width]; height];
    let (height, width) = (height as isize, width as isize);

    // Start position.
    let (x, y) = {
        let (mut rx, mut ry) = (0, 0);
        'outer: for (y, l) in board.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                if *c == b'^' {
                    rx = x;
                    ry = y;
                    break 'outer;
                }
            }
        }
        (rx, ry)
    };

    visited[y][x] = true;

    walk(
        &board,
        (y as isize, x as isize),
        (height, width),
        Dir::Up,
        |y, x, _| {
            visited[y as usize][x as usize] = true;
            false
        },
    );

    let num_visited: usize = visited
        .iter()
        .map(|r| r.iter().filter(|b| **b).count())
        .sum();

    println!("{num_visited}")
}

fn part2(input: &str) {
    let board: Vec<Vec<u8>> = input
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|line| {
            if line.is_empty() {
                Vec::new()
            } else {
                line.bytes().collect()
            }
        })
        .collect();

    let (height, width) = (board.len() as isize, board[0].len() as isize);

    // Start position.
    let (x, y) = {
        let (mut rx, mut ry) = (0, 0);
        'outer: for (y, l) in board.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                if *c == b'^' {
                    rx = x;
                    ry = y;
                    break 'outer;
                }
            }
        }
        (rx, ry)
    };

    let (start_y, start_x) = (y as isize, x as isize);

    let mut cycles = 0;
    let mut positions = HashSet::<(isize, isize)>::new();
    for y in 0..height {
        for x in 0..width {
            if board[y as usize][x as usize] != b'.' {
                continue;
            }

            let mut path = HashSet::<(isize, isize, Dir)>::new();
            let mut new_board = board.clone();

            new_board[y as usize][x as usize] = b'#';

            walk(
                &new_board,
                (start_y, start_x),
                (height, width),
                Dir::Up,
                |ny, nx, ndir| {
                    if path.contains(&(ny, nx, ndir)) {
                        positions.insert((y, x));
                        cycles += 1;
                        true
                    } else {
                        path.insert((ny, nx, ndir));
                        false
                    }
                },
            );
        }
    }

    println!("{cycles}")
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
