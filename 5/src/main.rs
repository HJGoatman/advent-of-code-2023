use env_logger;
use std::env;
use std::fmt::Debug;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
struct Almanac {
    seeds_to_be_planted: Vec<SeedId>,

    seed_to_soil_map: AlmanacMap<SeedId, SoilId>,
    soil_to_fertiliser_map: AlmanacMap<SoilId, FertiliserId>,
    fertiliser_to_water_map: AlmanacMap<FertiliserId, WaterId>,
    water_to_light_map: AlmanacMap<WaterId, LightId>,
    light_to_temperature_map: AlmanacMap<LightId, TemperatureId>,
    temperature_to_humidity_map: AlmanacMap<TemperatureId, HumidityId>,
    humidity_to_location_map: AlmanacMap<HumidityId, LocationId>,
}

#[derive(Debug)]
enum ParseAlmanacError {
    AlmanacFormatError,
    AlmanacMapError(ParseAlmanacMapItemError),
}

impl From<ParseAlmanacMapItemError> for ParseAlmanacError {
    fn from(value: ParseAlmanacMapItemError) -> Self {
        ParseAlmanacError::AlmanacMapError(value)
    }
}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parse_iterator = s.split("\n\n");

        let seeds: &str = parse_iterator
            .next()
            .ok_or_else(|| ParseAlmanacError::AlmanacFormatError)?;

        let seeds_to_be_planted: Vec<SeedId> = seeds
            .trim()
            .split_whitespace()
            .skip(1)
            .map_while(|s| s.parse::<u64>().ok())
            .map(|value| SeedId { value })
            .collect();

        let seed_to_soil_map = parse_iterator.next().unwrap().parse()?;
        let soil_to_fertiliser_map = parse_iterator.next().unwrap().parse()?;
        let fertiliser_to_water_map = parse_iterator.next().unwrap().parse()?;
        let water_to_light_map = parse_iterator.next().unwrap().parse()?;
        let light_to_temperature_map = parse_iterator.next().unwrap().parse()?;
        let temperature_to_humidity_map = parse_iterator.next().unwrap().parse()?;
        let humidity_to_location_map = parse_iterator.next().unwrap().parse()?;

        Ok(Almanac {
            seeds_to_be_planted,
            seed_to_soil_map,
            soil_to_fertiliser_map,
            fertiliser_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        })
    }
}

#[derive(Debug, Clone)]
struct AlmanacMap<SourceCategory, DestinationCategory> {
    items: Vec<AlmanacMapItem<SourceCategory, DestinationCategory>>,
}

fn try_map_across_range(
    source_number: u64,
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
) -> Option<u64> {
    if source_number >= source_range_start {
        let distance_into_range = source_number - source_range_start;

        if distance_into_range < range_length {
            let destination_number = destination_range_start + distance_into_range;

            return Some(destination_number);
        }
    }
    None
}

impl<SourceCategory: Ord + AlmanacNumber, DestinationCategory: Ord + AlmanacNumber>
    AlmanacMap<SourceCategory, DestinationCategory>
{
    fn get(&self, source_id: &SourceCategory) -> DestinationCategory {
        let items = &self.items;

        for map_item in items {
            let source_number = source_id.get_value();
            let source_range_start = map_item.source_range_start.get_value();
            let destination_range_start = map_item.destination_range_start.get_value();

            let maybe_destination_number = try_map_across_range(
                source_number,
                source_range_start,
                destination_range_start,
                map_item.range_length,
            );

            if let Some(destination_number) = maybe_destination_number {
                return DestinationCategory::new(destination_number);
            }
        }

        DestinationCategory::new(source_id.get_value())
    }

    fn get_reversed(&self, destination_id: &DestinationCategory) -> SourceCategory {
        let items = &self.items;

        for map_item in items {
            let destination_number = destination_id.get_value();
            let source_range_start = map_item.source_range_start.get_value();
            let destination_range_start = map_item.destination_range_start.get_value();

            let maybe_source_number = try_map_across_range(
                destination_number,
                destination_range_start,
                source_range_start,
                map_item.range_length,
            );

            if let Some(source_number) = maybe_source_number {
                return SourceCategory::new(source_number);
            }
        }

        SourceCategory::new(destination_id.get_value())
    }
}

impl<SourceCategory: FromStr, DestinationCategory: FromStr> FromStr
    for AlmanacMap<SourceCategory, DestinationCategory>
{
    type Err = ParseAlmanacMapItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .split('\n')
            .filter(|s| s != &"")
            .skip(1)
            .map(|item_s| item_s.parse::<AlmanacMapItem<SourceCategory, DestinationCategory>>())
            .collect::<Result<
                Vec<AlmanacMapItem<SourceCategory, DestinationCategory>>,
                ParseAlmanacMapItemError,
            >>()?;

        Ok(AlmanacMap { items })
    }
}

