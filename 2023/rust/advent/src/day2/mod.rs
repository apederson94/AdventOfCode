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
                    let valid_red = h.r <= 12;
                    let valid_green = h.g <= 13;
                    let valid_blue = h.b <= 14;

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
                let max_red = max(h.r, oh.r);
                let max_green = max(h.g, oh.g);
                let max_blue = max(h.b, oh.b);

                Hand {
                    r: max_red,
                    g: max_green,
                    b: max_blue
                }
            }).unwrap()
    }).map(|h| {
        let r = u32::from(h.r);
        let g = u32::from(h.g);
        let b = u32::from(h.b);
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
        r: 0,
        g: 0,
        b: 0,
    };

    re.find_iter(value)
        .fold(initial_hand, |mut h, m| {
            let num_and_color: Vec<_> = m.as_str().split(" ").collect();
            let num = num_and_color[0].parse::<u8>().unwrap();
            let color = num_and_color[1];

            match color {
                "red" => h.r = num,
                "green" => h.g = num,
                "blue" => h.b = num,
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
    r: u8,
    g: u8,
    b: u8
}