fn part1(crabs: &[i32]) -> i32 {
    let min = *crabs.iter().min().unwrap() as usize;
    let max = *crabs.iter().max().unwrap() as usize;

    (min..max)
        .map(|x| crabs.iter().map(|c| (x as i32 - c).abs()).sum())
        .min()
        .unwrap()
}

fn part2(crabs: &[i32]) -> i32 {
    let min = *crabs.iter().min().unwrap() as usize;
    let max = *crabs.iter().max().unwrap() as usize;

    (min..max)
        .map(|x| {
            crabs
                .iter()
                .map(|c| {
                    let diff = (x as i32 - c).abs();
                    (diff * (diff + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    let crabs: Vec<i32> = include_str!("../../data/day7.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", part1(&crabs));
    println!("{:?}", part2(&crabs));
}
