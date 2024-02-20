/*
https://adventofcode.com/2023/day/2

The Elf says they've stopped producing snow because they aren't getting any water!
He isn't sure why the water stopped; however, he can show you how to get to the water source
to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question:
in each game you played, what is the fewest number of cubes of each color that could have been
in the bag to make the game possible?

Again consider the example games from earlier:
```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes.
If any color had even one fewer cube, the game would have been impossible.
- Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
- Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
- Game 4 required at least 14 red, 3 green, and 15 blue cubes.
- Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.

The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630,
and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present.
What is the sum of the power of these sets?
*/

pub mod cube_conundrum {
    #[allow(dead_code)]
    pub fn get_game_id_sum(input: &str) -> usize {
        let lines: Vec<&str> = input.split('\n').collect();
        let games: Vec<Game> = lines_into_games(lines);

        let mut powered_numbers: Vec<usize> = Vec::new();

        for game in games {
            let (mut fewest_r, mut fewest_g, mut fewest_b) = (0, 0, 0);

            for round in game.rounds {
                if round.red != 0 && round.red > fewest_r {
                    fewest_r = round.red;
                }
                if round.green != 0 && round.green > fewest_g {
                    fewest_g = round.green;
                }
                if round.blue != 0 && round.blue > fewest_b {
                    fewest_b = round.blue;
                }
            }

            powered_numbers.push(fewest_r * fewest_g * fewest_b);
        }

        let mut power = 0;
        for sum in powered_numbers {
            if sum < 1 {
                continue;
            }
            power += sum;
        }

        power
    }

    #[derive(Debug, PartialEq)]
    pub struct Round {
        red: usize,
        green: usize,
        blue: usize,
    }
    #[derive(Debug, PartialEq)]
    pub struct Game {
        id: usize,
        rounds: Vec<Round>,
    }

    fn lines_into_games(lines: Vec<&str>) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();

        for line in lines {
            let mut temp_split = line.split(':');
            let game_id = temp_split
                .next()
                .unwrap()
                .replace("Game ", "")
                .parse::<usize>()
                .unwrap();
            let round_slices: Vec<&str> = temp_split
                .next()
                .unwrap()
                .split(';')
                .map(|item| item.trim())
                .collect();

            let mut rounds: Vec<Round> = Vec::new();
            for round_slice in round_slices {
                let round_details: Vec<&str> = round_slice
                    .trim()
                    .split(',')
                    .map(|item| item.trim())
                    .collect();

                let mut round_info = Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                for round_detail in round_details {
                    let mut temp_split = round_detail.split(' ');

                    let round_number = temp_split.next().unwrap().parse::<usize>().unwrap();
                    let round_color = temp_split.next().unwrap();

                    match round_color {
                        "red" => round_info.red = round_number,
                        "green" => round_info.green = round_number,
                        "blue" => round_info.blue = round_number,
                        _ => unreachable!(),
                    }
                }

                rounds.push(round_info);
            }

            games.push(Game {
                id: game_id,
                rounds,
            })
        }

        games
    }

    mod test {
        #[cfg(test)]
        mod get_game_id_sum {
            use super::super::get_game_id_sum;

            #[test]
            fn test_example() {
                let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

                let expected_answer: usize = 2286;

                assert_eq!(get_game_id_sum(input), expected_answer);
            }
        }
    }
}
