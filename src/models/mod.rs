// mod.rs
pub mod todo_list;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Todo, 
    Completed,
}

#[derive(Debug, Clone)]
pub struct Task {
    id: u32, 
    description: String, 
    status: Status,
}

#[derive(Debug, Default)]
pub struct TodoList {
    tasks: Vec<Task>,
}
