use iced::widget::{button, column, row, text, text_input, Column, container};
use iced::{Element, Center, Background, Border, Theme};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

// WORD-LEVEL AUTOCOMPLETE STRUCTURE
#[derive(Default, Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
    frequency: u32, // Track how often this word appears
}

#[derive(Clone)]
struct WordAutocomplete {
    root: TrieNode,
    common_verbs: Vec<&'static str>,
    common_objects: Vec<&'static str>,
}

impl WordAutocomplete {
    fn new() -> Self {
        let mut autocomplete = WordAutocomplete {
            root: TrieNode::default(),
            common_verbs: vec![
                "buy", "call", "clean", "complete", "create", "check", "contact",
                "delete", "discuss", "draft", "email", "finish", "fix", "get",
                "make", "meet", "order", "organize", "pay", "plan", "prepare",
                "review", "schedule", "send", "setup", "submit", "update", "visit", "write"
            ],
            common_objects: vec![
                "groceries", "milk", "bread", "eggs", "doctor", "dentist", "mom", "dad",
                "report", "presentation", "meeting", "appointment", "email", "invoice",
                "project", "homework", "laundry", "dishes", "room", "car", "bills",
                "documents", "files", "notes", "code", "website", "app", "design"
            ],
        };
        
        // Pre-populate with common words
        let verbs = autocomplete.common_verbs.clone();
        for verb in &verbs {
            autocomplete.insert(verb, 5); // Higher frequency for common verbs
        }
        
        let objects = autocomplete.common_objects.clone();
        for obj in &objects {
            autocomplete.insert(obj, 3);
        }
        
        autocomplete
    }

    fn insert(&mut self, word: &str, frequency_boost: u32) {
        let word = word.trim().to_lowercase();
        if word.is_empty() || word.len() < 2 {
            return;
        }
        
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert(TrieNode::default());
        }
        current.is_end_of_word = true;
        current.frequency += frequency_boost;
    }

    fn search(&self, prefix: &str, limit: usize) -> Vec<String> {
        let prefix = prefix.trim().to_lowercase();
        if prefix.is_empty() {
            return Vec::new();
        }

        let mut results = Vec::new();
        let mut current = &self.root;
        
        // Navigate to the prefix node
        for ch in prefix.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return results,
            }
        }
        
        // Collect words with their frequencies
        let mut words_with_freq = Vec::new();
        self.collect_words(current, prefix.clone(), &mut words_with_freq);
        
        // Sort by frequency (descending)
        words_with_freq.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Take top results
        results = words_with_freq.into_iter()
            .take(limit)
            .map(|(word, _)| word)
            .collect();
        
        results
    }

    fn collect_words(&self, node: &TrieNode, current_word: String, results: &mut Vec<(String, u32)>) {
        if node.is_end_of_word {
            results.push((current_word.clone(), node.frequency));
        }
        
        for (ch, child_node) in &node.children {
            let mut new_word = current_word.clone();
            new_word.push(*ch);
            self.collect_words(child_node, new_word, results);
        }
    }

    fn learn_from_task(&mut self, task: &str) {
        // Extract words from the task and add them to autocomplete
        let words: Vec<&str> = task.split_whitespace().collect();
        
        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if clean_word.len() >= 2 {
                self.insert(&clean_word, 1);
            }
        }
    }
}

impl Default for WordAutocomplete {
    fn default() -> Self {
        Self::new()
    }
}

// DATA MODEL
#[derive(Debug, Clone, Serialize, Deserialize)] 
struct Task {
    text: String,
    complete: bool,
}

struct TodoApp {
    tasks: Vec<Task>,
    input_value: String,
    autocomplete: WordAutocomplete,
    suggestions: Vec<String>,
    show_suggestions: bool,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    TaskCreated,
    DeleteTask(usize),
    ToggleTask(usize),
    SelectSuggestion(String),
}

impl TodoApp {
    fn save_to_disk(&self) {
        if let Ok(file) = File::create("tasks.json") {
            let _ = serde_json::to_writer_pretty(file, &self.tasks);
        }
    }

