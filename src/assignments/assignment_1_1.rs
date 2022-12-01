use super::Assignment;

pub fn get_assignment() -> Assignment {
    return Assignment::new(1, 1, "Calorie Counting".to_string(), Some(74198), _run);
}

fn _run(data: Vec<String>) -> Option<i32> {
    data.split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|amount| amount.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
}
