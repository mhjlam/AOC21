// Day 3: Binary Diagnostic

fn part1() -> u32 {
    const BIT_LEN : usize = 12;
    const NUM_LINES : usize = 1000;

    let mut rate = vec![0; BIT_LEN];
    let mut gamma_rate = 0b000000000000;

    // Determine most common bit in each position
    include_str!("day03")
    .lines()
    .for_each(|line| {
        for (i, c) in line.chars().enumerate() {
            rate[i] += c.to_digit(2).unwrap();
        }
    });

    // Set gamma rate bit based on most common bit in the corresponding position
    for (i, x) in rate.iter().enumerate() {
        if x >= &((NUM_LINES/2) as u32) {
            gamma_rate |= 1 << (BIT_LEN -1 - i);
        }
    }

    // Epsilon rate is the inverse of the gamma rate
    let epsilon_rate = !gamma_rate & 0b111111111111;
    return (gamma_rate * epsilon_rate) as u32;
}

fn part2() -> u32 {
    return 0;
}

fn main() {
    println!("{} {}", part1(), part2());
}
