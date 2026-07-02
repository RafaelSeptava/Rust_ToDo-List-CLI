use std::io::{ self, Write };

mod models;

use models::TodoList;


fn main() {
    println!("To-Do List CLI");
    println!("Command: add<task> | list | complete <id> | delete <id> | exit");
    println!("==============================================================");

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

            "delete" => {
                if let Some(id_str) = argument {
                    match id_str.parse::<u32>() {
                        Ok(id)  => {
                            todo_list.deleted_task(id);
                        }

                        Err(_)  => {
                            println!("Error!!! Masukkan angka yang valid untuk task ID!");
                        }
                    }
                }

                else {
                    println!("Error!!! command 'delete' membutuhkan task ID!");
                }
            }

            "exit" => {
                println!("Keluar program...");
                break;
            }

            _   => {
                println!("Command tidak diketahui! Gunakan 'add', 'list', 'complete', 'delete', atau 'exit'");
            }
        }  
    }
}
