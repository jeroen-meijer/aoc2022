use std::collections::HashMap;

use itertools::Itertools;

use super::{Answer, Assignment};

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        5,
        2,
        "Supply Stacks".to_string(),
        // cspell:disable-next
        Answer::String("SSCGWJCRB".to_string()),
        _run,
    );
}

fn _run(data: Vec<String>) -> Answer {
    let mut raw_state_lines: Vec<String> = vec![];
    let mut raw_instruction_lines: Vec<String> = vec![];

    for line in data {
        if line.trim().is_empty() {
            continue;
        } else if line.starts_with("move") {
            raw_instruction_lines.push(line);
        } else {
            raw_state_lines.push(line);
        }
    }

    let mut state = _parse_raw_state(raw_state_lines);
    let instructions = _parse_raw_instructions(raw_instruction_lines);

    for inst in instructions {
        let source_chars = state.get_mut(&inst.source_id).unwrap();

        let chars_to_move_start_index = source_chars.len() - (inst.amount as usize);
        let chars_to_move = source_chars
            .drain(chars_to_move_start_index..)
            .collect_vec();

        state
            .get_mut(&inst.target_id)
            .unwrap()
            .extend(chars_to_move);
    }

    let top_chars = state
        .iter()
        .sorted_by(|(id1, _), (id2, _)| id1.cmp(id2))
        .map(|(_, v)| v.last())
        .filter_map(|c| c)
        .join("");

    Answer::String(top_chars)
}

type State = HashMap<u32, Vec<char>>;

fn _parse_raw_state(lines: Vec<String>) -> State {
    let reversed = lines.iter().rev().collect_vec();

    let number_line_chars = reversed.first().unwrap().chars().enumerate();
    let state_char_lines = reversed
        .iter()
        .skip(1)
        .map(|line| line.chars().collect_vec());

    let mut result = HashMap::<u32, Vec<char>>::new();

    for (index, c) in number_line_chars {
        if !c.is_numeric() {
            continue;
        }

        let id = c.to_string().parse::<u32>().unwrap();
        let mut initial_state: Vec<char> = vec![];

        for chars in state_char_lines.clone() {
            if chars.len() > index && chars[index].is_ascii_uppercase() {
                initial_state.push(chars[index]);
            }
        }
        result.insert(id, initial_state);
    }

    result
}

#[derive(Debug)]
struct Instruction {
    amount: u32,
    source_id: u32,
    target_id: u32,
}

fn _parse_raw_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .filter(|line| line.trim().starts_with("move"))
        .map(|line| {
            let numbers = line
                .split(' ')
                .filter_map(|part| part.parse::<u32>().ok())
                .collect_vec();
            if numbers.len() != 3 {
                panic!("Line invalid: {line}");
            }

            Instruction {
                amount: numbers[0],
                source_id: numbers[1],
                target_id: numbers[2],
            }
        })
        .collect_vec()
}
