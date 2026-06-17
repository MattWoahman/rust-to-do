use std::{collections::HashMap, convert::TryFrom, fmt::Display, fs::{File, OpenOptions}, io::{self, Read, Write}, thread::sleep, time};

fn list_menu(){
    clear_screen();
    println!("What would you like to do?");
    println!("1. List tasks");
    println!("2. Create task");
    println!("3. Update task status");
    println!("4. Mark task as 'Complete'");
    println!("(press q to exit)");
    println!();
}

fn clear_screen() {
    println!("\x1B[3J\x1B[2J\x1B[1;1H");
}

fn done_prompt(is_done: &mut bool) {
        println!("\nDone? [y/n]");
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).expect("User should choose if done");
        if input.trim_end() == "y" {
            *is_done = true
        }
}

#[derive(Debug,Clone,Copy)]
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn as_str(self) -> &'static str {
            match self {
            Status::ToDo => "To-Do",
            Status::InProgress => &"In-Progress",
            Status::Done => &"Done"
        }
    }
}

// Maybe TO-DO: Implement to_status?
// trait ToStatus {
//     fn to_status(value: &str) -> Status {
//         Status::try_from(value)
//     }
// }

impl TryFrom<String> for Status {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lower_value = value.to_lowercase();
        match lower_value.as_str() {
             "todo" | "to-do" => Ok(Status::ToDo),
             "inprogress" | "in-progress" => Ok(Status::InProgress),
             "done" => Ok(Status::Done),
             _ => Err((value + " is not a valid status").to_string()),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lower_value = value.to_lowercase();
        match lower_value.as_str() {
             "todo" | "to-do" => Ok(Status::ToDo),
             "inprogress" | "in-progress" => Ok(Status::InProgress),
             "done" => Ok(Status::Done),
             _ => Err((value.to_string() + "is not a valid status").to_string()),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::ToDo => write!(f, "To-Do"),
            Status::InProgress => write!(f, "In-Progress"),
            Status::Done => write!(f, "Done")
        }
    }
}

fn list_tasks(task_list: &HashMap<String, (String,Status)>) {
    let mut is_done = false;
    while is_done == false {
        clear_screen();
        println!("Tasks:");
        for i in 1..= task_list.len() {
            println!("{}. {}, {}", i, task_list.get(&(i).to_string()).expect("should have a key").0, task_list.get(&i.to_string()).expect("should have a status").1)    
        }
        done_prompt(&mut is_done);
    }
    clear_screen();
}

fn create_task(task_list: &mut HashMap<String, (String,Status)>) {
    clear_screen();
    let mut is_unique = true;
    println!("What task do you want to add?");
    let mut new_task = String::new(); 
    io::stdin().read_line(&mut new_task).expect("User should insert task");
    for (_key, (string_value, _status_value)) in &mut *task_list {
        if *string_value.trim_end().to_string() == new_task.trim_end().to_string() {
            is_unique = false;
        }
    }
    if is_unique {
        task_list.insert((task_list.len()+1).to_string(), (new_task.trim_end().to_string(), Status::ToDo));
        println!();
        println!("Added task: {}, {}", task_list.get(&((task_list.len()).to_string())).expect("Task should be added").0, task_list.get(&((task_list.len()).to_string())).expect("Task should be added").1);
        sleep(time::Duration::from_secs(1));
    } else {
        println!();
        println!("Task already exists!");
        sleep(time::Duration::from_secs(1));
    }
}

