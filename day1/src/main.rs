use std::error::Error;

const INPUT: &str = include_str!("./input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let mut elves = vec![];
    let mut current_sum = 0;

    for l in INPUT.lines() {
        if l.trim().is_empty() {
            elves.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += l.trim_end().parse::<u32>()?;
        }
    }
    elves.sort();

    println!("Part1: {}", elves.last().unwrap());

    let top3 = elves[elves.len() - 3..].iter().sum::<u32>();
    println!("Part2: {}", top3);

    Ok(())
}
