use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/4.txt");
    let cards = input
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
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut copies: Vec<u128> = vec![1; cards.len()];
    while i < cards.len() {
        if cards[i] == 0 {
            i += 1;
            continue;
        };
        let left_bound = i + 1;
        let right_bound = if i + 1 + cards[i] < cards.len() {
            i + 1 + cards[i]
        } else {
            cards.len()
        };
        
        for j in left_bound..right_bound {
            copies[j] += copies[i]
        }
        i += 1;
    }

    let ans = copies.iter().sum::<u128>();
    println!("{ans}");
}
