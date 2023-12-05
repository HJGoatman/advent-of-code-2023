use env_logger;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

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
            .map_while(|s| s.parse::<u32>().ok())
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

#[derive(Debug)]
struct AlmanacMap<SourceCategory, DestinationCategory> {
    items: Vec<AlmanacMapItem<SourceCategory, DestinationCategory>>,
}

impl<SourceCategory: Ord + AlmanacNumber, DestinationCategory: Ord + AlmanacNumber>
    AlmanacMap<SourceCategory, DestinationCategory>
{
    fn get(&self, source_id: &SourceCategory) -> DestinationCategory {
        let items = &self.items;

        for map_item in items {
            let source_number = source_id.get_value();
            let range_start = map_item.source_range_start.get_value();

            if source_number >= range_start {
                let distance_into_range = source_number - range_start;

                if distance_into_range < map_item.range_length {
                    let destination_number =
                        map_item.destination_range_start.get_value() + distance_into_range;

                    return DestinationCategory::new(destination_number);
                }
            }
        }

        DestinationCategory::new(source_id.get_value())
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

#[derive(Debug)]
struct AlmanacMapItem<SourceCategory, DestinationCategory> {
    destination_range_start: DestinationCategory,
    source_range_start: SourceCategory,
    range_length: u32,
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
    fn new(value: u32) -> Self;
    fn get_value(&self) -> u32;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SeedId {
    value: u32,
}

impl AlmanacNumber for SeedId {
    fn new(value: u32) -> SeedId {
        SeedId { value }
    }

    fn get_value(&self) -> u32 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SoilId {
    value: u32,
}

impl AlmanacNumber for SoilId {
    fn new(value: u32) -> SoilId {
        SoilId { value }
    }

    fn get_value(&self) -> u32 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FertiliserId {
    value: u32,
}

impl AlmanacNumber for FertiliserId {
    fn new(value: u32) -> FertiliserId {
        FertiliserId { value }
    }

    fn get_value(&self) -> u32 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct WaterId {
    value: u32,
}

impl AlmanacNumber for WaterId {
    fn new(value: u32) -> WaterId {
        WaterId { value }
    }

    fn get_value(&self) -> u32 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct LightId {
    value: u32,
}

impl AlmanacNumber for LightId {
    fn new(value: u32) -> LightId {
        LightId { value }
    }

    fn get_value(&self) -> u32 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TemperatureId {
    value: u32,
}

impl AlmanacNumber for TemperatureId {
    fn new(value: u32) -> TemperatureId {
        TemperatureId { value }
    }

    fn get_value(&self) -> u32 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HumidityId {
    value: u32,
}

impl AlmanacNumber for HumidityId {
    fn new(value: u32) -> HumidityId {
        HumidityId { value }
    }

    fn get_value(&self) -> u32 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct LocationId {
    value: u32,
}

impl AlmanacNumber for LocationId {
    fn new(value: u32) -> LocationId {
        LocationId { value }
    }

    fn get_value(&self) -> u32 {
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
