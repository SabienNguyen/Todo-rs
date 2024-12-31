use std::io::stdin;
use todo_list::{self, format_list};
fn main() {
    println!("Welcome to Sabien's Todo-List!");
    handle_user();
}

fn handle_user() {
    let mut task_list: Vec<String> = Vec::new();
    let mut input = String::new();
    loop {
        println!("Enter a command: (add, delete, display, quit)");

        input.clear();

        stdin().read_line(&mut input).expect("Failed to read line");

        let command = input.trim();

        match command {
            "add" => {
                println!("Enter task name: ");
                let mut task_name = String::new();

                stdin()
                    .read_line(&mut task_name)
                    .expect("Failed to read line");
                todo_list::add_task(&mut task_list, task_name);
            }
            "delete" => println!("called delete command"),
            "display" => println!("{}", format_list(&task_list)),
            "quit" => {
                println!("called quit command");
                break;
            }
            _ => println!("use valid command!"),
        }
    }

    println!("Bye!");
}
