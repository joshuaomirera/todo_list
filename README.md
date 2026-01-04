# Rust Todo GUI

A simple, clean todo list application built with Rust and the Iced GUI framework.

![Rust](https://img.shields.io/badge/rust-2024-orange.svg)
![Iced](https://img.shields.io/badge/iced-0.14-blue.svg)

## Features

- ✅ Add new tasks
- ✅ Mark tasks as complete/incomplete
- ✅ Delete tasks
- ✅ Clean, minimal user interface
- ✅ Cross-platform (Windows, macOS, Linux)

## Prerequisites

- Rust 2024 edition or later
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd todo_list
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## Usage

### Adding Tasks
1. Type your task in the input field
2. Press `Enter` or click outside the field to add the task

### Completing Tasks
- Click the `[ ]` button next to a task to mark it as complete
- Click the `[X]` button to mark it as incomplete

### Deleting Tasks
- Click the `Delete` button next to any task to remove it

## Project Structure

```
todo_list/
├── Cargo.toml          # Project dependencies and metadata
├── Cargo.lock          # Dependency lock file
├── src/
│   └── main.rs         # Main application code
└── target/             # Build artifacts (generated)
```

## Dependencies

- [Iced](https://github.com/iced-rs/iced) v0.14 - A cross-platform GUI library for Rust

## Technical Details

The application uses the Elm architecture pattern:
- **Model** (`TodoApp`): Stores the application state (tasks and input value)
- **Update** (`update` method): Handles messages and updates the state
- **View** (`view` method): Renders the UI based on the current state

### Message Types
- `InputChanged(String)` - Updates the input field
- `TaskCreated` - Adds a new task
- `DeleteTask(usize)` - Removes a task by index
- `ToggleTask(usize)` - Toggles task completion status

## Future Enhancements

Potential features to add:
- [ ] Persistent storage (save/load tasks from file)
- [ ] Task editing
- [ ] Task priorities
- [ ] Filters (All/Active/Completed)
- [ ] Dark mode
- [ ] Due dates
- [ ] Categories/tags

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

Built with [Iced](https://github.com/iced-rs/iced), a cross-platform GUI library inspired by Elm.