use crate::PartRatingValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub min: PartRatingValue,
    pub max: PartRatingValue,
}

impl Range {
    fn has_overlap(&self, other: Range) -> bool {
        (self.min < other.max) && (self.max > other.min)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set(pub Vec<Range>);

impl Set {
    pub fn intersection(&mut self, other: &mut Set) {
        let mut new_ranges = Vec::new();

        while let Some(Range {
            min: other_min,
            max: other_max,
        }) = other.0.pop()
        {
            for Range {
                min: self_min,
                max: self_max,
            } in &self.0
            {
                let min = other_min.max(*self_min);
                let max = other_max.min(*self_max);

                if min < max {
                    new_ranges.push(Range { min, max });
                }
            }
        }

        self.0 = new_ranges;

        self.0.sort();
        self.join_continuous_ranges();
    }

    pub fn union(&mut self, other: &mut Set) {
        while let Some(other_range) = other.0.pop() {
            if let Some(overlapping_index) = self
                .0
                .iter()
                .position(|self_range| self_range.has_overlap(other_range))
            {
                let self_overlapped_range = self.0.remove(overlapping_index);

                find_distict_ranges(self_overlapped_range, other_range)
                    .into_iter()
                    .for_each(|range| other.0.push(range));
            } else {
                self.0.push(other_range);
            }
        }

        self.0.sort();
        self.join_continuous_ranges();
    }

    pub fn difference(&mut self, other: &mut Set) {
        while let Some(other_range) = other.0.pop() {
            if let Some(overlapping_index) = self
                .0
                .iter()
                .position(|self_range| self_range.has_overlap(other_range))
            {
                let self_overlapped_range = self.0.remove(overlapping_index);

                let distict_ranges = find_distict_ranges(self_overlapped_range, other_range);

                for range in distict_ranges {
                    if range.has_overlap(other_range) {
                        other.0.push(range);
                    } else {
                        self.0.push(range);
                    }
                }
            }
        }

        self.0.sort();
        self.join_continuous_ranges();
    }

    pub(crate) fn cardinality(&self) -> PartRatingValue {
        self.0.iter().map(|range| range.max - range.min).sum()
    }

    fn join_continuous_ranges(&mut self) {
        if self.0.is_empty() {
            return;
        }

        let mut i = 0;
        let mut list_len = self.0.len() - 1;
        while i < list_len {
            let range_1 = self.0[i];
            let range_2 = self.0[i + 1];

            if range_1.max == range_2.min {
                let new_range = Range {
                    min: range_1.min,
                    max: range_2.max,
                };

                self.0.remove(i + 1);
                self.0[i] = new_range;
                list_len = self.0.len() - 1;
            }

            i += 1;
        }
    }
}

fn find_distict_ranges(range_1: Range, range_2: Range) -> Vec<Range> {
    let mut ranges = Vec::new();
    let mut boundary_points = [range_1.min, range_1.max, range_2.min, range_2.max];

    boundary_points.sort();

    for i in 0..boundary_points.len() - 1 {
        let min = boundary_points[i];
        let max = boundary_points[i + 1];

        let range = Range { min, max };

        if min < max {
            ranges.push(range);
        }
    }

    ranges
}
