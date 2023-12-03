fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();

    println!("{}", advent(&file));
}

fn advent(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                sum += digit * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                sum += digit;
                break;
            }
        }
    }

    sum
}
