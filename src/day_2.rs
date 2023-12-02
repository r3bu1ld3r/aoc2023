use std::cmp::max;

const LIMIT_RED: u8 = 12;
const LIMIT_GREEN: u8 = 13;
const LIMIT_BLUE: u8 = 14;

#[derive(Debug)]
enum Colors {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl From<&str> for Colors {
    fn from(value: &str) -> Self {
        let (amount, color) = value.trim().split_once(' ').unwrap();
        let amount: u8 = u8::from_str_radix(amount, 10).unwrap();
        match color {
            "red" => Self::Red(amount),
            "green" => Self::Green(amount),
            "blue" => Self::Blue(amount),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct GSet {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<&str> for GSet {
    fn from(value: &str) -> Self {
        let mut set = GSet::default();
        let _ = value
            .split(',')
            .map(Colors::from)
            .map(|c| match c {
                Colors::Red(amount) => set.red = amount,
                Colors::Green(amount) => set.green = amount,
                Colors::Blue(amount) => set.blue = amount,
            })
            .collect::<Vec<_>>();
        set
    }
}

static LIMITS: GSet = GSet {
    red: LIMIT_RED,
    green: LIMIT_GREEN,
    blue: LIMIT_BLUE,
};

#[derive(Debug)]
struct GameId(u16);

impl From<&str> for GameId {
    fn from(value: &str) -> Self {
        Self(value.split_once(' ').unwrap().1.parse().unwrap())
    }
}

#[derive(Debug)]
struct Game {
    pub id: GameId,
    pub sets: Vec<GSet>,
}

impl Game {
    pub fn is_valid(&self) -> bool {
        for set in self.sets.iter() {
            if set.red > LIMITS.red || set.green > LIMITS.green || set.blue > LIMITS.blue {
                return false;
            }
        }
        true
    }

    pub fn min_set_power(self) -> u32 {
        let mut set = GSet::default();
        self.sets.into_iter().for_each(|s| {
            set.red = max(s.red, set.red);
            set.green = max(s.green, set.green);
            set.blue = max(s.blue, set.blue);
        });
        set.red as u32 * set.green as u32 * set.blue as u32
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game_id, sets) = value.split_once(':').unwrap();
        Self {
            id: GameId::from(game_id),
            sets: sets.split(';').map(GSet::from).collect(),
        }
    }
}

fn main() {
    let input = include_str!("../input/2.txt");
    let answer = input
        .trim()
        .split('\n')
        .map(Game::from)
        .map(Game::min_set_power)
        .sum::<u32>();
    println!("{answer}");
}
