fn part1(instructions: &Vec<(&str, i32)>) -> i32 {
    let horizontal: i32 = instructions
        .iter()
        .filter(|(command, _)| command == &"forward")
        .map(|(_, val)| val)
        .sum();

    let vertical: i32 = instructions
        .iter()
        .map(|(command, val)| match command {
            &"up" => -1 * val,
            &"down" => 1 * val,
            _ => 0,
        })
        .sum();

    vertical * horizontal
}

fn part2(instructions: &Vec<(&str, i32)>) -> i32 {
    let mut aim = 0;
    let mut vertical = 0;
    let mut horizontal = 0;

    for (command, val) in instructions {
        match command {
            &"forward" => {
                vertical += val;
                horizontal += val * aim
            }
            &"up" => aim -= val,
            &"down" => aim += val,
            _ => {}
        }
    }

    vertical * horizontal
}
fn main() {
    let instructions = include_str!("../../data/day2.txt")
        .split('\n')
        .map(|x| x.split(' ').collect())
        .map(|x: Vec<&str>| (x[0], x[1].parse::<i32>().unwrap()))
        .collect();

    println!("Part 1: {:?}", part1(&instructions));
    println!("Part 2: {:?}", part2(&instructions));
}
