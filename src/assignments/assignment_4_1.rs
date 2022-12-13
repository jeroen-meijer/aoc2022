use std::str::FromStr;

use super::Assignment;

pub fn get_assignment() -> Assignment {
    return Assignment::new(4, 1, "Camp Cleanup".to_string(), Some(599), _run);
}

#[derive(PartialEq, Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

impl FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('-').collect::<Vec<_>>();
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

        Ok(Section {
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

fn _run(data: Vec<String>) -> Option<i32> {
    let mut num_overlap = 0;
    for line in data {
        let mut pair_strings = line.split(',');
        let a = pair_strings.next().unwrap().parse::<Section>().unwrap();
        let b = pair_strings.next().unwrap().parse::<Section>().unwrap();

        if a.contains(&b) || b.contains(&a) {
            num_overlap += 1;
        }
    }

    Some(num_overlap)
}

#[cfg(test)]
mod tests {
    use test_case::case;

    use super::*;

    #[case(Section{start: 1, end: 5}, Section{start: 1, end: 5} => true; "when other is the same")]
    #[case(Section{start: 1, end: 5}, Section{start: 2, end: 4} => true; "when subject fully contains other")]
    #[case(Section{start: 1, end: 5}, Section{start: 3, end: 6} => false; "when subject does not fully contain other")]
    fn section_contains_works_properly(subject: Section, other: Section) -> bool {
        subject.contains(&other)
    }

    #[case("1-5" => Ok(Section{ start: 1, end: 5}); "when string is valid")]
    #[case("A-5" => Err(()); "when start is invalid")]
    #[case("1-F" => Err(()); "when end is invalid")]
    #[case("X" => Err(()); "when string is too short")]
    #[case("X-X-X" => Err(()); "when string is too long")]
    fn section_from_str_trait_works_properly(s: &str) -> Result<Section, ()> {
        s.parse()
    }
}
