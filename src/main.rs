// main.rs
use std::io::{ self, Write };

mod models;
use models::TodoList;

use crate::models::todo_list::parse_id;


fn main() {
    let filename = "to-do_list.txt";

    println!("To-Do List CLI");
    println!("Command: add<task> | list | complete <id> | delete <id> | exit");
    println!("==============================================================");

    let mut todo_list = TodoList::load(filename).unwrap_or_else(|_| {
        eprintln!("Gagal memuat file! Memulai list baru!");
        TodoList::default()
    });

    loop {
        print!("\n> ");
        if io::stdout().flush().is_err() { 
            eprintln!("Gagal untuk kosongkan stdout!"); 
            continue; 
        }            

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { 
            eprintln!("Gagal untuk membaca input!"); 
            continue; 
        }

        let trimmed = input.trim();
        if trimmed.is_empty() { 
            continue; 
        }

        let mut parts = trimmed.splitn(2, ' ');
        let command = parts.next().unwrap_or("");
        let argument = parts.next();

        match command {
            "add"       =>  {
                if let Some(desc) = argument {
                    todo_list.add_task(desc.to_string());
                }

                else {
                    println!("Error!!! command 'add' membutuhkan deskripsi task!"); 
                }
            }

            "list"      =>  {
                todo_list.list_tasks(); 
            }

            "complete"  =>  match parse_id(argument) {
                Ok(id)  =>  { todo_list.completed_task(id); }
                Err(error)  =>  println!("{}", error)
            }

            "delete"    =>  match parse_id(argument) {
                Ok(id)  =>  { todo_list.deleted_task(id); }
                Err(error)  =>  println!("{}", error),
            }

            "exit"      =>  {
                println!("Menyimpan data...");
                if let Err(error) = todo_list.save(filename) {
                    eprintln!("Gagal menyimpan task: {}", error);
                }

                println!("\nKeluar program..."); 
                break;
            }

            _           =>  println!("Command tidak diketahui! Gunakan 'add', 'list', 'complete', 'delete', atau 'exit'"),
        }
    }
}
