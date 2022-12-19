use std::collections::HashMap;

use super::{Answer, Assignment};

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        3,
        1,
        "Rucksack Reorganization".to_string(),
        Answer::Integer(7785),
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

fn _run(data: Vec<String>) -> Answer {
    data.iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|s| {
            (
                s.0.chars().collect::<Vec<_>>(),
                s.1.chars().collect::<Vec<_>>(),
            )
        })
        .map(|s| {
            let mut a_char_counts = HashMap::<char, u32>::new();
            let mut b_char_counts = HashMap::<char, u32>::new();

            for i in 0..s.0.len() {
                let a = s.0[i];
                let b = s.1[i];

                let a_count_in_b = *b_char_counts.get(&a).unwrap_or(&0);
                if a_count_in_b > 0 {
                    return a;
                }

                let a_count = *a_char_counts.get(&a).unwrap_or(&0);
                a_char_counts.insert(a, a_count + 1);

                let b_count_in_a = *a_char_counts.get(&b).unwrap_or(&0);
                if b_count_in_a > 0 {
                    return b;
                }

                let b_count = *b_char_counts.get(&b).unwrap_or(&0);
                b_char_counts.insert(b, b_count + 1);
            }

            panic!("This function should have already returned.")
        })
        .map(|c| _get_priority_from_char(c).unwrap())
        .sum::<u32>()
        .into()
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
