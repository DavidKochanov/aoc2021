use core::cmp::min;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn part1(levels: &Vec<Vec<u32>>) -> u32 {
    let mut levels = levels.clone();
    for i in 1..levels.len() {
        levels[i][0] += levels[i - 1][0];
    }
    for i in 1..levels[0].len() {
        levels[0][i] += levels[0][i - 1];
    }
    for i in 1..levels.len() {
        for j in 1..levels[0].len() {
            levels[i][j] += min(levels[i - 1][j], levels[i][j - 1]);
        }
    }
    levels[levels.len() - 1][levels[0].len() - 1] - levels[0][0]
}

fn part2(levels: &Vec<Vec<u32>>) -> u32 {
    let cost = |i: isize, j: isize| -> u32 {
        if i < 0 || j < 0 || i as usize >= 5 * levels.len() || j as usize >= 5 * levels[0].len() {
            return 10000000;
        }
        let c = levels[i as usize % levels.len()][j as usize % levels[0].len()]
            + (i as usize / levels.len()) as u32
            + (j as usize / levels[0].len()) as u32;
        let c = if c >= 10 { 1 + c } else { c };
        let c = if c >= 20 { c + 1 } else { c };

        c % 10
    };

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push((Reverse(0), 0, 0));

    while let Some((Reverse(current_cost), i, j)) = heap.pop() {
        // dbg!(current_cost, i, j);
        if i as usize == 5 * levels.len() - 1 && j as usize == 5 * levels[0].len() - 1 {
            return current_cost;
        }
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));
        if !visited.contains(&(i + 1, j)) {
            heap.push((Reverse(current_cost + cost(i + 1, j)), i + 1, j));
        }
        if !visited.contains(&(i, j + 1)) {
            heap.push((Reverse(current_cost + cost(i, j + 1)), i, j + 1));
        }
        if !visited.contains(&(i, j - 1)) {
            heap.push((Reverse(current_cost + cost(i, j - 1)), i, j - 1));
        }
        if !visited.contains(&(i - 1, j)) {
            heap.push((Reverse(current_cost + cost(i - 1, j)), i - 1, j));
        }
    }
    0
}

fn main() {
    let levels: Vec<Vec<u32>> = include_str!("../../data/day15.txt")
        .split('\n')
        .map(|x| {
            x.chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    println!("{:?}", part1(&levels));
    println!("{:?}", part2(&levels));
}
