use pchryss_todo_cli::{Todo, Priority, load_todos, save_todos};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: todo <command> [args]");
        return;
    }

    let command = &args[1];
    let mut todos = load_todos();

    match command.as_str() {
        "list" => {
            for (id, todo) in &todos {
                println!("{}. {}", id, todo);
            }
        },
        "add" => {
            if command.len() < 4 {
                eprintln!("Usage: todo add <priority> <task>");
                return;
            }
            let priority = match args[2].as_str() {
                "1" => Priority::High,
                "2" => Priority::Medium,
                "3" => Priority::Low,
                _ => {
                    eprintln!("Error: Priority must be 1-3");
                    return;
                }
            };
            let id = if todos.is_empty() {
                1
            } else {
                todos.keys().max().unwrap() + 1
            };
            let task = args[3..].join(" ");
            todos.insert(id, Todo { task, priority });
            save_todos(&todos);
            println!("Added todo #{}", id);
        },
        "complete" => {
            if args.len() != 3 {
                eprintln!("Usage: todo complete <id>");
                return;
            }
            let id: u32 = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("ID must be a number");
                    return;
                }
            };
            if todos.remove(&id).is_some() {
                save_todos(&todos);
                println!("Completed todo #{}", id);
            } else {
                eprintln!("No todo with id {}", id);
            }

        },
        "help" => {
            println!("
            pchryss-todo-cli - A simple persistent todo CLI tool

            Usage:
            todo list
                Lists all todos.

            todo add <priority> <task>
                Adds a new todo with priority (1=High, 2=Medium, 3=Low).
                Example: todo add 1 \"Buy groceries\"

            todo complete <id>
                Completes and removes the todo with the given id.
                Example: todo complete 3

            todo help
                Displays this help message.
            ");
        },
        _ => eprintln!("Invalid command.")
    }
}