    fn get_current_word(&self) -> String {
        // Get the word currently being typed (last word in input)
        self.input_value
            .split_whitespace()
            .last()
            .unwrap_or("")
            .to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
                
                // Get the current word being typed
                let current_word = self.get_current_word();
                
                // Update suggestions based on current word
                if current_word.len() >= 2 {
                    self.suggestions = self.autocomplete.search(&current_word, 5);
                    self.show_suggestions = !self.suggestions.is_empty();
                } else {
                    self.suggestions.clear();
                    self.show_suggestions = false;
                }
            }
            Message::TaskCreated => {
                if !self.input_value.trim().is_empty() {
                    let task_text = self.input_value.clone();
                    
                    // Learn from this task
                    self.autocomplete.learn_from_task(&task_text);
                    
                    self.tasks.push(Task {
                        text: task_text,
                        complete: false,
                    });
                    
                    self.input_value.clear();
                    self.suggestions.clear();
                    self.show_suggestions = false;
                    self.save_to_disk();
                }
            }
            Message::DeleteTask(index) => {
                if index < self.tasks.len() {
                    self.tasks.remove(index);
                    self.save_to_disk();
                }
            }
            Message::ToggleTask(index) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.complete = !task.complete;
                    self.save_to_disk();
                }
            }
            Message::SelectSuggestion(suggestion) => {
                // Replace the current word with the suggestion
                let words: Vec<&str> = self.input_value.split_whitespace().collect();
                
                if words.is_empty() {
                    self.input_value = suggestion + " ";
                } else {
                    // Replace last word with suggestion
                    let mut new_input = words[..words.len()-1].join(" ");
                    if !new_input.is_empty() {
                        new_input.push(' ');
                    }
                    new_input.push_str(&suggestion);
                    new_input.push(' ');
                    self.input_value = new_input;
                }
                
                self.suggestions.clear();
                self.show_suggestions = false;
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

        // Autocomplete suggestions dropdown
        let suggestions_view: Element<Message> = if self.show_suggestions {
            let suggestions_list = self.suggestions
                .iter()
                .fold(Column::<Message>::new().spacing(2), |column, suggestion| {
                    let suggestion_button = button(
                        text(suggestion).size(14)
                    )
                    .on_press(Message::SelectSuggestion(suggestion.clone()))
                    .padding([6, 12])
                    .width(iced::Length::Fill)
                    .style(|theme: &Theme, status| {
                        let palette = theme.extended_palette();
                        button::Style {
                            background: Some(Background::Color(
                                match status {
                                    button::Status::Hovered => palette.primary.weak.color,
                                    _ => palette.background.base.color,
                                }
                            )),
                            text_color: match status {
                                button::Status::Hovered => palette.primary.strong.text,
                                _ => palette.background.base.text,
                            },
                            border: Border::default(),
                            ..Default::default()
                        }
                    });
                    
                    column.push(suggestion_button)
                });

            container(suggestions_list)
                .padding(4)
                .max_width(500)
                .style(|theme| {
                    let palette = theme.extended_palette();
                    container::Style {
                        background: Some(Background::Color(palette.background.base.color)),
                        border: Border {
                            width: 1.0,
                            color: palette.background.strong.color,
                            radius: 4.0.into(),
                        },
                        shadow: iced::Shadow {
                            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                            offset: iced::Vector::new(0.0, 2.0),
                            blur_radius: 8.0,
                        },
                        ..Default::default()
                    }
                })
                .into()
        } else {
            column![].into()
        };

        let tasks: Element<Message> = self
            .tasks
            .iter()
            .enumerate()
            .fold(Column::new().spacing(10), |column, (i, task)| {
                let checkbox = button(
                    text(if task.complete { "✓" } else { "" })
                        .size(16)
                        .width(20)
                        .center()
                )
                .on_press(Message::ToggleTask(i))
                .padding(8)
                .width(36)
                .style(move |theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    button::Style {
                        background: Some(Background::Color(
                            if task.complete {
                                palette.success.base.color
                            } else {
                                match status {
                                    button::Status::Hovered => palette.background.weak.color,
                                    _ => palette.background.base.color,
                                }
                            }
                        )),
                        text_color: if task.complete {
                            palette.success.base.text
                        } else {
                            palette.background.base.text
                        },
                        border: Border {
                            width: 2.0,
                            color: if task.complete {
                                palette.success.base.color
                            } else {
                                palette.background.strong.color
                            },
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                });

                let task_text = text(&task.text)
                    .size(18)
                    .style(move |theme: &Theme| {
                        let palette = theme.extended_palette();
                        if task.complete {
                            text::Style {
                                color: Some(palette.background.weak.text),
                            }
                        } else {
                            text::Style {
                                color: Some(palette.background.base.text),
                            }
                        }
                    });

                let delete_btn = button(text("×").size(20))
                    .on_press(Message::DeleteTask(i))
                    .padding(8)
                    .style(|theme: &Theme, status| {
                        let palette = theme.extended_palette();
                        button::Style {
                            background: Some(Background::Color(
                                match status {
                                    button::Status::Hovered => palette.danger.base.color,
                                    _ => iced::Color::TRANSPARENT,
                                }
                            )),
                            text_color: match status {
                                button::Status::Hovered => palette.danger.base.text,
                                _ => palette.background.weak.text,
                            },
                            border: Border::default(),
                            ..Default::default()
                        }
                    });
                
                let task_row = container(
                    row![checkbox, task_text, delete_btn]
                        .spacing(15)
                        .align_y(Center)
                        .padding(10)
                )
                .style(|theme: &Theme| {
                    let palette = theme.extended_palette();
                    container::Style {
                        background: Some(Background::Color(palette.background.weak.color)),
                        border: Border {
                            width: 1.0,
                            color: palette.background.strong.color,
                            radius: 6.0.into(),
                        },
                        ..Default::default()
                    }
                })
                .width(iced::Length::Fill);

                column.push(task_row)
            })
            .into();

        column![title, input, suggestions_view, tasks]
            .padding(20)
            .spacing(20)
            .max_width(500)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(
        || {
            // Load tasks from disk
            let tasks: Vec<Task> = if let Ok(file) = File::open("tasks.json") {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
            } else {
                Vec::new()
            };

            // Build autocomplete with common words + learn from existing tasks
            let mut autocomplete = WordAutocomplete::new();
            for task in &tasks {
                autocomplete.learn_from_task(&task.text);
            }

            (
                TodoApp {
                    tasks,
                    input_value: String::new(),
                    autocomplete,
                    suggestions: Vec::new(),
                    show_suggestions: false,
                },
                iced::Task::none()
            )
        },
        TodoApp::update,
        TodoApp::view
    ).run()
}