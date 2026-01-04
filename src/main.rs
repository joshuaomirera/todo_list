use iced::widget::{button, column, row, text, text_input, Column};
use iced::{Element, Center};

#[derive(Debug, Clone)]
struct Task {
    text: String,
    complete: bool,
}

struct TodoApp {
    tasks: Vec<Task>,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    TaskCreated,
    DeleteTask(usize),
    ToggleTask(usize),
}

impl TodoApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::TaskCreated => {
                if !self.input_value.trim().is_empty() {
                    self.tasks.push(Task {
                        text: self.input_value.clone(),
                        complete: false,
                    });
                    self.input_value.clear();
                }
            }
            Message::DeleteTask(index) => {
                if index < self.tasks.len() {
                    self.tasks.remove(index);
                }
            }
            Message::ToggleTask(index) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.complete = !task.complete;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Tasks").size(40);

        let input = text_input("What needs to be done?", &self.input_value)
            .on_input(Message::InputChanged)
            .on_submit(Message::TaskCreated)
            .padding(10)
            .size(20);

        let tasks: Element<Message> = self
            .tasks
            .iter()
            .enumerate()
            .fold(Column::new().spacing(10), |column, (i, task)| {
                let status_icon = if task.complete { "[X]" } else { "[ ]" };
                
                let task_row = row![
                    button(status_icon).on_press(Message::ToggleTask(i)),
                    text(&task.text).size(20),
                    button("Delete").on_press(Message::DeleteTask(i))
                ]
                .spacing(20)
                .align_y(Center);

                column.push(task_row)
            })
            .into();

        column![title, input, tasks]
            .padding(20)
            .spacing(20)
            .max_width(500)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(
        || {
            (
                TodoApp {
                    tasks: Vec::new(),
                    input_value: String::new(),
                },
                iced::Task::none()
            )
        },
        TodoApp::update,
        TodoApp::view
    ).run()
}