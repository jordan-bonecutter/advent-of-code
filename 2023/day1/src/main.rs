use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed reading file");
    let sum: u32 = contents.lines().map(|line| {
        if line.len() == 0 {
            return 0;
        }

        let mut first_digit = None;
        let mut last_digit = None;
        for c in line.chars() {
            if c.is_numeric() {
                if first_digit.is_none() {
                    first_digit = Some(c);
                }
                last_digit = Some(c);
            }
        }

        return (first_digit.unwrap().to_digit(10).unwrap() * 10) + last_digit.unwrap().to_digit(10).unwrap();
    }).sum();
    println!("{}", sum);
}
