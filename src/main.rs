use std::io::stdin;
use todo_list::{self, complete_task, delete_task, format_list, Task};
fn main() {
    println!("Welcome to Sabien's Todo-List!");
    handle_user();
}

fn handle_user() {
    let mut task_list: Vec<Task> = Vec::new();
    let mut input = String::new();
    loop {
        println!("Enter a command: (add, delete, display, quit, complete)");

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

                println!("Enter quadrant: ");
                let mut quadrant = String::new();
                stdin()
                    .read_line(&mut quadrant)
                    .expect("Failed to read line");
                let quadrant: u8 = quadrant.trim().parse().expect("enter a valid number");
                todo_list::add_task(&mut task_list, task_name, quadrant);
            }
            "delete" => {
                println!("Enter task position: ");
                let mut string = String::new();

                stdin().read_line(&mut string).expect("Failed to read line");
                let task_pos: u8 = string.trim().parse().expect("enter a valid number");
                delete_task(&mut task_list, task_pos);
            }
            "complete" => {
                complete_task(&mut task_list);
            }
            "display" => println!("{}", format_list(&task_list)),
            "quit" => break,
            _ => println!("use valid command!"),
        }
    }

    println!("Bye!");
}