#[derive(Debug, Clone)]
struct AlmanacMapItem<SourceCategory, DestinationCategory> {
    source_range_start: SourceCategory,
    destination_range_start: DestinationCategory,
    range_length: u64,
}

#[derive(Debug)]
struct ParseAlmanacMapItemError;

impl<SourceCategory: FromStr, DestinationCategory: FromStr> FromStr
    for AlmanacMapItem<SourceCategory, DestinationCategory>
{
    type Err = ParseAlmanacMapItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split_whitespace();

        let destination_range_start = split
            .next()
            .unwrap()
            .parse()
            .map_err(|_| ParseAlmanacMapItemError)?;
        let source_range_start = split
            .next()
            .unwrap()
            .parse()
            .map_err(|_| ParseAlmanacMapItemError)?;
        let range_length = split
            .next()
            .unwrap()
            .parse()
            .map_err(|_| ParseAlmanacMapItemError)?;

        Ok(AlmanacMapItem {
            destination_range_start,
            source_range_start,
            range_length,
        })
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let almanac: Almanac = input.parse().unwrap();
    log::debug!("{:#?}", almanac);

    let location_numbers: Vec<LocationId> = almanac
        .seeds_to_be_planted
        .iter()
        .map(|seed_id| get_location_id(seed_id, &almanac))
        .collect();

    log::debug!("{:?}", location_numbers);
    let lowest_location_number = location_numbers.iter().min().unwrap();

    println!("{}", lowest_location_number.get_value());

    let temperature_to_location = reduce_maps(
        almanac.temperature_to_humidity_map,
        almanac.humidity_to_location_map,
    );
    let light_to_location = reduce_maps(almanac.light_to_temperature_map, temperature_to_location);

    log::debug!("{:#?}", light_to_location);
    let water_to_location = reduce_maps(almanac.water_to_light_map, light_to_location);
    let fertiliser_to_location = reduce_maps(almanac.fertiliser_to_water_map, water_to_location);
    let soil_to_location = reduce_maps(almanac.soil_to_fertiliser_map, fertiliser_to_location);
    let seed_to_location = reduce_maps(almanac.seed_to_soil_map, soil_to_location);

    let seed_ranges_items: Vec<AlmanacMapItem<SeedId, SeedId>> = almanac
        .seeds_to_be_planted
        .chunks(2)
        .map(|seed_ids| (seed_ids[0], seed_ids[1].get_value()))
        .map(|(start, range)| AlmanacMapItem {
            source_range_start: start,
            destination_range_start: start,
            range_length: range,
        })
        .collect();

    let seed_ranges = AlmanacMap {
        items: seed_ranges_items,
    };

    let mut range_splits = reduce_maps(seed_ranges.clone(), seed_to_location);
    range_splits.items = range_splits
        .items
        .into_iter()
        .filter(|item| {
            seed_ranges.items.iter().any(|original_item| {
                (item.source_range_start >= original_item.source_range_start)
                    && (item.source_range_start.get_value() + item.range_length
                        <= original_item.source_range_start.get_value()
                            + original_item.range_length)
            })
        })
        .collect();

    log::debug!("{:#?}", range_splits);

    let lowest_location_number = range_splits
        .items
        .iter()
        .map(|item| item.destination_range_start)
        .min()
        .unwrap();

    println!("{}", lowest_location_number.get_value());
}

fn try_get_overlapping_ranges(
    start_1: u64,
    end_1: u64,
    start_2: u64,
    end_2: u64,
) -> Option<Vec<(u64, u64)>> {
    let ranges_overlap = (start_1 < end_2) && (end_1 > start_2);
    if ranges_overlap {
        log::trace!(
            "Overlap Found: {}..{} and {}..{}",
            start_1,
            end_1,
            start_2,
            end_2
        );

        let mut range_points = vec![start_1, end_1, start_2, end_2];
        range_points = range_points.into_iter().unique().collect();
        range_points.sort();

        let new_ranges: Vec<(u64, u64)> = (0..(range_points.len() - 1))
            .map(|i| (range_points[i], range_points[i + 1]))
            .collect();

        log::trace!(
            "\tNew Ranges: [{}]",
            new_ranges
                .iter()
                .map(|(start, end)| format!("{start}..{end}"))
                .join(", ")
        );

        return Some(new_ranges);
    } else {
        log::trace!(
            "No Overlap Found: {}..{} and {}..{}",
            start_1,
            end_1,
            start_2,
            end_2
        );

        return None;
    }
}

fn get_start_and_end<A: AlmanacNumber>(range_start: A, range_len: u64) -> (u64, u64) {
    let start_value = range_start.get_value();
    let end_value = start_value + range_len;

    (start_value, end_value)
}

