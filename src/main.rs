use std::io::{ self, Write };


#[derive(Debug, Clone, Copy, PartialEq)]
enum Status {
    Todo,
    Completed,
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    status: Status,
}

#[derive(Debug, Default)]
struct TodoList {
    tasks: Vec<Task>,
}


impl TodoList {
    fn add_task(&mut self, desc: String) {
        let next_id = self.tasks
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0) + 1;

        let new_task = Task {
            id: next_id,
            description: desc,
            status: Status::Todo,
        };

        self.tasks.push(new_task);
        println!("Menambahkan task #{}: {}", next_id, self.tasks.last().unwrap().description);
    }

    fn list_tasks(&self) {
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

    fn completed_task(&mut self, id: u32) -> bool {
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
}

fn main() {
    println!("To-Do List CLI");
    println!("Command: add<task> | list | complete <id> | exit");
    println!("================================================");

    let mut todo_list = TodoList::default();

    loop {
        print!("\n> ");
        if io::stdout()
            .flush()
            .is_err() {
                eprintln!("Gagal untuk kosongkan stdout!");
                continue;
            }
                
            

        let mut input = String::new();
        if io::stdin()
            .read_line(&mut input)
            .is_err() {
                eprintln!("Gagal untuk membaca input!");
                continue;
            }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.splitn(2, ' ');
        let command = parts.next()
            .unwrap_or("");
        let argument = parts.next();

        match command {
            "add" => {
                if let Some(desc) = argument {
                    todo_list.add_task(desc.to_string());
                }

                else {
                    println!("Error!!! command 'add' membutuhkan deskripsi task!");
                }
            }

            "list" => {
                todo_list.list_tasks();
            }

            "complete" => {
                if let Some(id_str) = argument {
                    match id_str.parse::<u32>() {
                        Ok(id)  => {
                            todo_list.completed_task(id);
                        }

                        Err(_)  => {
                            println!("Error!!! Masukkan angka yang valid untuk task ID!");
                        }
                    }
                }

                else {
                    println!("Error!!! command 'complete' membutuhkan task ID!");
                }
            }

            "exit" => {
                println!("Keluar program...");
                break;
            }

            _   => {
                println!("Command tidak diketahui! Gunakan 'add', 'list', 'complete', atau 'exit'");
            }
        }  
    }
}
