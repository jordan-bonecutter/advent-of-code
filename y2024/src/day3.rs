use std::fs;
use regex::Regex;

enum Instr {
    Do,
    Dont,
    Mul(i16, i16)
}

pub fn part2() {
    let re = Regex::new(r"(?m)(mul\(\d+,\d+\)|do\(\)|don't\(\))").expect("to compile");
    let data = fs::read_to_string("inputs/day3.txt").expect("to read a file");
    let (_, sum) = re.find_iter(&data)
        .map(|m| m.as_str())
        .map(|s| {
            match s {
                "do()" => Instr::Do,
                "don't()" => Instr::Dont,
                _ => {
                    let (a, b) = (&s[4..s.len()-1]).split_once(',').expect("to split");
                    Instr::Mul(
                        a.parse::<i16>().expect("to parse"),
                        b.parse::<i16>().expect("to parse"),
                    )
                }
            }
        })
        .fold((true, 0u64), |(enabled, sum), instr| {
            match instr {
                Instr::Do => (true, sum),
                Instr::Dont => (false, sum),
                Instr::Mul(a, b) => {
                    if enabled {
                        (true, sum + (a as u64 * b as u64))
                    } else {
                        (false, sum)
                    }
                }
            }
        });
    println!("{sum}");
}

pub fn day3() {
    let re = Regex::new(r"(?m)mul\(\d+,\d+\)").expect("to compile");
    let data = fs::read_to_string("inputs/day3.txt").expect("to read a file");
    let sum = re.find_iter(&data)
        .map(|m| m.as_str())
        .map(|s| &s[4..s.len()-1])
        .map(|s| s.split_once(',').expect("to split"))
        .map(|(l, r)| (l.parse::<i16>().expect("to parse"), r.parse::<i16>().expect("to parse")))
        .map(|(l, r)| (l as u64) * (r as u64))
        .sum::<u64>();
    println!("{sum}");
}
