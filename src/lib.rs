use std::fmt;
use colored::*;
pub enum Priority {
    High,
    Medium,
    Low
}

pub struct Todo {
    task: String,
    priority: Priority,
}

impl Todo {
    pub fn new(task: String, priority: Priority) -> Todo {
        Todo { task , priority}
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