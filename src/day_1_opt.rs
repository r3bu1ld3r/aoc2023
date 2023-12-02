#[derive(Debug, Eq, PartialEq)]
pub struct Calibration(u32);
use once_cell::sync::Lazy;
use std::str;

#[derive(Clone, Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    value: Option<u32>,
}

static TRIE: Lazy<Trie> = Lazy::new(|| {
    let mut trie = Trie::new();
    STR_DIGITS.iter().for_each(|(s, d)| trie.insert(s, *d));
    trie
});

static REV_TRIE: Lazy<Trie> = Lazy::new(|| {
    let mut rev_trie = Trie::new();
    STR_DIGITS
        .iter()
        .for_each(|(s, d)| rev_trie.insert(s.chars().rev().collect::<String>().as_str(), *d));
    rev_trie
});

impl Trie {
    fn new() -> Self {
        Trie {
            children: Default::default(),
            value: None,
        }
    }

    fn insert(&mut self, key: &str, value: u32) {
        let mut node = self;
        for c in key.chars() {
            let idx = c as usize - 'a' as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(Trie::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.value = Some(value);
    }

    fn exists(&self, key: &str) -> bool {
        let mut node = self;
        for c in key.chars() {
            let idx = c as usize - 'a' as usize;
            if node.children[idx].is_none() {
                return false;
            }
            node = node.children[idx].as_ref().unwrap();
        }
        true
    }

    fn find(&self, key: &str) -> Option<u32> {
        let mut node = self;
        for c in key.chars() {
            let idx = c as usize - 'a' as usize;
            if node.children[idx].is_none() {
                return None;
            }
            node = node.children[idx].as_ref().unwrap();
        }
        node.value
    }
}

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
        let s = value.as_bytes();
        let mut first = 0;
        let mut last = 0;

        let mut acc = [0u8; 5];
        let mut pos: usize = 0;
        let mut i = 0;

        while i < s.len() {
            let c = s[i] as char;
            if c.is_ascii_digit() {
                first = c.to_digit(10).unwrap();
                break;
            } else {
                acc[pos] = c as u8;
                let slice = str::from_utf8(&acc[..pos + 1]).unwrap();
                match TRIE.find(slice) {
                    Some(v) => {
                        first = v;
                        break;
                    }
                    None if TRIE.exists(slice) => {
                        pos += 1;
                    }
                    None if pos != 0 => {
                        i -= pos - 1;
                        pos = 1;
                        acc.fill(0);
                        acc[0] = s[i];
                    }
                    None => {}
                }
            }
            i += 1;
        }

        acc.fill(0);
        pos = 0;
        i = 0;
        let s = value.chars().rev().map(|c| c as u8).collect::<Vec<u8>>();
        while i < s.len() {
            let c = s[i] as char;
            if c.is_ascii_digit() {
                last = c.to_digit(10).unwrap();
                break;
            } else {
                acc[pos] = c as u8;
                let slice = str::from_utf8(&acc[..pos + 1]).unwrap();
                match REV_TRIE.find(slice) {
                    Some(v) => {
                        last = v;
                        break;
                    }
                    None if REV_TRIE.exists(slice) => pos += 1,
                    None if pos != 0 => {
                        i -= pos - 1;
                        pos = 1;
                        acc.fill(0);
                        acc[0] = s[i];
                    }
                    None => {}
                }
            }
            i += 1;
        }

        Calibration(first * 10 + last)
    }
}

fn main() {
    let input = include_str!("../input/1.txt");
    let t = std::time::Instant::now();
    let answer = input
        .trim()
        .split("\n")
        .map(|s| Calibration::from(s))
        .fold(0, |acc, cal| acc + cal.0);
    println!("elapsed: {}", t.elapsed().as_micros());
    println!("{}", answer);
}

#[test]
fn correctness() {
    let s = "zkjkctxvssix1dqb22five";
    let mut trie = Trie::new();
    let mut rev_trie = Trie::new();

    STR_DIGITS.iter().for_each(|(s, d)| trie.insert(s, *d));
    STR_DIGITS
        .iter()
        .for_each(|(s, d)| rev_trie.insert(s.chars().rev().collect::<String>().as_str(), *d));

    assert_eq!(Calibration::from(s).0, 65)
}

#[test]
fn correctness_v2() {
    let s = "vtnxseight1ndlxgleighttwosixthree";
    let mut trie = Trie::new();
    let mut rev_trie = Trie::new();

    STR_DIGITS.iter().for_each(|(s, d)| trie.insert(s, *d));
    STR_DIGITS
        .iter()
        .for_each(|(s, d)| rev_trie.insert(s.chars().rev().collect::<String>().as_str(), *d));

    assert_eq!(Calibration::from(s).0, 83)
}

#[test]
fn correctness_v3() {
    let s = "fivepqxlpninevh2xxsnsgg63pbvdnqptmg";
    let mut trie = Trie::new();
    let mut rev_trie = Trie::new();

    STR_DIGITS.iter().for_each(|(s, d)| trie.insert(s, *d));
    STR_DIGITS
        .iter()
        .for_each(|(s, d)| rev_trie.insert(s.chars().rev().collect::<String>().as_str(), *d));

    assert_eq!(Calibration::from(s).0, 53)
}
