use std::fs;
use std::collections::HashMap;

pub fn input() -> (Vec<i32>, Vec<i32>) {
    let data = fs::read_to_string("inputs/day1.txt").expect("a string");
    let (mut v1, mut v2) = data.lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(p1, p2)| (p1.trim(), p2.trim()))
        .map(|(p1, p2)| (p1.parse::<i32>().expect("to parse"), p2.parse::<i32>().expect("to parse")))
        .fold((Vec::<i32>::new(), Vec::<i32>::new()), |(mut v1, mut v2), (n1, n2)| {
            v1.push(n1);
            v2.push(n2);
            return (v1, v2);
        });
    v1.sort();
    v2.sort();
    return (v1, v2);
}

pub fn part2() {
    let (v1, v2) = input();
    let counts = v2.into_iter().fold(HashMap::<i32, i32>::new(), |mut m, n| {
        match m.get_mut(&n) {
            Some(count) => {
                *count = *count+1
            },
            None => {
                m.insert(n, 1);
            },
        }
        return m;
    });
    let similarity = v1.into_iter().fold(0u64, |score, n| score + ((n*counts.get(&n).cloned().unwrap_or(0)) as u64));
    println!("{similarity}");
}

pub fn day1() {
    let (v1, v2) = input();
    let distance = std::iter::zip(v1, v2)
        .fold(0u64, |dist, (a, b)| dist + ((a - b).abs() as u64));
    println!("{distance}");
}
