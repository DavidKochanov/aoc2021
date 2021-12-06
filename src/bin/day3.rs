fn count_ones(numbers: &Vec<&str>, bit: usize) -> i32 {
    numbers
        .iter()
        .map(|num| (num.chars().nth(bit).unwrap() == '1') as i32)
        .sum()
}

fn part1(numbers: &Vec<&str>) -> i32 {
    let mut gamma: u32 = 0;
    let mut eps: u32 = 0;
    for bit in 0..12 {
        let ones = count_ones(numbers, bit);
        if ones >= numbers.len() as i32 - ones {
            gamma |= 1 << bit;
        } else {
            eps |= 1 << bit;
        }
    }
    (gamma * eps) as i32
}

fn part2(numbers: &Vec<&str>) -> i32 {
    let mut oxygen_numbers = numbers.clone();
    let mut bit = 0;
    while oxygen_numbers.len() > 1 {
        let ones = count_ones(numbers, bit);
        let most_common = if ones >= oxygen_numbers.len() as i32 - ones {
            '1'
        } else {
            '0'
        };
        oxygen_numbers = oxygen_numbers
            .into_iter()
            .filter(|&num| num.chars().nth(bit).unwrap() == most_common)
            .collect();
        bit += 1;
    }
    let mut co2_numbers = numbers.clone();
    let mut bit = 0;
    while co2_numbers.len() > 1 {
        let ones = count_ones(numbers, bit);
        let most_common = if ones >= co2_numbers.len() as i32 - ones {
            '0'
        } else {
            '1'
        };
        co2_numbers = co2_numbers
            .into_iter()
            .filter(|&num| num.chars().nth(bit).unwrap() == most_common)
            .collect();
        bit += 1;
    }

    let oxygen = i32::from_str_radix(oxygen_numbers[0], 2).unwrap();
    let co2 = i32::from_str_radix(co2_numbers[0], 2).unwrap();
    oxygen * co2
}

fn main() {
    let numbers = include_str!("../../data/day3.txt").split('\n').collect();

    println!("Part 1: {:?}", part1(&numbers));
    println!("Part 2: {:?}", part2(&numbers));
}
