extern crate stopwatch;

mod assignments;
mod helpers;

use stopwatch::Stopwatch;

use assignments::{get_assignments, Answer};

fn throw_invalid_assignment_number_error() -> ! {
    println!("Invalid &assignment number.");
    println!(
        "Usage: src/main.rs <assignment_number[1 through {}]>",
        get_assignments().len()
    );
    std::process::exit(1);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 2 {
        throw_invalid_assignment_number_error();
    }

    let should_run_single_assignment = args.len() > 1;
    if should_run_single_assignment {
        let assignment_number = match args[1].parse::<i32>() {
            Ok(n) => n,
            Err(_) => throw_invalid_assignment_number_error(),
        };

        _run_single_assignment(assignment_number);
    } else {
        _run_all_assignments();
    }
}

fn _run_single_assignment(n: i32) {
    let assignments = get_assignments();

    let assignment = match assignments.get(n as usize - 1) {
        Some(a) => a,
        None => throw_invalid_assignment_number_error(),
    };

    println!(
        "Running assignment Day {} Part {}: {}",
        assignment.day, assignment.part, assignment.description
    );

    let did_succeed = match assignment.run() {
        Err(e) => {
            println!("Error while running assignment: {}", e);
            false
        }
        Ok(Answer::None) => {
            println!("No answer given.");
            false
        }
        Ok(answer) => {
            println!("Answer given: {}", answer.to_string());
            match assignment.answer {
                Answer::None => {
                    println!("No real answer to compare to.");
                    false
                }
                _ => {
                    println!("Real answer: {}", assignment.answer.to_string());
                    answer == assignment.answer
                }
            }
        }
    };

    let has_real_answer = assignment.answer != Answer::None;

    if did_succeed {
        println!("✅ Correct!");
    } else if has_real_answer {
        println!("❌ Wrong.");
    }
}

fn _run_all_assignments() {
    for assignment in get_assignments() {
        let title = format!(
            "Day {}: {} (Part {})",
            assignment.day, assignment.description, assignment.part
        );
        let result_icon: String;
        let result_description: String;

        let mut sw = Stopwatch::start_new();
        let run_result = assignment.run();
        sw.stop();

        let runtime = format!("{}ms", sw.elapsed_ms());

        match run_result {
            Ok(Answer::None) => {
                result_icon = "➖".to_string();
                result_description = "No answer.".to_string();
            }
            Err(e) => {
                result_icon = "⚠️".to_string();
                result_description = format!("Error: {}", e).to_string();
            }
            Ok(answer) => {
                let full_answer_string = format!("Answer given: {}", answer.to_string());
                match assignment.answer {
                    Answer::None => {
                        result_icon = "❓".to_string();
                        result_description =
                            format!("{}. No correct answer given.", full_answer_string);
                    }
                    real_answer => {
                        if answer == real_answer {
                            result_icon = "✅".to_string();
                            result_description = full_answer_string;
                        } else {
                            result_icon = "❌".to_string();
                            result_description = format!(
                                "{}. Correct answer: {}",
                                full_answer_string,
                                real_answer.to_string()
                            );
                        }
                    }
                }
            }
        };

        println!(
            "{} {} - [{}] {}",
            result_icon, title, runtime, result_description
        );
    }
}
