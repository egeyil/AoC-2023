use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("input1.txt not found");

    let res = process(&input);

    println!("{res}");
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;

            let mut res = String::new();

            for c in line.chars() {
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = Some(c);
                        continue;
                    }
                    last = Some(c);
                }
            }

            if let Some(c) = first {
                res.push(c);
                if last.is_none() {
                    res.push(c);
                }
            }
            if let Some(c) = last {
                res.push(c);
            }
            res.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, process(test_input));
    }
}