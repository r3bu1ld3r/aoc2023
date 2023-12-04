use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/4.txt");
    let ans = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| line.split_once('|').unwrap())
        .map(|(wins, card)| {
            (
                wins.trim()
                    .split(' ')
                    .filter_map(|s| u32::from_str_radix(s, 10).ok())
                    .collect::<HashSet<u32>>(),
                card.trim()
                    .split(' ')
                    .filter_map(|s| u32::from_str_radix(s, 10).ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .map(|(win_set, card)| card.iter().filter(|n| win_set.contains(n)).count())
        .filter(|&n| n > 0)
        .fold(0, |acc, n| acc + 2u32.pow((n - 1) as u32));
    println!("{ans}");
}
