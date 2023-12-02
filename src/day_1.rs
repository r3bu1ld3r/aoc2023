pub struct Calibration(u32);

const STR_DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

impl From<&str> for Calibration {
    fn from(value: &str) -> Self {
        let first;
        let earliest = value
            .find(|c: char| c.is_ascii_digit())
            .unwrap_or(value.len());
        match STR_DIGITS
            .iter()
            .map(|(s, d)| (s, d, value.find(s)))
            .filter(|(_, _, idx)| idx.is_some())
            .min_by_key(|&(_, _, idx)| idx)
        {
            Some((_, &d, Some(idx))) if idx < earliest => first = d,
            Some((_, _, _)) | None => {
                first = value
                    .chars()
                    .nth(earliest)
                    .unwrap_or('0')
                    .to_digit(10)
                    .unwrap()
            }
        };

        let last;
        let latest = value.rfind(|c: char| c.is_ascii_digit()).unwrap_or(0);
        match STR_DIGITS 
            .iter()
            .map(|(s, d)| (s, d, value.rfind(s)))
            .filter(|(_, _, idx)| idx.is_some())
            .max_by_key(|&(_, _, idx)| idx)
        {
            Some((_, &d, Some(idx))) if idx > latest => last = d,
            Some((_, _, _)) | None => {
                last = value
                    .chars()
                    .nth(latest)
                    .unwrap_or('0')
                    .to_digit(10)
                    .unwrap()
            }
        };
        Calibration(first * 10 + last)
    }
}

fn main() {
    let input = include_str!("../input/1.txt");
    let answer = input
        .split("\n")
        .map(|s| Calibration::from(s))
        .fold(0, |acc, cal| acc + cal.0);
    println!("{}", answer);
}
