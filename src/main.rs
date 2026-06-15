use std::{collections::HashMap, fs, io, thread::sleep, time};

fn list_menu(){
    clear_screen();
    println!("What would you like to do?");
    println!("1. List tasks");
    println!("2. Create task");
    println!("3. Update task");
    println!("4. Mark task as 'Complete'");
    println!("(press q to exit)");
    println!();
}

fn clear_screen() {
    println!("\x1B[3J\x1B[2J\x1B[1;1H");
}

enum Status {
    ToDo,
    InProgress,
    Done
}

fn list_tasks(task_list: &HashMap<String, (String,Status)>) {
    let mut is_done = false;
    while is_done == false {
        clear_screen();
        println!("Tasks:");
        for i in 1..= task_list.len() {
            println!("{}. {}, {}", i, task_list.get(&(i).to_string()).expect("should have a key").0, task_list.get(&(i).to_string()).expect("should have a key").1)    
        }
        println!("\nDone? [y/n]");
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).expect("User should choose if done");
        if input.trim_end() == "y" {
            is_done = true
        }
    }
    clear_screen();
}

fn create_task(task_list: &mut HashMap<String, (String,String)>) {
    clear_screen();
    println!("What task do you want to add?");
    let mut new_task = String::new(); 
    io::stdin().read_line(&mut new_task).expect("User should insert task");
    task_list.insert((task_list.len()+1).to_string(), new_task, );
    println!();
    println!("Added task: {}, {}", task_list.get(&((task_list.len()).to_string())).expect("Task should be added").0, task_list.get(&((task_list.len()).to_string())).expect("Task should be added").1);
    sleep(time::Duration::from_secs(1));
    clear_screen();
}

fn update_task() {
    println!("You selected update task!");
    println!();
}

fn complete_task() {
    println!("You selected complete task!");
    println!();
}

fn quit(is_done: &mut bool) {
    println!("Quitting!");
    *is_done = true;
    sleep(time::Duration::from_millis(500));
    clear_screen();
}

fn print_error() {
    println!("That's not a valid option! Please try again.");
    println!();
}

fn main() {
    let mut menu_choice = String::new();
    let mut is_done = false;
    
    println!("To-Do List Menu:");
    println!();

    let contents: String = fs::read_to_string("tasks.txt").expect("File can't be read");
    let mut task_list: HashMap<String, (String,String)> = HashMap::new();

    let mut counter= 0;
    for item in contents.lines() {
        counter += 1;
        let split_item = item.split_once(',').expect("String should be delimited by a comma");
        let split_one = split_item.0;
        let split_two = split_item.1; 
        task_list.insert(counter.to_string(), (split_one.to_string(), split_two.to_string()));
    }

    while is_done == false {
        list_menu();
        io::stdin().read_line(&mut menu_choice).expect("failed to read choice");

        match menu_choice.as_str().trim_end() {
            "1" => list_tasks(&task_list),
            "2" => create_task(&mut task_list),
            "3" => update_task(),
            "4" => complete_task(),
            "q" => quit(&mut is_done),
            _ => print_error()
        };
        menu_choice.clear();
    }

    
}
