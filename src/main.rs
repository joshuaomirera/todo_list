// src/main.rs

use std::io;

struct Task{
   text: String,
   complete: bool,
}

fn main() {

   let mut task_list: Vec<Task> = Vec::new();

   loop {

      let mut input = String::new();

      let _ = io::stdin()
         .read_line(&mut input);

      let trimmed_input = input.trim();

      let task = Task{
         text: String::from(trimmed_input),
         complete: false,
      };

      task_list.push(task);

      println!("Current list size: {}", task_list.len());
   }
}