use std::fmt;
use colored::*;
use std::{collections::HashMap, fs, path::PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub priority: Priority,
    pub task: String,
}

impl Todo {
    pub fn new(task: String, priority: Priority) -> Todo {
        Todo { priority, task }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_star = match self.priority {
            Priority::High => "[High]".red(),
            Priority::Medium => "[Medium]".yellow(),
            Priority::Low => "[Low]".green(),
        };
        write!(f, "{} {}", colored_star, self.task)
    }
}

fn storage_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannt find home directory");
    path.push(".todo_list.json");
    path
}

fn load_todos() -> HashMap<u32, Todo> {
    let path = storage_path();
    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_todos(todos: &HashMap<u32, Todo>) {
    let path = storage_path();
    let data = serde_json::to_string_pretty(todos).unwrap();
    fs::write(path, data).unwrap();
}