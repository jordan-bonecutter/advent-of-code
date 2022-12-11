use std::fs;

fn find_max_group() -> Result<u64, Box<dyn std::error::Error>> {
    let mut calories = vec![];
    let mut cur = 0u64;
    for line in fs::read_to_string("input.txt")?.lines() {
        match line.chars().count() {
            0 => {
                calories.push(cur);
                cur = 0;
            }
            _ => {
                cur += line.parse::<u64>()?;
            }
        }
    }

    calories.sort();
    calories.reverse();
    Ok(calories.into_iter().take(3).sum())
}

fn main() {
    match find_max_group() {
        Ok(max) => println!("{}", max),
        Err(err) => println!("Failed finding max: {}", err),
    }
}
