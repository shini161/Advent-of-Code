use std::collections::VecDeque;

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    println!("{}", advent(&file));
}

fn advent(input: &str) -> u32 {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sol: u32 = 0;

    for line in input.lines() {
        let mut deque = VecDeque::new();
        let mut i = 0;

        while i < line.len() {
            if let Some(c) = line[i..].chars().next() {
                if c.is_digit(10) {
                    deque.push_back(c.to_digit(10).unwrap() as i64);
                    i += 1;
                } else {
                    for (j, n) in numbers.iter().enumerate() {
                        if line[i..].starts_with(n) {
                            deque.push_back(j as i64);
                            i += 1;
                            break;
                        }
                    }
                    i += 1;
                }
            }
        }

        if let (Some(first), Some(last)) = (deque.front(), deque.back()) {
            sol += (first * 10 + last) as u32;
        }
    }

    sol
}
