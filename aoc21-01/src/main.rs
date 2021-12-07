// Day 1: Sonar Sweep

fn part1() {
    let input = include_str!("input.txt");
    let mut prev = std::i32::MAX;

    let matches = input.lines()
    .map(|x| x.parse::<i32>().unwrap())
    .filter(|x| {
        let cond = x > &prev;
        prev = *x;
        return cond;
    });

    println!("{}", matches.count());
}

fn part2() {
    let input = include_str!("input.txt").lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut prev_sum = 0;
    let mut increases = 0;

    for i in 2..input.len() {
        let sum = input[i - 2 as usize] + input[i - 1 as usize] + input[i];

        if prev_sum > 0 && sum > prev_sum {
            increases += 1;
        }
        prev_sum = sum;
    }

    println!("{}", increases);
}

fn main() {
    part1();
    part2();
}
