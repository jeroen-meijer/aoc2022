use super::{Answer, Assignment};
use itertools::Itertools;

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        1,
        2,
        "Calorie Counting".to_string(),
        Answer::Integer(209914),
        _run,
    );
}

fn _run(data: Vec<String>) -> Answer {
    data.split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|amount| amount.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted_by(|a, b| a.cmp(b).reverse())
        .take(3)
        .sum::<i32>()
        .into()
}
