use std::fs;

fn range_contains(first: &[u64; 2], other: &[u64; 2]) -> bool {
    first[0] <= other[0] && first[1] >= other[1]
}

fn either_contains(first: &[u64; 2], other: &[u64; 2]) -> bool {
    range_contains(first, other) || range_contains(other, first)
}

fn inside_range(range: &[u64; 2], item: u64) -> bool {
    item >= range[0] && item <= range[1]
}

fn range_overlaps(first: &[u64; 2], other: &[u64; 2]) -> bool {
    inside_range(first, other[0]) || inside_range(first, other[1]) || inside_range(other, first[0]) || inside_range(other, first[1])
}

fn day4(reducer: fn(&[u64; 2], &[u64; 2]) -> bool) -> u64 {
    fs::read_to_string("input.txt").unwrap().lines().map(|line| {
        // Split lines in half
        line.split(",").collect::<Vec<&str>>()
    }).map(|pairs| {
        pairs.iter().map(|item_range| {
            item_range.split("-").collect::<Vec<&str>>()
        }).collect::<Vec<Vec<&str>>>()
    }).map(|pair_ranges| {
        pair_ranges.iter().map(|range| {
            range.iter().map(|bound| {
                bound.parse::<u64>().unwrap()
            }).collect::<Vec<u64>>()
        }).collect::<Vec<Vec<u64>>>()
    }).map(|ranges| {
        if reducer(ranges[0][..2].try_into().unwrap(), ranges[1][..2].try_into().unwrap()) {
            1
        } else { 0 }
    }).sum()
}

fn main() {
    println!("{:?}", day4(range_overlaps));
    println!("Hello, world!");
}
