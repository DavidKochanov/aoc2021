use std::collections::HashSet;

// use std::collections::HashSet;

struct HeightMap {
    heights: Vec<Vec<u32>>,
    height: isize,
    width: isize,
}

impl HeightMap {
    fn new(h: Vec<Vec<u32>>) -> Self {
        Self {
            height: h.len() as isize,
            width: if h.is_empty() { 0 } else { h[0].len() as isize },
            heights: h,
        }
    }

    fn get_height(&self, i: isize, j: isize) -> u32 {
        self.heights[i as usize][j as usize]
    }

    fn is_valid(&self, i: isize, j: isize) -> bool {
        i >= 0 && i < self.height && j >= 0 && j < self.width
    }

    fn neighbors(&self, i: isize, j: isize) -> Vec<(isize, isize)> {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .map(|(di, dj)| (i + di, dj + j))
            .filter(|&(x, y)| self.is_valid(x, y))
            .collect()
    }

    fn get_low_points(&self) -> Vec<(isize, isize)> {
        let mut res = Vec::new();

        for i in 0..self.height {
            for j in 0..self.width {
                if self
                    .neighbors(i, j)
                    .iter()
                    .all(|&(k, l)| self.get_height(k, l) > self.get_height(i, j))
                {
                    res.push((i, j));
                }
            }
        }

        res
    }
}

fn part1(m: &HeightMap) -> u32 {
    m.get_low_points()
        .iter()
        .map(|&(x, y)| m.get_height(x, y) + 1)
        .sum()
}

fn part2(m: &HeightMap) -> usize {
    let mut sizes = Vec::new();
    for (s_i, s_j) in m.get_low_points() {
        let mut front = vec![(s_i, s_j)];
        let mut visited = HashSet::new();
        while !front.is_empty() {
            let (i, j) = front.pop().unwrap();
            visited.insert((i, j));
            for (k, l) in m.neighbors(i, j) {
                if !visited.contains(&(k, l))
                    && m.get_height(k, l) > m.get_height(i, j)
                    && m.get_height(k, l) != 9
                {
                    front.push((k, l));
                }
            }
        }
        sizes.push(visited.len());
    }
    sizes.sort_unstable();
    sizes[sizes.len() - 3..sizes.len()].iter().product()
}

fn main() {
    let m: Vec<Vec<u32>> = include_str!("../../data/day9.txt")
        .split('\n')
        .map(|x| {
            x.chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let m = HeightMap::new(m);

    println!("{:?}", part1(&m));
    println!("{:?}", part2(&m));
}
