use std::collections::HashMap;

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    println!("{}", advent(&file));
}

fn advent(input: &str) -> u32 {
    let mut res: u32 = 0;

    for line in input.lines() {
        let mut counts = HashMap::new();
        counts.insert("red", 0);
        counts.insert("green", 0);
        counts.insert("blue", 0);

        let line = line.replace(":", " ").replace(",", " ").replace(";", " ");

        let id = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u8>()
            .unwrap();

        let color_pairs = line.split_whitespace().skip(2).collect::<Vec<&str>>();

        for pair in color_pairs.chunks(2) {
            if pair.len() == 2 {
                if let Ok(count) = pair[0].parse::<u8>() {
                    let color = pair[1];
                    let entry = counts.entry(color).or_insert(0);
                    *entry = (*entry).max(count);
                }
            }
        }

        let red = *counts.get("red").unwrap();
        let green = *counts.get("green").unwrap();
        let blue = *counts.get("blue").unwrap();

        if red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE {
            res += id as u32;
        }
    }

    res
}
