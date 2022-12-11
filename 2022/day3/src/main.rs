use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_score(item_type: char) -> u64 {
    match item_type {
        'a'..='z' => ((item_type as u8)- ('a' as u8) + 1) as u64,
        'A'..='Z' => ((item_type as u8)- ('A' as u8) + 27) as u64,
        _ => 0u64,
    }
}

fn day3() -> Result<u64, Box<dyn std::error::Error>>{
    Ok(fs::read_to_string("input.txt")?.lines().map(|line| {
        let bytes = line.as_bytes();
        let pivot = bytes.len()>>1;
        (&bytes[..pivot], &bytes[pivot..])
    }).map(|rucksack| {
        let mut hm = HashMap::new();
        for &item_type in rucksack.0 {
            hm.insert(item_type, ());
        }
        for &item_type in rucksack.1 {
            match hm.get(&item_type) {
                Some(_) => return Some(item_type as char),
                None => continue,
            }
        }
        None
    }).map(|item_type_opt| {
        match item_type_opt {
            Some(item_type) => get_score(item_type),
            None => 0u64
        }
    }).sum())
}

fn day3_2() -> Result<u64, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string("input.txt")?.lines().map(|line| {
        line.chars().collect::<HashSet<char>>()
    }).collect::<Vec<HashSet<char>>>().chunks(3).map(|chunk| {
        let i1 = chunk[0].intersection(&chunk[1]).map(|&c| { c }).collect::<HashSet<char>>();
        i1.intersection(&chunk[2]).map(|&c| { c } ).collect::<HashSet<char>>()
    }).map(|set| {
        get_score(set.iter().map(|&c| { c }).collect::<Vec<char>>().pop().unwrap())
    }).sum())
}

fn main() {
    println!("{:?}", day3_2());
    println!("Hello, world!");
}
