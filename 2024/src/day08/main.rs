use std::{
    array,
    io::{self, Read},
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Vec2d {
    x: i32,
    y: i32,
}

impl Vec2d {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Vec2d) -> Self::Output {
        let mut t = self;
        t += rhs;
        t
    }
}

impl SubAssign for Vec2d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Vec2d) -> Self::Output {
        let mut t = self;
        t -= rhs;
        t
    }
}

impl Mul<i32> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Vec2d {
    type Output = Vec2d;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

fn oob(point: Vec2d, dims: Vec2d) -> bool {
    !((0..dims.x).contains(&point.x) && (0..dims.y).contains(&point.y))
}

fn part1(input: &str) {
    let mut dims = Vec2d::new(0, 0);
    let mut antennas: [Vec<_>; 256] = array::from_fn(|_| vec![]);
    for (y, l) in input.split('\n').take_while(|l| !l.is_empty()).enumerate() {
        dims.x = l.len() as i32;
        for (x, c) in l.bytes().enumerate() {
            if c != b'.' {
                antennas[c as usize].push(Vec2d::new(x as i32, y as i32));
            }
        }
        dims.y += 1;
    }

    let mut board = vec![vec![false; dims.y as usize]; dims.x as usize];

    for antennas in antennas.iter().filter(|a| !a.is_empty()) {
        for (i, a) in antennas.iter().copied().enumerate() {
            for b in antennas.iter().copied().skip(i + 1) {
                let anti_a = a + (a - b);
                let anti_b = b + (b - a);
                if !oob(anti_a, dims) {
                    board[anti_a.x as usize][anti_a.y as usize] = true;
                }
                if !oob(anti_b, dims) {
                    board[anti_b.x as usize][anti_b.y as usize] = true;
                }
            }
        }
    }

    let num_anti: usize = board.iter().map(|r| r.iter().filter(|b| **b).count()).sum();

    println!("{num_anti}");
}

fn part2(input: &str) {
    let mut dims = Vec2d::new(0, 0);
    let mut antennas: [Vec<_>; 256] = array::from_fn(|_| vec![]);
    for (y, l) in input.split('\n').take_while(|l| !l.is_empty()).enumerate() {
        dims.x = l.len() as i32;
        for (x, c) in l.bytes().enumerate() {
            if c != b'.' {
                antennas[c as usize].push(Vec2d::new(x as i32, y as i32));
            }
        }
        dims.y += 1;
    }

    let mut board = vec![vec![false; dims.y as usize]; dims.x as usize];

    for antennas in antennas.iter().filter(|a| !a.is_empty()) {
        for (i, a) in antennas.iter().copied().enumerate() {
            for b in antennas.iter().copied().skip(i + 1) {
                let mut anti_a = a;
                while !oob(anti_a, dims) {
                    board[anti_a.x as usize][anti_a.y as usize] = true;
                    anti_a += a - b;
                }
                let mut anti_b = b;
                while !oob(anti_b, dims) {
                    board[anti_b.x as usize][anti_b.y as usize] = true;
                    anti_b += b - a;
                }
            }
        }
    }

    let num_anti: usize = board.iter().map(|r| r.iter().filter(|b| **b).count()).sum();

    println!("{num_anti}");
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
