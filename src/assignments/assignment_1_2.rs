use super::Assignment;
use itertools::Itertools;

pub fn get_assignment() -> Assignment {
    return Assignment::new(1, 2, "Calorie Counting".to_string(), Some(209914), _run);
}

fn _run(data: Vec<String>) -> Option<i32> {
    Some(
        data.split(|line| line.is_empty())
            .map(|group| {
                group
                    .iter()
                    .map(|amount| amount.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .sorted_by(|a, b| a.cmp(b).reverse())
            .take(3)
            .sum(),
    )
}
