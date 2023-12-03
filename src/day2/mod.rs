use std::cmp::max;
use std::fs::read_to_string;
use regex::Regex;

pub(crate) fn part1(filename: &str) -> u32 {
    find_12_13_14_games(filename)
}

pub(crate) fn part2(filename: &str) -> u32 {
    find_power_of_games(filename)
}

fn find_12_13_14_games(filename: &str) -> u32 {
    let games = get_games_from_file(filename);

    let games_left: Vec<_> = games.iter()
        .filter(|g| {
            let hands = g.hands.iter()
                .filter(|h| {
                    let valid_red = h.red <= 12;
                    let valid_green = h.green <= 13;
                    let valid_blue = h.blue <= 14;

                    valid_red && valid_green && valid_blue
                }).collect::<Vec<_>>();

            hands.iter().count() == g.hands.iter().count()
        }).collect();

    games_left
        .into_iter()
        .map(|g| g.id)
        .sum()
}

fn find_power_of_games(filename: &str) -> u32 {
    let games = get_games_from_file(filename);

    games.iter().map(|g| {
        g.hands.iter()
            .map(|g| g.to_owned())
            .reduce(|h, oh| {
                let max_red = max(h.red, oh.red);
                let max_green = max(h.green, oh.green);
                let max_blue = max(h.blue, oh.blue);

                Hand {
                    red: max_red,
                    green: max_green,
                    blue: max_blue
                }
            }).unwrap()
    }).map(|h| {
        let r = u32::from(h.red);
        let g = u32::from(h.green);
        let b = u32::from(h.blue);
        r * g * b
    })
        .sum()
}

fn get_games_from_file(filename: &str) -> Vec<Game> {
    let input = read_to_string(filename)
        .unwrap();
    let mut game_id = 1;

    input.lines()
        .map(|s| {
            let hands = s.split(";")
                .map(|v| {
                    get_hand_from_str(v)
                }).collect::<Vec<_>>();

            let game = Game {
                id: game_id,
                hands
            };
            game_id += 1;

            game
        }).collect()
}

fn get_hand_from_str(value: &str) -> Hand {
    let pattern = "(\\d+ red)|(\\d+ green)|(\\d+ blue)";
    let re = Regex::new(pattern).unwrap();

    let initial_hand = Hand {
        red: 0,
        green: 0,
        blue: 0,
    };

    re.find_iter(value)
        .fold(initial_hand, |mut h, m| {
            let num_and_color: Vec<_> = m.as_str().split(" ").collect();
            let num = num_and_color[0].parse::<u8>().unwrap();
            let color = num_and_color[1];

            match color {
                "red" => h.red = num,
                "green" => h.green = num,
                "blue" => h.blue = num,
                _ => {}
            }

            h
        })
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>
}

#[derive(Debug, Copy, Clone)]
struct Hand {
    red: u8,
    green: u8,
    blue: u8
}

// impl fmt::Debug for Hand {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "red: {}, green: {}, blue: {}", self.red, self.green, self.blue)
//     }
// }