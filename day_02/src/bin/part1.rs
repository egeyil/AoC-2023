use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt missing");

    let possible_games = calc_possible_games(&input, Subset {
        red: 12,
        green: 13,
        blue: 14,
    });

    println!("Possible Games: {:?}", possible_games);

    let sum = possible_games.iter().sum::<u32>();

    println!("Sum: {sum}");
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


fn calc_possible_games(input: &str, rgb: Subset) -> Vec<u32> {
    let games_str: Vec<&str> = input.split("Game ").filter(|g| g != &""
    ).map(|w| w.trim()).collect();

    let all_games: Vec<Game> =
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
            .collect();

    all_games
        .iter()
        .filter(|game| {
            let impossible_sets: Vec<&Subset> = game.subsets
                .iter()
                .filter(|subset| {
                    subset.red >= rgb.red ||
                        subset.blue >= rgb.blue ||
                        subset.green >= rgb.green
                })
                .collect();

            impossible_sets.len() == 0
        })
        .map(|game: &Game| game.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_match() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(vec![1, 2, 5], calc_possible_games(test_input, Subset {
            red: 12,
            green: 13,
            blue: 14,
        }));
    }

    #[test]
    fn sums_correct() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let sum: u32 = 8;

        assert_eq!(sum, calc_possible_games(test_input, Subset {
            red: 12,
            green: 13,
            blue: 14,
        }).iter().sum());
    }
}