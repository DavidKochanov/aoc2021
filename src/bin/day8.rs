use std::{
    collections::{BTreeSet, HashMap},
    panic,
};

fn part1(lines: &Vec<(&str, &str)>) -> usize {
    lines
        .iter()
        .map(|&(_, out)| {
            out.split_whitespace()
                .map(|x| x.len())
                .filter(|&x| x == 2 || x == 3 || x == 4 || x == 7)
                .count()
        })
        .sum()
}

fn part2(lines: &Vec<(&str, &str)>) -> usize {
    let mut total = 0;
    for &(inp, out) in lines.iter() {
        let parse_sets = |x: &str| {
            x.split_whitespace()
                .map(|x| BTreeSet::from_iter(x.chars().into_iter()))
                .collect()
        };
        let inp_sets: Vec<BTreeSet<char>> = parse_sets(inp);

        let mut encode = HashMap::new();
        // let mut decode = HashMap::new();

        //find 1, 4, 7 and 8
        for set in inp_sets.iter() {
            match set.len() {
                2 => encode.insert(1, set),
                3 => encode.insert(7, set),
                4 => encode.insert(4, set),
                7 => encode.insert(8, set),
                _ => None,
            };
        }

        for set in inp_sets.iter() {
            // find 0 6 9
            // |0| = 6, |6 & 4| = 3, |6 & 1| = 2
            // |6| = 6, |6 & 4| = 3, |6 & 1| = 1
            // |9| = 6, |9 & 4| = 4, |9 & 1| = 2
            if set.len() == 6 {
                match (
                    (set & encode.get(&4).unwrap()).len(),
                    (set & encode.get(&1).unwrap()).len(),
                ) {
                    (3, 2) => encode.insert(0, set),
                    (3, 1) => encode.insert(6, set),
                    (4, 2) => encode.insert(9, set),
                    _ => panic!("Oopps"),
                };

            // Find 2 3 5
            // |2| = 5, |2 & 7| = 2, |2 & 4| = 2
            // |3| = 5, |3 & 7| = 3, |3 & 4| = 3
            // |5| = 5, |5 & 7| = 2, |5 & 4| = 3
            } else if set.len() == 5 {
                match (
                    (set & encode.get(&7).unwrap()).len(),
                    (set & encode.get(&4).unwrap()).len(),
                ) {
                    (2, 2) => encode.insert(2, set),
                    (3, 3) => encode.insert(3, set),
                    (2, 3) => encode.insert(5, set),
                    _ => panic!("Ooppss"),
                };
            }
        }
        let decode: HashMap<&BTreeSet<_>, i32> =
            HashMap::from_iter(encode.into_iter().map(|(x, y)| (y, x)));

        let current_out = parse_sets(out)
            .iter()
            .fold(0, |acc, x| 10 * acc + decode.get(&x).unwrap());

        total += current_out as usize;
    }
    total
}
fn main() {
    let lines: Vec<(&str, &str)> = include_str!("../../data/day8.txt")
        .split('\n')
        .map(|x| x.split_once('|').unwrap())
        .collect();

    println!("{:?}", part1(&lines));
    println!("{:?}", part2(&lines));
}