fn update_task(task_list: &mut HashMap<String, (String,Status)>) {
    let mut is_done = false;
    while is_done == false {
        clear_screen();
        println!("What task do you want to update? [#]");
        let mut task_selection = String::new(); 
        io::stdin().read_line(&mut task_selection).expect("User should select a task");
        task_selection = task_selection.trim_end().to_string();
        let task_selection_num = match task_selection.parse::<usize>() {
            Ok(num) => num,
            Err(_err) => 0
        };
        if task_selection_num != 0 && task_selection_num <= task_list.len()  {
            println!();

            let task_name = task_list.get(&task_selection).expect("should be a valid task number").0.clone();
            println!("You selected: {}", task_selection);
            println!("Task: {}, Status: {}", task_name, task_list.get(&task_selection).expect("should be a valid task number").1);
            println!();
            
            println!("What is the new status?");
            let mut new_status = String::new(); 
            io::stdin().read_line(&mut new_status).expect("User should select a task");
            let new_status = new_status.trim_end().to_string();
            println!();
            
            match Status::try_from(new_status) {
                Ok(status) => {
                    task_list.insert(task_selection, (task_name, Status::try_from(status).expect("status")));
                    println!("Task inserted!");
                    println!();
                }
                Err(_err) => {
                    println!("Not a valid status!");
                    println!();
                }
            };
        } else {
            println!("Not a valid task number!")
        }
        done_prompt(&mut is_done);
    }
}

fn complete_task(task_list: &mut HashMap<String, (String,Status)>) {
    let mut is_done = false;
    while is_done == false {
        clear_screen();
        println!("What task do you want to complete? [#]");
        let mut task_selection = String::new(); 
        io::stdin().read_line(&mut task_selection).expect("User should select a task");
        task_selection = task_selection.trim_end().to_string();
        let task_selection_num = match task_selection.parse::<usize>() {
            Ok(num) => num,
            Err(_err) => 0
        };
        println!();
        if task_selection_num != 0 && task_selection_num <= task_list.len()  {
            println!("You selected: {}", task_selection);
            println!("Task: {}, Status: {}", task_list.get(&task_selection).expect("should be a valid task number").0, task_list.get(&task_selection).expect("should be a valid task number").1);
            println!();
            println!("Are you ready to complete it? This will remove it from the list: [y/n]");
            let mut completion_decision = String::new(); 
            io::stdin().read_line(&mut completion_decision).expect("User should select a task");
            match completion_decision.trim() {
                "y" => println!("Removed task {}", task_list.remove(&task_selection).expect("should be a valid task").0),
                "n" => println!("Continuing"),
                _  => println!("Please select a valid option!")
            };
        } else {
            println!("Not a valid task number!")
        }
        done_prompt(&mut is_done);
    }
}


fn quit(is_done: &mut bool) {
    println!("Quitting!");
    *is_done = true;
    sleep(time::Duration::from_millis(500));
    clear_screen();
}

fn tasks_file_write(task_list: &HashMap<String, (String,Status)>) {
    let mut tasks_file_content = String::new();
    let mut tasks_file = match OpenOptions::new().read(true).write(true).truncate(true).open("tasks.txt") {
        Ok(file) => file,
        Err(err) => File::create("tasks.txt").expect(&err.to_string())
    };
    for (_task_num, task) in task_list.iter() {
         tasks_file_content += (task.0.clone() + "," + task.1.as_str() + "\n").as_str()
    }
    tasks_file.write_all(&tasks_file_content.into_bytes()).expect("File should be writable");
    


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

    let mut task_file = OpenOptions::new().read(true).write(true).create(true).open("tasks.txt").expect("File should be openable");
    let mut task_file_contents = String::new();
    task_file.read_to_string(&mut task_file_contents).expect("File reading failed");
    drop(task_file);

    let mut task_list: HashMap<String, (String,Status)> = HashMap::new();

    let mut counter= 0;
    for item in task_file_contents.lines() {
        counter += 1;
        let split_item = item.split_once(',').expect("String should be delimited by a comma");
        let split_one = split_item.0;
        let split_two = Status::try_from(split_item.1).expect("should be a valid status type"); 
        task_list.insert(counter.to_string(), (split_one.to_string(), split_two));
    }

    while is_done == false {
        list_menu();
        io::stdin().read_line(&mut menu_choice).expect("failed to read choice");

        match menu_choice.as_str().trim_end() {
            "1" => list_tasks(&task_list),
            "2" => create_task(&mut task_list),
            "3" => update_task(&mut task_list),
            "4" => complete_task(&mut task_list),
            "q" => quit(&mut is_done),
            _ => print_error()
        };
        menu_choice.clear();
    }
    tasks_file_write(&task_list)

    
}
