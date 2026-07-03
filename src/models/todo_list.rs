// todo_list.rs
use super::{ Status, Task, TodoList };
use std::fs::File;
use std::io::{ BufRead, BufReader, Write };
use std::str::FromStr;
use std::fmt;


impl TodoList {
    pub fn add_task(&mut self, desc: String) {
        let next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        let new_task = Task {
            id: next_id, 
            description: desc.clone(), 
            status: Status::Todo,
        };

        self.tasks.push(new_task);
        println!("Menambahkan task #{}: {}", next_id, self.tasks.last().unwrap().description);
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("To-Do List kosong!"); 
            return;
        }

        println!("\n-----Daftar To-Do List-----");
        for task in &self.tasks {
            let status_marker = match task.status {
                Status::Todo        => "[ ]",
                Status::Completed   => "[✓]",
            };

            println!("{} #{}: {}", status_marker, task.id, task.description);
        }

        println!("---------------------------");
    }

    pub fn completed_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = Status::Completed;
            println!("Task #{} sudah selesai!", id);
            true
        }

        else { 
            println!("Task dengan ID #{} tidak ditemukan!", id);
            false
        }
    }

    pub fn deleted_task(&mut self, id: u32) -> bool {
        let original_length = self.tasks.len();

        self.tasks.retain(|task| task.id != id);

        if self.tasks.len() < original_length {
            println!("Task #{} berhasil dihapus!", id); 
            true
        }

        else {
            println!("Task dengan ID #{} tidak ditemukan!", id);
            false
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        for task in &self.tasks {
            writeln!(file, "{} | {} | {}", task.id, task.status, task.description)?;
        }

        Ok(())
    }

    pub fn load(filename: &str) -> std::io::Result<Self> {
        let mut todo_list = TodoList { tasks: Vec::new() };

        let file = match File::open(filename) {
            Ok(file)    => file,
            Err(_)  => return Ok(todo_list),
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;

            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue; 
            }

            let parts: Vec<&str> = trimmed_line.split('|').collect();

            if parts.len() == 3 {
                let id = parts[0].trim().parse::<u32>().unwrap_or(0);
                let status = parts[1].trim().parse::<Status>().unwrap_or(Status::Todo);
                let description = parts[2].trim().to_string();

                todo_list.tasks.push(Task {id, description, status} );
            }
        }

        Ok(todo_list)
    }
}


impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Todo" => Ok(Status::Todo),
            "Completed" => Ok(Status::Completed),
            _ => Err(()),
        }
    }
}


pub fn parse_id(argument: Option<&str>) -> Result<u32, &'static str> {
    let id_str = argument.ok_or("Error!!! command ini membutuhkan task ID!")?;
    id_str.parse::<u32>().map_err(|_| "Error!!! Masukkan angka yang valid untuk task ID!")
}