fn reduce_maps<S1, D1, D2>(
    map_1: AlmanacMap<S1, D1>,
    map_2: AlmanacMap<D1, D2>,
) -> AlmanacMap<S1, D2>
where
    S1: Debug + AlmanacNumber + Copy + Ord,
    D1: Debug + AlmanacNumber + Copy + Ord,
    D2: Debug + AlmanacNumber + Copy + Ord,
{
    log::debug!("{:#?}", &map_1);
    log::debug!("{:#?}", &map_2);

    let mut ranges_1: Vec<(u64, u64)> = map_1
        .items
        .iter()
        .map(|item| get_start_and_end(item.destination_range_start, item.range_length))
        .collect();
    let mut ranges_2: Vec<(u64, u64)> = map_2
        .items
        .iter()
        .map(|item| get_start_and_end(item.source_range_start, item.range_length))
        .collect();

    // Let's put ranges 1 into ranges 2.
    log::trace!("Ranges:\n\t{:?}\n\t{:?}", ranges_1, ranges_2);

    while let Some((start_1, end_1)) = ranges_1.pop() {
        let mut new_ranges: Vec<(u64, u64)> = Vec::new();
        let mut i = 0;

        for (start_2, end_2) in ranges_2.iter() {
            if let Some(ranges) = try_get_overlapping_ranges(start_1, end_1, *start_2, *end_2) {
                new_ranges = ranges;
                break;
            }

            i += 1;
        }

        if new_ranges.is_empty() {
            // They all don't overlap
            ranges_2.push((start_1, end_1));
        } else {
            // Remove old one and the splits.
            ranges_2.remove(i);
            new_ranges
                .into_iter()
                .for_each(|range| ranges_1.push(range));
        }
    }

    log::debug!("{:?}", ranges_2);

    let map_1_source_range_starts = ranges_2
        .iter()
        .map(|(start, _)| map_1.get_reversed(&D1::new(*start)));
    let map_2_destination_range_starts = ranges_2
        .iter()
        .map(|(start, _)| map_2.get(&D1::new(*start)));
    let range_lengths = ranges_2.iter().map(|(start, end)| end - start);

    let items: Vec<AlmanacMapItem<S1, D2>> = map_1_source_range_starts
        .zip(map_2_destination_range_starts)
        .zip(range_lengths)
        .map(
            |((source_range_start, destination_range_start), range_length)| AlmanacMapItem {
                destination_range_start,
                source_range_start,
                range_length,
            },
        )
        .collect();

    AlmanacMap { items }
}

fn get_location_id(seed_id: &SeedId, almanac: &Almanac) -> LocationId {
    let location_id = Some(seed_id)
        .map(|seed_id| almanac.seed_to_soil_map.get(seed_id))
        .map(|soil_id| almanac.soil_to_fertiliser_map.get(&soil_id))
        .map(|fertiliser_id| almanac.fertiliser_to_water_map.get(&fertiliser_id))
        .map(|water_id| almanac.water_to_light_map.get(&water_id))
        .map(|light_id| almanac.light_to_temperature_map.get(&light_id))
        .map(|temperature_id| almanac.temperature_to_humidity_map.get(&temperature_id))
        .map(|humidity_id| almanac.humidity_to_location_map.get(&humidity_id))
        .unwrap();

    location_id
}

trait AlmanacNumber {
    fn new(value: u64) -> Self;
    fn get_value(&self) -> u64;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct SeedId {
    value: u64,
}

impl AlmanacNumber for SeedId {
    fn new(value: u64) -> SeedId {
        SeedId { value }
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

impl FromStr for SeedId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;

        Ok(SeedId { value })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct SoilId {
    value: u64,
}

impl AlmanacNumber for SoilId {
    fn new(value: u64) -> SoilId {
        SoilId { value }
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

impl FromStr for SoilId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;

        Ok(SoilId { value })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct FertiliserId {
    value: u64,
}

impl AlmanacNumber for FertiliserId {
    fn new(value: u64) -> FertiliserId {
        FertiliserId { value }
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

impl FromStr for FertiliserId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;

        Ok(FertiliserId { value })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct WaterId {
    value: u64,
}

impl AlmanacNumber for WaterId {
    fn new(value: u64) -> WaterId {
        WaterId { value }
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

impl FromStr for WaterId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;

        Ok(WaterId { value })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct LightId {
    value: u64,
}

impl AlmanacNumber for LightId {
    fn new(value: u64) -> LightId {
        LightId { value }
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

impl FromStr for LightId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;

        Ok(LightId { value })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct TemperatureId {
    value: u64,
}

impl AlmanacNumber for TemperatureId {
    fn new(value: u64) -> TemperatureId {
        TemperatureId { value }
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

impl FromStr for TemperatureId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;

        Ok(TemperatureId { value })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct HumidityId {
    value: u64,
}

impl AlmanacNumber for HumidityId {
    fn new(value: u64) -> HumidityId {
        HumidityId { value }
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

impl FromStr for HumidityId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;

        Ok(HumidityId { value })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct LocationId {
    value: u64,
}

impl AlmanacNumber for LocationId {
    fn new(value: u64) -> LocationId {
        LocationId { value }
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

impl FromStr for LocationId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;

        Ok(LocationId { value })
    }
}
