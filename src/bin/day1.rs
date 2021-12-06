fn part1(measurements: &Vec<i32>) -> i32 {
    measurements.windows(2).map(|x| (x[1] > x[0]) as i32).sum()
}

fn part2(measurements: &Vec<i32>) -> i32 {
    let window_sums = measurements.windows(3).map(|x| x.iter().sum()).collect();
    part1(&window_sums)
}

fn main() {
    let measurements = include_str!("../../data/day1.txt")
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("Part 1: {:?}", part1(&measurements));
    println!("Part 2: {:?}", part2(&measurements));
}
