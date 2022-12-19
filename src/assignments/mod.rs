mod assignment_1_1;
mod assignment_1_2;
mod assignment_2_1;
mod assignment_3_1;
mod assignment_3_2;
mod assignment_4_1;
mod assignment_4_2;

use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
pub enum Answer {
    None,
    Integer(u32),
    String(String),
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        match self {
            Answer::None => "None".to_string(),
            Answer::Integer(i) => format!("(u32) {i}"),
            Answer::String(s) => format!("(String) {s}"),
        }
    }
}

impl From<u32> for Answer {
    fn from(value: u32) -> Self {
        Answer::Integer(value)
    }
}

impl From<Option<u32>> for Answer {
    fn from(value: Option<u32>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Answer::None,
        }
    }
}

impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        if value < 0 {
            panic!("Integer value has to be positive.");
        }
        Answer::Integer(value as u32)
    }
}

impl From<Option<i32>> for Answer {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Answer::None,
        }
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Answer::String(value)
    }
}

impl From<Option<String>> for Answer {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Answer::None,
        }
    }
}

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl From<Option<&str>> for Answer {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Answer::None,
        }
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Answer::Integer(value as u32)
    }
}

pub fn get_assignments() -> Vec<Assignment> {
    return vec![
        assignment_1_1::get_assignment(),
        assignment_1_2::get_assignment(),
        assignment_2_1::get_assignment(),
        assignment_3_1::get_assignment(),
        assignment_3_2::get_assignment(),
        assignment_4_1::get_assignment(),
        assignment_4_2::get_assignment(),
        assignment_5_1::get_assignment(),
    ];
}

pub struct Assignment {
    pub day: i32,
    pub part: i32,
    pub description: String,
    pub answer: Answer,
    _f: InternalAssignmentCallback,
}

type InternalAssignmentCallback = fn(data: Vec<String>) -> Answer;

impl Assignment {
    pub fn new(
        day: i32,
        part: i32,
        description: String,
        answer: Answer,
        run: InternalAssignmentCallback,
    ) -> Assignment {
        return Assignment {
            day,
            part,
            description,
            answer,
            _f: run,
        };
    }

    pub fn run(&self) -> Result<Answer, String> {
        // Reads the file <id>.txt and returns the contents as a vector of strings.
        let path = format!("src/assignments/assignment_{}.txt", self.day);
        let data = _read_lines(&path)
            .and_then(|lines| lines.collect::<Result<Vec<String>, io::Error>>())
            .map_err(|e| format!("Could not read file at {}\nError: {}", &path, e))?;

        return Ok((self._f)(data));
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
