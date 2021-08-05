use std::fs::File;
use std::io::BufReader;
use std::fs::OpenOptions;
use serde_json;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::error::Error;
use std::io::Read;


#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    is_complete: bool
}

fn main() {
    let command = std::env::args().nth(1);

    let mut tasks: Vec<Task> = Vec::new();

    if !Path::new("./tasks.txt").exists() { create_tasks_file(); }

    
    tasks.push(Task{name: String::from("Hello"), is_complete: false});

    println!("{:?}",tasks);
    save_to_file(tasks);
    
    /*
    let task = read_tasks_file(Path::new("./tasks.txt"));
    println!("{:?}", task);
    */

    /*
    
    match command.as_deref() {
        Some("help")        => { 
            println!("help\t\t\t\t: Show help");
            println!("list\t\t\t\t: List all tasks");
            println!("add \"Do homework\"\t\t: Add a task");
            println!("complete \"Do homework\"\t\t: Set a task as completed");
            println!("remove \"Do homework\"\t\t: Remove a task");
        },
        Some("add")         => {
            if std::env::args().nth(2) == None {
                println!("wrong syntax.. should use: add \"Do homework\"");
                return
            }
            println!("Task: {:?} has been added", std::env::args().nth(2).unwrap());
            tasks.push(Task{name: std::env::args().nth(2).unwrap(), is_complete: false});
            let result = save_to_file(tasks);
            println!("{:?}", result);
        },
        Some("list") => {
            println!("Listing all tasks..");
            for task in tasks {
                println!("is_complete: {} - name: {}", task.is_complete, task.name);
            }
        },
        Some("complete") => {
            let mut current_task = tasks.iter_mut().filter(|x| x.name == std::env::args().nth(2).unwrap()).next().unwrap();
            current_task.is_complete = true;
            println!("The task: {} has ben marked as completed", current_task.name);
        },
        Some("remove") => {
            println!("Removing the task: {}", std::env::args().nth(2).unwrap());
            tasks.retain(|b| b.name != std::env::args().nth(2).unwrap());
            println!("{:?}", tasks);
        },
        _ => println!("List of avaiable args: help, add, list, complete, remove"),
    }
    */
    
}



fn read_tasks_file<P: AsRef<Path>>(path: P) -> Result<Task, Box<Error>>{


    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("{:?}", file);

    println!("{:?}", contents);
    let t = serde_json::from_str(&contents).unwrap();
    Ok(t)
}

fn create_tasks_file() -> std::io::Result<()>{
    File::create("tasks.txt")?;
    Ok(())
}


fn save_to_file(tasks: Vec<Task>) -> std::io::Result<()> {
    let file = OpenOptions::new().write(true).open("tasks.txt")?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}