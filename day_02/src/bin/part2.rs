use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt missing");

    println!("Sum: {}", process(&input));
}

#[derive(Debug)]
struct Subset {
    red: u8,
    blue: u8,
    green: u8,
}

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

fn parse_games(input: &str) -> Vec<Game> {
    let games_str: Vec<&str> = input.split("Game ").filter(|g| g != &""
    ).map(|w| w.trim()).collect();

    games_str
        .iter()
        .map(|game| {
            let mut subsets: Vec<Subset> = Vec::new();

            let mut it = game.split(":").into_iter();

            let id = it.next().unwrap().parse().expect("Expected a number for id.");

            let sets = it.next().unwrap();

            for set in sets.split(";") {
                let (mut r, mut g, mut b): (u8, u8, u8) = (0, 0, 0);
                let colors: Vec<&str> = set.split(",").map(|c| c.trim()).collect();
                for color in colors {
                    let v: Vec<&str> = color.split(" ").into_iter().collect();
                    let value: u8 = v[0].parse().ok().unwrap();
                    let color_name = v[1];

                    match color_name {
                        "red" => r = value,
                        "blue" => b = value,
                        "green" => g = value,
                        _ => panic!("Expected colors are red, green, blue")
                    }
                }

                subsets.push(Subset {
                    red: r,
                    green: g,
                    blue: b,
                })
            }

            Game {
                id,
                subsets,
            }
        })
        .collect()
}

fn find_minimums(games: Vec<Game>) -> Vec<Subset> {
    games
        .iter()
        .map(|game| {
            let (mut min_r, mut min_g, mut min_b): (u8, u8, u8) = (0, 0, 0);
            for set in game.subsets.iter() {
                if set.red > min_r {
                    min_r = set.red;
                }
                if set.green > min_g {
                    min_g = set.green;
                }
                if set.blue > min_b {
                    min_b = set.blue;
                }
            }
            Subset {
                red: min_r,
                green: min_g,
                blue: min_b,
            }
        })
        .collect()
}


fn process(input: &str) -> u32 {
    let all_games = parse_games(input);
    let minimums = find_minimums(all_games);

    let powers: Vec<u32> = minimums.into_iter().map(|set| {
        let power = u32::from(set.red) * u32::from(set.green) * u32::from(set.blue);
        return power;
    }).collect();

    powers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, process(test_input));
    }
}