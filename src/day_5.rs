use std::{collections::HashMap, ops::Range, thread};

const MAP_NAMES: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

#[derive(Clone, Debug)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl From<&str> for MapType {
    fn from(value: &str) -> Self {
        match value {
            "seed-to-soil" => MapType::SeedToSoil,
            "soil-to-fertilizer" => MapType::SoilToFertilizer,
            "fertilizer-to-water" => MapType::FertilizerToWater,
            "water-to-light" => MapType::WaterToLight,
            "light-to-temperature" => MapType::LightToTemperature,
            "temperature-to-humidity" => MapType::TemperatureToHumidity,
            "humidity-to-location" => MapType::HumidityToLocation,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    id: MapType,
    data: HashMap<Range<u64>, u64>,
}

impl Map {
    pub fn translate(&self, input: u64) -> u64 {
        for (source, dst) in &self.data {
            if source.contains(&input) {
                return dst + (input - source.start);
            }
        }
        return input;
    }
}

// destination range start | source range start | range length

fn main() {
    let input = include_str!("../input/5.txt");
    let mut iter = input.trim().lines();
    let seeds: Vec<&str> = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .collect::<Vec<&str>>();

    let seeds: Vec<Range<u64>> = seeds
        .iter()
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect::<Vec<u64>>()
        .as_slice()
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect::<Vec<Range<u64>>>();

    let mut maps = vec![];
    for name in MAP_NAMES.iter() {
        'outer: while let Some(line) = iter.next() {
            if line.contains(*name) {
                let mut map = Map {
                    id: MapType::from(*name),
                    data: HashMap::new(),
                };
                while let Some(map_line) = iter.next() {
                    if map_line.is_empty() {
                        break;
                    } else {
                        //fill map
                        let map_values = map_line
                            .trim()
                            .split(' ')
                            .map(|s| u64::from_str_radix(s, 10).unwrap())
                            .collect::<Vec<u64>>();
                        map.data
                            .insert(map_values[1]..map_values[1] + map_values[2], map_values[0]);
                    }
                }
                maps.push(map);
                break 'outer;
            }
        }
    }
    assert_eq!(maps.len(), 7);

    let (tx, rx) = std::sync::mpsc::channel();

    for seed in seeds {
        let local_maps = maps.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let mut local_min = u64::MAX;
            for i in seed {
                let mut current = i;
                for map in local_maps.iter() {
                    current = map.translate(current);
                }
                if current < local_min {
                    local_min = current;
                    tx.send(current).unwrap();
                }
            }
        });
    }

    drop(tx);

    println!("day 5 part 2: {}", rx.iter().min().unwrap());
}
