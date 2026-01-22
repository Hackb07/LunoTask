use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const FILE_NAME: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Parser)]
#[command(name = "Task Manager")]
#[command(about = "High-Performance CLI Task Manager using Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
}

fn load_tasks() -> Vec<Task> {
    let mut file = match File::open(FILE_NAME) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILE_NAME)
        .unwrap();

    file.write_all(json.as_bytes()).unwrap();
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { title } => {
            let id = tasks.len() as u32 + 1;
            tasks.push(Task {
                id,
                title,
                completed: false,
            });
            save_tasks(&tasks);
            println!("✅ Task added successfully!");
        }

        Commands::List => {
            if tasks.is_empty() {
                println!("📭 No tasks available.");
            } else {
                for task in tasks {
                    let status = if task.completed { "✔" } else { "✘" };
                    println!("[{}] {} - {}", task.id, task.title, status);
                }
            }
        }

        Commands::Complete { id } => {
            for task in &mut tasks {
                if task.id == id {
                    task.completed = true;
                    save_tasks(&tasks);
                    println!("🎉 Task marked as completed!");
                    return;
                }
            }
            println!("❌ Task not found.");
        }

        Commands::Delete { id } => {
            let len_before = tasks.len();
            tasks.retain(|t| t.id != id);

            if tasks.len() == len_before {
                println!("❌ Task not found.");
            } else {
                save_tasks(&tasks);
                println!("🗑️ Task deleted successfully!");
            }
        }
    }
}
