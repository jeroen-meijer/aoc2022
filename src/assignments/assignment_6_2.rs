use std::collections::HashMap;

use itertools::Itertools;

use super::{Answer, Assignment};

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        6,
        2,
        "Tuning Trouble".to_string(),
        Answer::Integer(2334),
        _run,
    );
}

const WINDOW_SIZE: usize = 14;

fn _run(data: Vec<String>) -> Answer {
    let chars = data.first().unwrap().chars().collect_vec();

    let mut char_count_in_window = HashMap::<char, u32>::new();

    for i in 0..chars.len() {
        let current_char = chars[i];
        *char_count_in_window.entry(current_char).or_insert(0) += 1;

        if i < WINDOW_SIZE {
            continue;
        }

        let char_outside_window = chars[i - WINDOW_SIZE];
        let char_outside_window_value = char_count_in_window.get(&char_outside_window).unwrap();
        if *char_outside_window_value == 1 {
            char_count_in_window.remove(&char_outside_window);
        } else {
            char_count_in_window
                .entry(char_outside_window)
                .and_modify(|v| *v -= 1);
        }

        if char_count_in_window.values().all(|v| *v == 1) {
            return Answer::Integer(i as u32 + 1);
        }
    }

    panic!("An answer should have been returned.")
}
