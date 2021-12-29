use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<Vec<u32>>,
    height: isize,
    width: isize,
}
impl Grid {
    fn new(g: Vec<Vec<u32>>) -> Self {
        Self {
            height: g.len() as isize,
            width: if g.is_empty() { 0 } else { g[0].len() as isize },
            grid: g,
        }
    }

    fn is_valid(&self, cell: (isize, isize)) -> bool {
        let (i, j) = cell;
        i >= 0 && i < self.height && j >= 0 && j < self.width
    }

    fn neighbors(&self, cell: (isize, isize)) -> Vec<(isize, isize)> {
        let (i, j) = cell;
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|(di, dj)| (i + di, dj + j))
        .filter(|&(x, y)| self.is_valid((x, y)))
        .collect()
    }

    fn inc(&mut self, cell: (isize, isize)) {
        let (i, j) = cell;
        self.grid[i as usize][j as usize] = (self.grid[i as usize][j as usize] + 1) % 10;
    }

    fn step(&mut self) -> usize {
        let mut flashing = Vec::new();
        let mut total = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                self.inc((i, j));
                if self.grid[i as usize][j as usize] == 0 {
                    flashing.push((i as isize, j as isize));
                    total += 1;
                }
            }
        }
        let mut flashed = HashSet::new();
        while let Some(c) = flashing.pop() {
            flashed.insert(c);
            for (i, j) in self.neighbors(c) {
                if self.grid[i as usize][j as usize] != 0 {
                    self.inc((i, j));
                    if self.grid[i as usize][j as usize] == 0 {
                        flashing.push((i as isize, j as isize));
                        total += 1;
                    }
                }
            }
        }
        total
    }
}

fn part1(octs: &Grid) -> i64 {
    let mut octs = octs.clone();
    let mut total = 0;
    for _ in 0..100 {
        total += octs.step() as i64;
    }
    total
}

fn part2(octs: &Grid) -> i64 {
    let mut octs = octs.clone();
    let all = octs.width * octs.height;
    let mut count = 1;
    loop {
        if octs.step() == all as usize {
            break;
        }
        count += 1;
    }
    count
}
fn main() {
    let octs: Vec<Vec<u32>> = include_str!("../../data/day11.txt")
        .split('\n')
        .map(|x| {
            x.chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let octs = Grid::new(octs);
    println!("{:?}", part1(&octs));
    println!("{:?}", part2(&octs));
}
