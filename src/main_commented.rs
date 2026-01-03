// src/main.rs

// Import the standard I/O module for reading input from stdin
use std::io;

// Define a Task struct to represent a single to-do item
struct Task {
    // The task description entered by the user
    text: String,

    // Whether the task is completed or not
    complete: bool,
}

fn main() {
    // Create a mutable vector to store Task structs
    let mut task_list: Vec<Task> = Vec::new();

    // Start an infinite loop to continuously accept user input
    loop {
        // Create a new String buffer for user input
        let mut input = String::new();

        // Read a line of input from standard input into `input`
        // The result is ignored here using `_`
        let _ = io::stdin()
            .read_line(&mut input);

        // Trim leading/trailing whitespace (including the newline)
        // This returns a &str (string slice), not a new String
        let trimmed_input = input.trim();

        // Create a new Task instance using the trimmed input
        let task = Task {
            // Convert &str into an owned String
            text: String::from(trimmed_input),

            // New tasks start as incomplete
            complete: false,
        };

        // Add the new task to the task list
        task_list.push(task);

        // Print the current number of tasks in the list
        println!("Current list size: {}", task_list.len());
    }
}
