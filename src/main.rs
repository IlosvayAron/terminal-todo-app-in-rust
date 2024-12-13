mod function_modules;
mod struct_module;

use crate::function_modules::functions_and_struct:: {
    list_tasks,
    add_task,
    pause_for_user,
    update_task,
    delete_task
};

use std::io;

fn main() {
    let mut tasks = Vec::new();

    loop {
        println!("\n1. Add Task");
        println!("2. List Tasks");
        println!("3. Update Task");
        println!("4. Delete Task");
        println!("5. Exit");
        let question: &str = "What do you want to do?";
        let instruction: &str = "Type in the appropriate number: ";
        println!("{} {}", question, instruction);
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");

        match choice {
            1 => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");

                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");

                add_task(&mut tasks, title.trim(), description.trim());
            }
            2 => {
                list_tasks(&tasks);
                pause_for_user();
            },
            3 => {
                update_task(&mut tasks);
            },
            4 => delete_task(&mut tasks),
            5 => break println!("You have exited the ToDo app. Goodbye!"),
            _ => println!("Invalid choice, please try again."),
        }
    }
}
