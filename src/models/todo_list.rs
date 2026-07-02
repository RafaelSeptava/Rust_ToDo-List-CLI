use super::{ Status, Task, TodoList };

impl TodoList {
    pub fn add_task(&mut self, desc: String) {
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
}