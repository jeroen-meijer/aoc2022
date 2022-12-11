use std::collections::HashMap;

use itertools::Itertools;

use super::Assignment;

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        3,
        2,
        "Rucksack Reorganization".to_string(),
        Some(2633),
        _run,
    );
}

fn _get_priority_from_char(c: char) -> Result<u32, ()> {
    if !c.is_ascii_alphabetic() {
        return Err(());
    }

    let offset = if c.is_lowercase() { 96 } else { 64 - 26 };

    Ok((c as u32) - offset)
}

fn _run(data: Vec<String>) -> Option<i32> {
    let ans = data
        .iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|chunk| {
            let chunk_length = chunk.len();
            let mut char_counts_per_line = vec![HashMap::<&char, u32>::new(); chunk_length];

            for i in 0..chunk_length {
                for c in &chunk[i] {
                    *char_counts_per_line[i].entry(c).or_insert(0) += 1;
                    if char_counts_per_line.iter().all(|line| line.contains_key(c)) {
                        return *c;
                    }
                }
            }

            panic!("This function should have already returned.")
        })
        .map(|c| _get_priority_from_char(c).unwrap())
        .sum::<u32>();

    Some(ans as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _get_priority_from_char_is_correct_for_lowercase_chars() {
        let letters = ('a'..='z').collect::<Vec<_>>();
        let expected_offsets = (1..=26).collect::<Vec<_>>();

        for (i, letter) in letters.into_iter().enumerate() {
            let offset = _get_priority_from_char(letter);
            assert_eq!(offset, Ok(expected_offsets[i]));
        }
    }

    #[test]
    fn _get_priority_from_char_is_correct_for_uppercase_chars() {
        let letters = ('A'..='Z').collect::<Vec<_>>();
        let expected_offsets = (27..=52).collect::<Vec<_>>();

        for (i, letter) in letters.into_iter().enumerate() {
            let offset = _get_priority_from_char(letter);
            assert_eq!(offset, Ok(expected_offsets[i]));
        }
    }
}
