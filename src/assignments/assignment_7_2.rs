use core::panic;
use std::collections::HashMap;

use itertools::Itertools;

use super::{Answer, Assignment};

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        7,
        2,
        "No Space Left On Device".to_string(),
        Answer::Integer(1544176),
        _run,
    );
}

const TOTAL_DISK_SIZE: u32 = 70000000;
const REQUIRED_SPACE: u32 = 30000000;
const ROOT_DIR: &str = "/";
const POP_DIR: &str = "..";

fn _run(data: Vec<String>) -> Answer {
    let commands = _parse_commands(data);
    let dir_graph = _create_dir_graph(commands);

    let dirs = dir_graph.dirs_vec_depth_first();
    let dirs_with_sizes = dirs.iter().map(|dir| (&dir.name, dir.get_size()));

    let root_dir_size = dirs.first().unwrap().get_size();
    let currently_available_space = TOTAL_DISK_SIZE - root_dir_size;
    let additional_required_space = REQUIRED_SPACE - currently_available_space;

    Answer::Integer(
        dirs_with_sizes
            .map(|(_, size)| size)
            .filter(|size| size >= &additional_required_space)
            .min()
            .unwrap(),
    )
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct Directory {
    name: String,
    entities: SystemEntityGraph,
}

#[derive(Debug)]
enum Command {
    ChangeDir { dirname: String },
    List { files: Vec<File> },
}

fn _parse_commands(lines: Vec<String>) -> Vec<Command> {
    let mut commands = Vec::<Command>::new();
    let mut current_files = Vec::<File>::new();

    let mut line_index = 0;
    while line_index < lines.len() {
        let line = &lines[line_index];
        let line_parts = line.split(' ');
        let is_command = line.starts_with("$");

        if is_command {
            if !current_files.is_empty() {
                commands.push(Command::List {
                    files: current_files.drain(0..current_files.len()).collect_vec(),
                });
            }

            let command_name = line_parts.clone().collect_vec()[1];
            match command_name {
                "cd" => {
                    let dirname = line_parts.last().unwrap();
                    commands.push(Command::ChangeDir {
                        dirname: dirname.to_string(),
                    });
                }
                "ls" => {}
                other => panic!("Unrecognized command: {}", other),
            }
        } else {
            let (meta, name) = line_parts.collect_tuple().unwrap();

            match meta {
                "dir" => {}
                size => {
                    current_files.push(File {
                        size: size.parse().unwrap(),
                        name: name.to_string(),
                    });
                }
            };
        }

        line_index += 1;
    }

    if !current_files.is_empty() {
        commands.push(Command::List {
            files: current_files.drain(0..current_files.len()).collect_vec(),
        });
    }

    commands
}

#[derive(Debug)]
enum SystemEntity {
    Directory(Directory),
    File(File),
}

impl Directory {
    fn get_size(&self) -> u32 {
        self.entities
            .iter()
            .map(|(_, entity)| entity.get_size())
            .sum()
    }
}

impl SystemEntity {
    fn get_size(&self) -> u32 {
        match self {
            SystemEntity::File(file) => file.size,
            SystemEntity::Directory(dir) => dir.get_size(),
        }
    }
}

type SystemEntityGraph = HashMap<String, SystemEntity>;

trait SystemEntityGraphTraits {
    fn get_or_create_dir_deep_mut(&mut self, path: Vec<String>) -> &mut SystemEntityGraph;

    fn dirs_vec_depth_first(&self) -> Vec<&Directory>;
}

impl SystemEntityGraphTraits for SystemEntityGraph {
    fn get_or_create_dir_deep_mut(&mut self, path: Vec<String>) -> &mut SystemEntityGraph {
        let mut path_parts_to_process = path.clone();
        path_parts_to_process.reverse();

        let mut result = self;

        while !path_parts_to_process.is_empty() {
            let subpath = path_parts_to_process.pop().unwrap();

            let next = result
                .entry(subpath.clone())
                .or_insert(SystemEntity::Directory(Directory {
                    name: subpath.clone(),
                    entities: SystemEntityGraph::new(),
                }));
            result = match next {
                SystemEntity::Directory(dir) => &mut dir.entities,
                _ => panic!("Unreachable"),
            }
        }

        result
    }

    fn dirs_vec_depth_first(&self) -> Vec<&Directory> {
        let mut result: Vec<&Directory> = vec![];

        for dir in self.values() {
            match dir {
                SystemEntity::Directory(dir) => {
                    result.push(dir);
                    result.extend(dir.entities.dirs_vec_depth_first());
                }
                _ => {}
            }
        }

        result
    }
}

fn _create_dir_graph(commands: Vec<Command>) -> SystemEntityGraph {
    let mut graph = SystemEntityGraph::new();

    let mut current_path = vec![ROOT_DIR.to_string()];

    for command in commands {
        match command {
            Command::ChangeDir { dirname } => match dirname.as_str() {
                ROOT_DIR => {
                    current_path.clear();
                    current_path.push(ROOT_DIR.to_string());
                }
                POP_DIR => {
                    current_path.pop();
                }
                dirname => {
                    current_path.push(dirname.to_string());
                }
            },
            Command::List { files } => {
                for file in files {
                    let graph_at_path = graph.get_or_create_dir_deep_mut(current_path.clone());
                    graph_at_path.insert(file.name.clone(), SystemEntity::File(file));
                }
            }
        }
    }

    graph
}
