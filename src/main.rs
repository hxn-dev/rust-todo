use std::fs::File;
use std::io::BufReader;
use std::fs::OpenOptions;
use serde_json;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::error::Error;


#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq)]
struct Task {
    name: String,
    is_complete: bool
}

fn main() {
    let command = std::env::args().nth(1);
    let mut tasks = Vec::new();

    if !Path::new("tasks.json").exists() { 
        match create_tasks_file() {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    } else {
        if check_tasks_file_size() > 0 {
            match read_tasks_file(Path::new("tasks.json")) {
                Err(e) => println!("{:?}", e),
                Ok(e) => { tasks = e }
            }
        }
    }    
    
    match command.as_deref() {
        Some("help") => { 
            println!("help\t\t\t\t: Show help");
            println!("list\t\t\t\t: List all tasks");
            println!("add \"Do homework\"\t\t: Add a task");
            println!("complete \"Do homework\"\t\t: Mark task as completed");
            println!("uncomplete \"Do homework\"\t: Mark task as uncompleted");
            println!("remove \"Do homework\"\t\t: Remove a task");
        },
        Some("add") => {
            if std::env::args().nth(2) == None {
                println!("wrong syntax.. use: add \"Do homework\"");
                return
            }
            match find_task_by_name(std::env::args().nth(2).unwrap(), &mut tasks) {
                None => {
                    println!("Task: {:?} has been added", std::env::args().nth(2).unwrap());
                    tasks.push(Task{name: std::env::args().nth(2).unwrap(), is_complete: false});
                    match save_to_file(&mut tasks) {
                        Err(e) => println!("{:?}", e),
                        _ => ()
                    }
                },
                Some(task) => {
                    println!("Task: {} already exist", task.name);
                }
            }
        },
        Some("list") => {
            println!("Listing all tasks.. {:?} task(s) found", tasks.len());
            tasks.sort_by(|a, b| a.is_complete.cmp(&b.is_complete));
            for task in &mut tasks {
                let is_complete = if task.is_complete == true { "X" } else { "_" };
                println!("[{}] - {}", is_complete, task.name);
            }
        },
        Some("complete") => {
            if !task_name_given() {
                return
            }

            match find_task_by_name(std::env::args().nth(2).unwrap(), &mut tasks) {
                None => println!("Couldn't find task: {:?}", std::env::args().nth(2).unwrap().to_lowercase()),
                Some(task) => {
                    task.is_complete = true;
                    println!("The task: {} has been marked as completed", task.name);
                    match save_to_file(&mut tasks) {
                        Err(e) => println!("{:?}", e),
                        _ => ()
                    }
                },
            }
        },
        Some("uncomplete") => {
            if !task_name_given() {
                return
            }

            match find_task_by_name(std::env::args().nth(2).unwrap(), &mut tasks) {
                None => println!("Couldn't find task: {:?}", std::env::args().nth(2).unwrap().to_lowercase()),
                Some(task) => {
                    task.is_complete = false;
                    println!("The task: {} has been marked as uncompleted", task.name);
                    match save_to_file(&mut tasks) {
                        Err(e) => println!("{:?}", e),
                        _ => ()
                    }
                },
            }
        },
        Some("remove") => {
            if !task_name_given() {
                return
            }

            let tasks_count = tasks.len();
            tasks.retain(|b| (b.name).to_lowercase() != std::env::args().nth(2).unwrap().to_lowercase());
            let new_tasks_count = tasks.len();
            if tasks_count > new_tasks_count { 
                println!("Task: {:?} has been removed.", std::env::args().nth(2).unwrap()); 
                match save_to_file(&mut tasks) {
                    Err(e) => println!("{:?}", e),
                    _ => ()
                }
            } else { 
                println!("0 task removed. Couldn't find task: {:?}", std::env::args().nth(2).unwrap());
            }
        },
        Some("save") => {
            match save_to_file(&mut tasks) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
        }
        _ => println!("List of avaiable args: help, add, list, complete, remove"),
    }
}

fn find_task_by_name(task_name: String, tasks: &mut Vec<Task>) -> Option<&mut Task> {
    let current_task = tasks.iter_mut().filter(|x| x.name.to_lowercase() == task_name.to_lowercase()).next();
    if current_task == None { 
        None 
    } else {
        current_task
    }   
}

fn task_name_given() -> bool {
    if std::env::args().nth(2) == None {
        println!("Task name is missing");
        false
    } else {
        true
    }
}

fn check_tasks_file_size() -> u64 {
    let file = File::open("tasks.json").expect("File error");
    let metadata = file.metadata().unwrap();
    metadata.len()
}

fn read_tasks_file<P: AsRef<Path>>(path: P) -> Result<Vec<Task>, Box<dyn Error>>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let t = serde_json::from_reader(reader)?;
    Ok(t)
}

fn create_tasks_file() -> std::io::Result<()>{
    File::create("tasks.json")?;
    Ok(())
}


fn save_to_file(tasks: &Vec<Task>) -> Result<bool, Box<dyn Error>> {
    let file = OpenOptions::new().write(true).truncate(true).open("tasks.json")?;
    serde_json::to_writer(file, &tasks)?;
    Ok(true)
}