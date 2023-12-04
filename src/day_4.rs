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
                    .filter(|s| s.len() > 0)
                    .map(|s| u32::from_str_radix(s, 10).unwrap())
                    .collect::<HashSet<u32>>(),
                card.trim()
                    .split(' ')
                    .filter(|s| s.len() > 0)
                    .map(|s| u32::from_str_radix(s, 10).unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .map(|(win_set, card)| card.iter().filter(|n| win_set.contains(n)).count())
        .filter(|&n| n > 0)
        .map(|n| 2u32.pow((n - 1) as u32))
        .sum::<u32>();
    println!("{ans}");
}
