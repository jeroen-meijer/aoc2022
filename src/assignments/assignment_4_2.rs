use std::ops::RangeInclusive;

use super::Assignment;

pub fn get_assignment() -> Assignment {
    return Assignment::new(4, 2, "Camp Cleanup".to_string(), Some(928), _run);
}

#[derive(PartialEq, Debug)]
struct Section {
    start: u32,
    end: u32,
}

trait ToRangeInclusive {
    type T;

    fn to_range(&self) -> Result<RangeInclusive<Self::T>, ()>;
}

impl ToRangeInclusive for &str {
    type T = u32;

    fn to_range(&self) -> Result<RangeInclusive<Self::T>, ()> {
        let split = self.split('-').collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(());
        }

        let start = split[0].parse::<u32>();
        if let Err(_) = start {
            return Err(());
        }

        let end = split[1].parse::<u32>();
        if let Err(_) = end {
            return Err(());
        }

        Ok(start.unwrap()..=end.unwrap())
    }
}

trait Overlap {
    fn fully_contains(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl Overlap for RangeInclusive<u32> {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start() <= other.start() && other.end() <= self.end()
    }

    fn overlaps(&self, other: &Self) -> bool {
        // self:  12345....
        // other: 12345....
        (self == other) ||
        // self:  12345....
        // other: .234.....
        self.fully_contains(other) ||
        // self:  .234.....
        // other: 12345....
        other.fully_contains(self) ||
        // self:  12345....
        // other: ..34567..
        (self.start() <= other.start() && other.start() <= self.end() && self.end() <= other.end()) ||
        // self:  ..34567..
        // other: 12345....
        (other.start() <= self.start() && self.start() <= other.end() && other.end() <= self.end())
    }
}

fn _run(data: Vec<String>) -> Option<i32> {
    let section_pairs = data.iter().map(|line| {
        let mut pair_strings = line.split(',');
        let a = pair_strings.next().unwrap().to_range().unwrap();
        let b = pair_strings.next().unwrap().to_range().unwrap();

        (a, b)
    });

    Some(section_pairs.filter(|(a, b)| a.overlaps(&b)).count() as i32)
}

#[cfg(test)]
mod tests {
    use test_case::case;

    use super::*;

    #[case(1..=5, 1..=5 => true; "when other is the same")]
    #[case(1..=5, 2..=4 => true; "when subject overlaps other fully")]
    #[case(1..=5, 3..=6 => true; "when subject overlaps other partially")]
    #[case(64..=67, 43..=63 => false; "when subject and other do not overlap")]
    fn range_incl_overlaps_trait_works_properly(
        subject: RangeInclusive<u32>,
        other: RangeInclusive<u32>,
    ) -> bool {
        subject.overlaps(&other)
    }

    #[case("1-5" => Ok(1..=5); "when string is valid")]
    #[case("A-5" => Err(()); "when start is invalid")]
    #[case("1-F" => Err(()); "when end is invalid")]
    #[case("X" => Err(()); "when string is too short")]
    #[case("X-X-X" => Err(()); "when string is too long")]
    fn str_to_range_trait_works_properly(s: &str) -> Result<RangeInclusive<u32>, ()> {
        s.to_range()
    }
}
