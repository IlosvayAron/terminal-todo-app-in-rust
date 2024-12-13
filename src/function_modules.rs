pub mod functions_and_struct {
    use std::io;
    use crate::struct_module::task_structure:: {
        Task
    };

    fn is_tasks_empty(tasks: &Vec<Task>) {
        if tasks.is_empty() {
            println!("No tasks to modify.");
            return;
        }
    }

    fn sleep() {
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    pub fn pause_for_user() {
        println!("Press Enter key to continue!");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
    }
    pub fn list_tasks(tasks: &Vec<Task>) {
        if (!tasks.is_empty()) {
            for (index, task) in tasks.iter().enumerate() {
                let status = if task.done { "Done" } else { "Pending" };
                println!("{}: {} - {} [{}]", index + 1, task.title, task.description, status);
            }

        } else {
            println!("The list is empty!");
            sleep();
            return;
        }
    }

    pub fn add_task(tasks: &mut Vec<Task>, title: &str, description: &str) {
        let task = Task::new(title, description);
        tasks.push(task);
        println!("Task was added to the list.");
        sleep();
    }

    pub fn update_task(tasks: &mut Vec<Task>) {
        if tasks.is_empty() {
             println!("No tasks to modify.");
             sleep();
             return;
        }
        // if (is_tasks_empty(&tasks)) {
        //     println!("No tasks to modify.");
        // };

        loop {
            list_tasks(&tasks);
            println!("Enter the number of the task you want to modify!");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let task_number: usize = input.trim().parse().expect("Please enter a number");

            let task_index = task_number - 1;
            let task = &mut tasks[task_index];

            println!("What would you like to modify?");
            println!("1. Title");
            println!("2. Description");
            println!("3. Mark as Done");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            let choice: u32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    return;
                }
            };

            match choice {
                1 => {
                    println!("Enter new title:");
                    let mut new_title = String::new();
                    io::stdin().read_line(&mut new_title).expect("Failed to read line");
                    task.title = new_title.trim().to_string();
                    println!("Task title updated.");
                }
                2 => {
                    println!("Enter new description:");
                    let mut new_description = String::new();
                    io::stdin().read_line(&mut new_description).expect("Failed to read line");
                    task.description = new_description.trim().to_string();
                    println!("Task description updated.");
                }
                3 => {
                    if task.done {
                        println!("Task is already marked as done.");
                    } else {
                        task.mark_done();
                        println!("Task marked as done.");
                    }
                }
                _ => println!("Invalid choice, please try again."),
            }

            println!("Want to make more modifications? Press y if yes, otherwise press n");
            let mut more_modification = String::new();
            io::stdin().read_line(&mut more_modification).expect("Failed to read line");
            let more_modification: &str  =  more_modification.trim();

            match more_modification {
            "y" => continue,
            "n" => break,
            _ => {}
            }
        }

    }

    pub fn delete_task(tasks: &mut Vec<Task>) {
        if tasks.is_empty() {
            println!("No tasks to delete!");
            sleep();
            return;
        }

        list_tasks(&tasks);
        println!("Enter the number of the task you want to delete!");

        let mut task_number = String::new();
        io::stdin().read_line(&mut task_number).expect("Failed to read line");
        let index: usize = task_number.trim().parse().expect("Please enter a number");

        tasks.remove(index-1);
        println!("Task was deleted.");
    }
}