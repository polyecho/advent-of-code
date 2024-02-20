/*
https://adventofcode.com/2023/day/2

You're launched high into the atmosphere! The apex of your trajectory just barely
reaches the surface of a large island floating in the sky. You gently land in a fluffy
pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow.
He'll be happy to explain the situation, but it's a bit of a walk, so you have some time.
They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red,
green, or blue. Each time you play this game, he will hide a secret number of cubes of
each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag,
grab a handful of random cubes, show them to you, and then put them back in the bag.
He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input).
Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a
semicolon-separated list of subsets of cubes that were revealed from
the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:
```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
````

In game 1, three sets of cubes are revealed from the bag (and then put back again).
The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes,
and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained
only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been
loaded with that configuration. However, game 3 would have been impossible because
at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also
have been impossible because the Elf showed you 15 blue cubes at once.
If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with
only 12 red cubes, 13 green cubes, and 14 blue cubes.
What is the sum of the IDs of those games?
*/

pub mod cube_conundrum {
    #[allow(dead_code)]
    pub fn get_game_id_sum(input: &str) -> usize {
        let lines: Vec<&str> = input.split('\n').collect();
        let games: Vec<Game> = lines_into_games(lines);

        let mut possible_game_ids: Vec<usize> = Vec::new();

        for game in games {
            let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);

            for round in game.rounds {
                if round.red > max_r {
                    max_r = round.red;
                }
                if round.green > max_g {
                    max_g = round.green;
                }
                if round.blue > max_b {
                    max_b = round.blue;
                }
            }

            if max_r <= 12 && max_g <= 13 && max_b <= 14 {
                possible_game_ids.push(game.id);
            }
        }

        let mut id_sum = 0;
        for id in possible_game_ids {
            id_sum += id;
        }

        id_sum
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

                let expected_answer: usize = 8;

                assert_eq!(get_game_id_sum(input), expected_answer);
            }
        }
    }
}
