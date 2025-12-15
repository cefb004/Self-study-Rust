#[derive(Debug, Clone)]
struct Task {
    id: u64,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: u64, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

#[derive(Debug)]
enum Command {
    Add(String),
    List,
    Complete(u64),
    Remove(u64),
    Exit,
}

fn add_task(tasks: &mut Vec<Task>, title: String) {
    let id = tasks.len() as u64 + 1;
    let task = Task::new(id, title);
    tasks.push(task);
}

fn find_task(tasks: &Vec<Task>, id: u64) -> Option<&Task> {
    tasks.iter().find(|task| task.id == id)
}

fn complete_task(tasks: &mut Vec<Task>, id: u64) -> Option<()> {
    let task = tasks.iter_mut().find(|task| task.id == id)?;
    task.complete();
    Some(())
}


fn execute_command(command: Command, tasks: &mut Vec<Task>) {
    match command {
        Command::Add(title) => {
            add_task(tasks, title);
        }
        Command::List => {
            for task in tasks {
                println!(
                    "- [{}] {} (id={})",
                    if task.completed { "x" } else { " " },
                    task.title,
                    task.id
                );
            }
        }
        Command::Complete(id) => {
            if complete_task(tasks, id).is_some() {
                println!("Task {} completed", id);
            } else {
                println!("Task {} not found", id);
            }
        }
        Command::Remove(_id) => {
            println!("Remove not implemented yet");
        }
        Command::Exit => {
            println!("Exiting...");
        }
    }
}


fn main() {

    let mut tasks: Vec<Task> = Vec::new();

    execute_command(Command::Add("Learn Rust".to_string()), &mut tasks);
    execute_command(Command::Add("Build CLI".to_string()), &mut tasks);
    execute_command(Command::List, &mut tasks);
    execute_command(Command::Complete(1), &mut tasks);
    execute_command(Command::List, &mut tasks);

}
