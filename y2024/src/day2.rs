use std::fs;

#[derive(Debug)]
enum Mode {
    Uninit,
    Init(i32),
    Inc(i32),
    Dec(i32),
}

pub fn part2() {
    let n_safe = fs::read_to_string("inputs/day2.txt").expect("a file")
        .lines()
        .filter(|line| line.len() > 3)
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|line| {
            'outer: for ignore_idx in 0..line.len() {
                let mut mode = Mode::Uninit;
                let mut idx = 0;
                for part in &line {
                    if idx == ignore_idx {
                        idx += 1;
                        continue;
                    }
                    idx += 1;
                    let part = part.parse::<i32>().expect("a number");
                    mode = match mode {
                        Mode::Uninit => Mode::Init(part),
                        Mode::Init(prev) => {
                            let delta = part - prev;
                            if delta == 0 || delta > 3 || delta < -3 {
                                continue 'outer;
                            } 

                            if delta > 0 {
                                Mode::Inc(part)
                            } else {
                                Mode::Dec(part)
                            }
                        },
                        Mode::Inc(prev) => {
                            let delta = part - prev;
                            if delta <= 0 || delta > 3 {
                                continue 'outer;
                            }

                            Mode::Inc(part)
                        },
                        Mode::Dec(prev) => {
                            let delta = part - prev;
                            if delta >= 0 || delta < -3 {
                                    continue 'outer;
                            }

                            Mode::Dec(part)
                        }
                    }
                }
                return 1;
            }
            return 0;
        }).sum::<i32>();
    println!("{n_safe}");
}

pub fn day2() {
    let n_safe = fs::read_to_string("inputs/day2.txt").expect("a file")
        .lines()
        .filter(|line| line.len() > 3)
        .map(|line| {
            let mut mode = Mode::Uninit;
            for part in line.split_whitespace() {
                let part = part.parse::<i32>().expect("a number");
                mode = match mode {
                    Mode::Uninit => Mode::Init(part),
                    Mode::Init(prev) => {
                        let delta = part - prev;
                        if delta == 0 || delta > 3 || delta < -3 {
                            return 0;
                        }
                        if delta > 0 {
                            Mode::Inc(part)
                        } else {
                            Mode::Dec(part)
                        }
                    },
                    Mode::Inc(prev) => {
                        let delta = part - prev;
                        if delta <= 0 || delta > 3 {
                            return 0;
                        }
                        Mode::Inc(part)
                    },
                    Mode::Dec(prev) => {
                        let delta = part - prev;
                        if delta >= 0 || delta < -3 {
                            return 0;
                        }
                        Mode::Dec(part)
                    }
                }
            }
            return 1;
        }).sum::<i32>();
    println!("{n_safe}");
}
