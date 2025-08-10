use std::io::{self, Write};
use todo::{Todo, Priority};
use std::collections::HashMap;

fn main() {
    println!("Welcome to your todo list!");
    println!("Use command 'help' for useful tips!");
    let mut todos: HashMap<u32, Todo> = HashMap::new();
    let mut id: u32 = 1;
    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command: Vec<&str> = input.trim().split_whitespace().collect();
        if command.is_empty() {
            println!("");
            continue;
        }
        match command[0] {
            "quit" => break,
            "list" => {
                for (id, todo) in &todos {
                    println!("{}. {}", id, todo);
                }
            },
            "add" => {
                if command.len() != 3 {
                    eprintln!("Error: Must pass in a priority and task");
                    eprintln!("");
                    continue;
                }
                let priority = match command[1] {
                    "1" => Priority::High,
                    "2" => Priority::Medium,
                    "3" => Priority::Low,
                    _ => {
                        eprintln!("Error: Priority must be 1-3");
                        eprintln!("");
                        continue;
                    }
                };
                todos.insert(id, Todo::new(command[2..].join(" "), priority));
                id += 1;
            },
            "complete" => {
                if command.len() != 2 {
                    eprintln!("Error: Provide a todo id to complete");
                    eprintln!("");
                    continue;
                }
                let task_id = command[1];
                let task_id: u32 = match task_id.parse() {
                    Ok(id) => id,
                    Err(_) => {
                        eprintln!("Error: todo id must be a number");
                        eprintln!("");
                        continue;
                    }
                };
                if !todos.contains_key(&task_id) {
                    eprintln!("Error: Please provide a valid todo id");
                    eprintln!("");
                    continue;
                }
                todos.remove(&task_id);

            },
            _ => println!("Invalid command.")
        }
        println!("");
    }
}
