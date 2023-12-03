use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed reading file");
    let sum: u32 = contents.lines().map(|line| {
        let mut ret = line.replace("one", "one1one");
        ret = ret.replace("two", "two2two");
        ret = ret.replace("three", "three3three");
        ret = ret.replace("four", "four4four");
        ret = ret.replace("five", "five5five");
        ret = ret.replace("six", "six6six");
        ret = ret.replace("seven", "seven7seven");
        ret = ret.replace("eight", "eight8eight");
        ret = ret.replace("nine", "nine9nine");
        return ret;
    }).map(|line| {
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
