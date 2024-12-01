use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::{env, process};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.completed { "[x]" } else { "[ ]" };
        write!(f, "{} {}: {}", self.id, status, self.description)
    }
}

fn load_tasks(filename: &str) -> io::Result<Vec<Task>> {
    if !Path::new(filename).exists() {
        return Ok(Vec::new());
    }

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

fn save_tasks(filename: &str, tasks: &Vec<Task>) -> io::Result<()> {
    let file = File::create(filename)?;
    serde_json::to_writer_pretty(file, tasks)?;
    Ok(())
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = match tasks.last() {
        Some(task) => task.id + 1,
        None => 1,
    };
    let task = Task {
        id,
        description,
        completed: false,
    };
    tasks.push(task);
    println!("Task added successfully.");
}

fn remove_task(tasks: &mut Vec<Task>, id: usize) -> Result<(), String> {
    if let Some(pos) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(pos);
        println!("Task removed successfully.");
        Ok(())
    } else {
        Err(format!("No task found with ID {}", id))
    }
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for task in tasks {
            println!("{}", task);
        }
    }
}

fn print_help() {
    println!("Usage:");
    println!("  todo add \"Task description\"    Add a new task");
    println!("  todo remove <task_id>           Remove a task by ID");
    println!("  todo view                       View all tasks");
    println!("  todo help                       Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = "tasks.json";

    let mut tasks = match load_tasks(filename) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            process::exit(1);
        }
    };

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: No task description provided.");
                print_help();
                process::exit(1);
            }
            let description = args[2..].join(" ");
            add_task(&mut tasks, description);
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Error: No task ID provided.");
                print_help();
                process::exit(1);
            }
            let id = match args[2].parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Error: Task ID must be a number.");
                    process::exit(1);
                }
            };
            if let Err(e) = remove_task(&mut tasks, id) {
                eprintln!("Error: {}", e);
            }
        }
        "view" => {
            view_tasks(&tasks);
        }
        "help" => {
            print_help();
        }
        _ => {
            eprintln!("Error: Unknown command '{}'.", args[1]);
            print_help();
            process::exit(1);
        }
    }

    if let Err(e) = save_tasks(filename, &tasks) {
        eprintln!("Error saving tasks: {}", e);
        process::exit(1);
    }
}
