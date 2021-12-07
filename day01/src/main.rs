// Day 1: Sonar Sweep

fn part1() -> i32 {
    let mut prev = i32::MAX;

    return include_str!("input.txt")
    .lines()
    .map(|x| x.parse::<i32>().unwrap())
    .filter(|x| {
        let cond = x > &prev;
        prev = *x;
        return cond;
    }).count() as i32;
}

fn part2() -> i32 {
    let mut prev_sum = i32::MAX;
    let mut increases = 0;

    include_str!("input.txt")
    .lines()
    .map(|x| x.parse::<i32>().unwrap())
    .collect::<Vec<i32>>()
    .windows(3)
    .for_each(|x| {
        let sum = x.iter().sum::<i32>();
        if sum > prev_sum {
            increases += 1;
        }
        prev_sum = sum;
    });
    return increases;
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
