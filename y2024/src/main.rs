use std::env;

mod day1;
mod day2;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    assert_eq!(args.len(), 2);
    match args[1].as_str() {
        "1" => day1::day1(),
        "1.2" => day1::part2(),
        "2" => day2::day2(),
        "2.2" => day2::part2(),
        _ => println!("Unimplemented day: {}", args[1])
    }
}
