// Day 2: Dive!

fn part1() -> i32 {
    let mut position = 0;
    let mut depth = 0;

    include_str!("input.txt")
    .lines()
    .for_each(|l| {
        let s = l.split_whitespace().collect::<Vec<&str>>();
        let x = s[1].parse::<i32>().unwrap();
        match s[0] {
            "forward" => position += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => panic!("clutch towel")
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
    .for_each(|l| {
        let s = l.split_whitespace().collect::<Vec<&str>>();
        let x = s[1].parse::<i32>().unwrap();
        match s[0] {
            "forward" => {
                position += x;
                depth += aim * x;
            },
            "down" => aim += x,
            "up" => aim -= x,
            _ => panic!("oh noo")
        }
    });
    return position * depth;
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
