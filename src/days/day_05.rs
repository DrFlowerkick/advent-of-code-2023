//!day_05.rs

use anyhow::{anyhow, Result};

#[derive(Default)]
struct TransferMap {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

impl TransferMap {
    fn from_str(input: &str) -> Result<Self> {
        let mut result = Self::default();
        let mut input_iter = input.split_ascii_whitespace();
        match input_iter.next() {
            Some(ds) => result.destination_start = ds.parse::<u64>()?,
            None => return Err(anyhow!("bad input")),
        }
        match input_iter.next() {
            Some(ss) => result.source_start = ss.parse::<u64>()?,
            None => return Err(anyhow!("bad input")),
        }
        match input_iter.next() {
            Some(r) => result.range = r.parse::<u64>()?,
            None => return Err(anyhow!("bad input")),
        }
        match input_iter.next() {
            Some(_) => return Err(anyhow!("bad input")),
            None => Ok(result),
        }
    }
    fn get_destination(&self, source: u64) -> Option<u64> {
        if self.source_start <= source && source < self.source_start + self.range {
            Some(self.destination_start + source - self.source_start)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
enum TransferMapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl Default for TransferMapType {
    fn default() -> Self {
        TransferMapType::SeedToSoil
    }
}

impl TransferMapType {
    fn next(&self) -> Option<Self> {
        let result = match self {
            TransferMapType::SeedToSoil => TransferMapType::SoilToFertilizer,
            TransferMapType::SoilToFertilizer => TransferMapType::FertilizerToWater,
            TransferMapType::FertilizerToWater => TransferMapType::WaterToLight,
            TransferMapType::WaterToLight => TransferMapType::LightToTemperature,
            TransferMapType::LightToTemperature => TransferMapType::TemperatureToHumidity,
            TransferMapType::TemperatureToHumidity => TransferMapType::HumidityToLocation,
            TransferMapType::HumidityToLocation => return None,
        };
        Some(result)
    }
}

#[derive(Default)]
struct TransferMapSet {
    seed_to_soil: Vec<TransferMap>,
    soil_to_fertilizer: Vec<TransferMap>,
    fertilizer_to_water: Vec<TransferMap>,
    water_to_light: Vec<TransferMap>,
    light_to_temperature: Vec<TransferMap>,
    temperature_to_humidity: Vec<TransferMap>,
    humidity_to_location: Vec<TransferMap>,
}

impl TransferMapSet {
    fn get_trans_map(&self, map_type: TransferMapType) -> &Vec<TransferMap> {
        match map_type {
            TransferMapType::SeedToSoil => &self.seed_to_soil,
            TransferMapType::SoilToFertilizer => &self.soil_to_fertilizer,
            TransferMapType::FertilizerToWater => &self.fertilizer_to_water,
            TransferMapType::WaterToLight => &self.water_to_light,
            TransferMapType::LightToTemperature => &self.light_to_temperature,
            TransferMapType::TemperatureToHumidity => &self.temperature_to_humidity,
            TransferMapType::HumidityToLocation => &self.humidity_to_location,
        }
    }
    fn get_trans_map_mut(&mut self, map_type: TransferMapType) -> &mut Vec<TransferMap> {
        match map_type {
            TransferMapType::SeedToSoil => &mut self.seed_to_soil,
            TransferMapType::SoilToFertilizer => &mut self.soil_to_fertilizer,
            TransferMapType::FertilizerToWater => &mut self.fertilizer_to_water,
            TransferMapType::WaterToLight => &mut self.water_to_light,
            TransferMapType::LightToTemperature => &mut self.light_to_temperature,
            TransferMapType::TemperatureToHumidity => &mut self.temperature_to_humidity,
            TransferMapType::HumidityToLocation => &mut self.humidity_to_location,
        }
    }
    fn add_transfer_map(&mut self, map: TransferMap, map_type: TransferMapType) {
        self.get_trans_map_mut(map_type).push(map);
    }
    fn get_destination(&self, source: u64, map_type: TransferMapType) -> u64 {
        let transfer_map = self.get_trans_map(map_type);
        for map in transfer_map.iter() {
            match map.get_destination(source) {
                Some(dest) => return dest,
                None => (),
            }
        }
        source
    }
    fn get_location(&self, seed: u64) -> u64 {
        let mut map_type = Some(TransferMapType::default());
        let mut location = seed;
        while let Some(tmt) = map_type {
            location = self.get_destination(location, tmt);
            map_type = tmt.next();
        }
        location
    }
}

pub fn day_05() -> Result<()> {
    let input = include_str!("../../assets/day_05.txt");
    let mut transfer_maps = TransferMapSet::default();
    let mut seeds: Vec<u64> = Vec::new();
    let mut transfer_map_type: Option<TransferMapType> = None;
    for line in input.lines().filter(|l| !l.is_empty()) {
        if seeds.len() == 0 {
            seeds = line
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split_ascii_whitespace()
                .map(|u| u.parse::<u64>().expect("bad input"))
                .collect();
        } else if line.contains("map") {
            transfer_map_type = match transfer_map_type {
                Some(tmt) => tmt.next(),
                None => Some(TransferMapType::default()),
            };
        } else if let Some(tmt) = transfer_map_type {
            transfer_maps.add_transfer_map(TransferMap::from_str(line)?, tmt);
        }
    }

    let mut lowest_location = u64::MAX;
    for seed in seeds.iter() {
        let location = transfer_maps.get_location(*seed);
        lowest_location = lowest_location.min(location);
    }
    println!("result day 05 part 1: {}", lowest_location);

    // Part 2
    
    #[cfg(feature = "long-run-time")]
    {
        let mut seed_iter = seeds.iter();
        let mut lowest_location = u64::MAX;
        while let Some(&start_seed) = seed_iter.next() {
            let &seed_range = seed_iter.next().expect("bad input");
            for seed in start_seed..start_seed+seed_range {
                let location = transfer_maps.get_location(seed);
                lowest_location = lowest_location.min(location);
            }
        }
        println!("result day 05 part 2: {}", lowest_location);
    }

    if !cfg!(feature = "long-run-time") {
        println!("result day 05 part 2: skipped because of long run time.");
    }

    Ok(())
}
