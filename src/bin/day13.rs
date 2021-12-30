use std::collections::HashSet;

fn part1(dots: &[(i64, i64)]) -> usize {
    let dots_folded: HashSet<(i64, i64)> = HashSet::from_iter(
        dots.iter()
            .map(|&(x, y)| (y, if x < 655 { x } else { 2 * 655 - x })),
    );
    dots_folded.len()
}

fn part2(dots: &[(i64, i64)]) {
    let folds = vec![
        (1, 655),
        (0, 447),
        (1, 327),
        (0, 223),
        (1, 163),
        (0, 111),
        (1, 81),
        (0, 55),
        (1, 40),
        (0, 27),
        (0, 13),
        (0, 6),
    ];
    let mut dots: HashSet<(i64, i64)> = HashSet::from_iter(dots.iter().cloned());
    for (axis, by) in folds {
        if axis == 1 {
            dots = HashSet::from_iter(
                dots.iter()
                    .map(|&(x, y)| (if x < by { x } else { 2 * by - x }, y)),
            );
        } else {
            dots = HashSet::from_iter(
                dots.iter()
                    .map(|&(x, y)| (x, if y < by { y } else { 2 * by - y })),
            );
        }
    }
    for i in 0..15 {
        for j in 0..40 {
            if dots.contains(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn main() {
    let dots: Vec<_> = include_str!("../../data/day13.txt")
        .split('\n')
        .map(|line| {
            let (y, x) = line.split_once(',').unwrap();
            (y.parse::<i64>().unwrap(), x.parse::<i64>().unwrap())
        })
        .collect();

    println!("{:?}", part1(&dots));
    part2(&dots);
}
