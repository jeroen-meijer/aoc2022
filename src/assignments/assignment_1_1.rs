use super::{Answer, Assignment};

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        1,
        1,
        "Calorie Counting".to_string(),
        Answer::Integer(74198),
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
        .max()
        .into()
}
