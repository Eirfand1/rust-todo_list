use std::fs::{write, File, OpenOptions};
use std::io::{self, stdout, Read, Write};
use std::path::Path;
use std::task;

fn main() {
    let mut todo = Todo::new("todo.txt");

    println!("{:#?}", todo);

    loop {
        println!("\n=== Todo List App ===");
        println!("1. Tambah task");
        println!("2. Hapus task");
        println!("3. Lihat semua task");
        println!("4. Keluar");
        print!("Pilih menu (1-4): ");

        io::stdout().flush().unwrap();

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("gagal membaca input");

        match choice.trim() {
            "1" => {
                println!("Masukan todo: ");
                io::stdout().flush().unwrap();

                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("gagal membaca input");
                todo.add_task(task.trim());
            }
            "2" => {
                todo.list_task();
                print!("Masukkan nomor task yang akan dihapus: ");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Gagal membaca input");
                if let Ok(i) = index.trim().parse::<usize>() {
                    todo.remove_task(i - 1);
                    println!("Task berhasil dihapus!");
                } else {
                    println!("Nomor task tidak valid!");
                }
            }
            "3" => todo.list_task(),
            "4" => {
                println!("Gud Bai  Nerd");
                break;
            }
            // jika tidak ada yg sesuai pilihanya
            _ => println!("pilihan invalid"),
        }
    }
}

#[derive(Debug)]
struct Todo {
    tasks: Vec<String>,
    file_path: String,
}

impl Todo {
    fn new(file_path: &str) -> Self {
        let tasks = if Path::new(file_path).exists() {
            let mut file = File::open(file_path).expect("Gagal membuka file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Gagal membaca file");
            contents.lines().map(String::from).collect()
        } else {
            Vec::new()
        };

        Todo {
            tasks: tasks,
            file_path: String::from(file_path),
        }
    }

    fn add_task(&mut self, task: &str) {
        self.tasks.push(String::from(task));
        self.save_to_file()
    }

    fn save_to_file(&self) {
        let mut file: File = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.file_path)
            .expect("Gagal membuka file");

        for task in &self.tasks {
            writeln!(file, "{}", task).expect("gagal mebambahkan todo");
        }
    }

    fn list_task(&self) {
        if self.tasks.is_empty() {
            println!("Tidak ada todo")
        }

        println!(" == daftar task ==");
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            self.save_to_file();
        }
    }
}
