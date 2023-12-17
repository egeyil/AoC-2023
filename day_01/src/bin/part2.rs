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

            let mut it = (0..line.len()).filter_map(|index| {
                // println!("{}", &line[index..]);
                match &line[index..] {
                    line if line.starts_with("one") => Some('1'),
                    line if line.starts_with("two") => Some('2'),
                    line if line.starts_with("three") => Some('3'),
                    line if line.starts_with("four") => Some('4'),
                    line if line.starts_with("five") => Some('5'),
                    line if line.starts_with("six") => Some('6'),
                    line if line.starts_with("seven") => Some('7'),
                    line if line.starts_with("eight") => Some('8'),
                    line if line.starts_with("nine") => Some('9'),
                    line => {
                        // let mut res: Option<char> = None;
                        // for c in line.chars() {
                        //     if c.is_ascii_digit() {
                        //         res = Some(c);
                        //     }
                        // }
                        //
                        // res
                        line.chars().next()
                    }
                }
            });

            first = it.next();
            last = it.last();


            if let Some(c) = first {
                res.push(c);
                if last.is_none() {
                    res.push(c);
                }
            }
            if let Some(c) = last {
                res.push(c);
            }
            // println!("{res}");
            res.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part_2() {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, process(test_input));
    }
}