use env_logger;
use std::collections::BTreeSet;
use std::env;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;
use std::u64;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coordinate {
    y: u64,
    x: u64,
}

#[derive(Debug)]
struct Image {
    pixels: BTreeSet<Coordinate>,
}

#[derive(Debug)]
struct ParseImageError;

impl FromStr for Image {
    type Err = ParseImageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pixels = BTreeSet::new();

        let mut x = 0;
        let mut y = 0;

        for c in s.chars() {
            if c == '#' {
                pixels.insert(Coordinate { x, y });
                x += 1;
            } else if c == '\n' {
                y += 1;
                x = 0;
            } else {
                x += 1;
            }
        }

        Ok(Image { pixels })
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;

        let max_x = self.pixels.iter().map(|coord| coord.x).max().unwrap() + 1;
        let max_y = self.pixels.iter().map(|coord| coord.y).max().unwrap() + 1;

        let mut galaxy_count = 1;
        for i in 0..(max_x * max_y) {
            let x = i % max_x;
            let y = i / max_x;

            if self.pixels.contains(&Coordinate { x, y }) {
                f.write_fmt(format_args!("{}", galaxy_count))?;
                galaxy_count += 1;
            } else {
                f.write_str(".")?;
            }

            if x == max_x - 1 {
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    log::debug!("\n{}", input);

    let image: Image = input.parse().unwrap();
    log::debug!("{}", image);

    let adjusted_image = account_for_gravitational_effects(&image, 2);
    log::debug!("{}", adjusted_image);

    let shortest_paths_between_galaxies = find_shortest_paths_between_galaxies(&adjusted_image);
    log::debug!("Shortest Paths: {:?}", shortest_paths_between_galaxies);
    let shortest_paths_between_galaxies_sum: u64 = shortest_paths_between_galaxies.iter().sum();
    println!("{}", shortest_paths_between_galaxies_sum);

    let adjusted_image = account_for_gravitational_effects(&image, 1000000);
    let shortest_paths_between_galaxies = find_shortest_paths_between_galaxies(&adjusted_image);
    let shortest_paths_between_galaxies_sum: u64 = shortest_paths_between_galaxies.iter().sum();
    println!("{}", shortest_paths_between_galaxies_sum);
}

fn find_shortest_paths_between_galaxies(adjusted_image: &Image) -> Vec<u64> {
    adjusted_image
        .pixels
        .iter()
        .enumerate()
        .flat_map(|(i, coordinate_1)| {
            adjusted_image
                .pixels
                .iter()
                .skip(i + 1)
                .map(|coordinate_2| {
                    (coordinate_2.y - coordinate_1.y)
                        + (coordinate_1.x.max(coordinate_2.x) - coordinate_1.x.min(coordinate_2.x))
                })
        })
        .collect()
}

fn account_for_gravitational_effects(image: &Image, gap_increase_factor: u64) -> Image {
    let xs: BTreeSet<u64> = image.pixels.iter().map(|coordinate| coordinate.x).collect();
    let ys: BTreeSet<u64> = image.pixels.iter().map(|coordinate| coordinate.y).collect();

    log::trace!("{:?}", xs);
    log::trace!("{:?}", ys);

    let x_gaps = find_gaps(&xs);
    let y_gaps = find_gaps(&ys);
    log::trace!("X gaps: {:?}", x_gaps);
    log::trace!("Y gaps: {:?}", y_gaps);

    let x_shifts = calculate_shifts(&x_gaps, gap_increase_factor);
    let y_shifts = calculate_shifts(&y_gaps, gap_increase_factor);

    log::trace!("X shifts: {:?}", x_shifts);
    log::trace!("Y shifts: {:?}", y_shifts);

    let mut shifted = BTreeSet::new();
    for coordinate in &image.pixels {
        let x = match x_shifts
            .iter()
            .rev()
            .find(|(shift_start, _)| coordinate.x >= *shift_start)
        {
            Some((_, shift_amount)) => coordinate.x + shift_amount,
            None => coordinate.x,
        };

        let y = match y_shifts
            .iter()
            .rev()
            .find(|(shift_start, _)| coordinate.y >= *shift_start)
        {
            Some((_, shift_amount)) => coordinate.y + shift_amount,
            None => coordinate.y,
        };

        let replacement = Coordinate { x, y };

        shifted.insert(replacement);
    }

    Image { pixels: shifted }
}

fn find_gaps(ordinates: &BTreeSet<u64>) -> Vec<(u64, u64)> {
    let mut gaps = Vec::new();
    let mut ordinate_iter = ordinates.iter();

    let mut previous = None;
    while let Some(current) = ordinate_iter.next() {
        if let Some(previous) = previous {
            let gap_start = previous + 1;
            let gap = current - gap_start;

            if gap > 0 {
                gaps.push((gap_start, gap));
            }
        }

        previous = Some(current)
    }

    gaps
}

fn calculate_shifts(gaps: &[(u64, u64)], gap_increase_factor: u64) -> Vec<(u64, u64)> {
    let mut cumulative_gap_size = 0;
    let mut shifts = Vec::new();

    for (gap_start, gap_lengh) in gaps {
        let gap_increase = gap_lengh * (gap_increase_factor - 1);

        let shift_start = *gap_start;
        let shift_amount = cumulative_gap_size + gap_increase;
        shifts.push((shift_start, shift_amount));
        cumulative_gap_size += gap_increase;
    }

    shifts
}
