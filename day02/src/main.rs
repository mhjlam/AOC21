// Day 2: Dive!

fn part1() -> i32 {
    let mut position = 0;
    let mut depth = 0;

    include_str!("input.txt")
    .lines()
    .for_each(|line| {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<i32>().unwrap();
        match direction {
            "forward" => position += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            other => panic!("Error: {}", other)
        }
    });
    return position * depth;
}

fn part2() -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    include_str!("input.txt")
    .lines()
    .for_each(|line| {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<i32>().unwrap();
        match direction {
            "forward" => {
                position += amount;
                depth += aim * amount;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            other => panic!("Error: {}", other)
        }
    });
    return position * depth;
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
