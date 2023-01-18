use std::collections::HashMap;

use itertools::Itertools;

use super::{Answer, Assignment};

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        8,
        1,
        "Treetop Tree House".to_string(),
        Answer::Integer(1801),
        _run,
    );
}

struct VerticalIter<T> {
    data: Vec<Vec<T>>,
    index: usize,
}

impl<T> Iterator for VerticalIter<T>
where
    T: Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner_len = self.data[0].len();

        if self.index == inner_len {
            return None;
        }

        let mut item = vec![];

        for row in self.data.iter() {
            let e = row[self.index];
            item.push(e);
        }

        self.index += 1;

        Some(item)
    }
}

trait IntoVerticalIter<T> {
    fn into_vertical_iter(self) -> VerticalIter<T>;
}

impl<T> IntoVerticalIter<T> for Vec<Vec<T>> {
    fn into_vertical_iter(self) -> VerticalIter<T> {
        let inner_len = self[0].len();
        assert!(
            self.iter().all(|l| l.len() == inner_len),
            "Every vec must have the same size."
        );
        VerticalIter {
            data: self,
            index: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    pos: (usize, usize),
    height: u8,
}

fn _run(data: Vec<String>) -> Answer {
    let numbers = data
        .iter()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(num_index, c)| Point {
                    pos: (num_index, row_index),
                    height: c.to_string().parse::<u8>().unwrap(),
                })
                .collect_vec()
        })
        .collect_vec();

    let left_right_iter = numbers.clone();
    let top_down_iter = numbers.clone().into_vertical_iter().collect_vec();
    let right_left_iter = numbers
        .clone()
        .iter()
        .map(|row| {
            let mut new = row.clone();
            new.reverse();
            new
        })
        .collect_vec();
    let down_top_iter = numbers
        .clone()
        .into_vertical_iter()
        .map(|row| {
            let mut new = row.clone();
            new.reverse();
            new
        })
        .collect_vec();

    let all_rows = vec![
        left_right_iter,
        top_down_iter,
        right_left_iter,
        down_top_iter,
    ];
    let all_rows = all_rows.iter().flatten().collect_vec();

    let mut visible_points = HashMap::<(usize, usize), u8>::new();

    for row in all_rows {
        let first = row.first().unwrap();
        visible_points.insert(first.pos, first.height);

        row.iter().reduce(|highest, item| {
            if item.height <= highest.height {
                highest
            } else {
                visible_points.insert(item.pos, item.height);
                item
            }
        });
    }

    Answer::Integer(visible_points.len() as u32)
}
