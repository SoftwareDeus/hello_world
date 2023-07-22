// todo_connector.rs

use crate::file_writer::FileWriter;
use crate::todo_list::{TodoEntry, TodoList};
use crate::todo_planner::{TodoListElement, TodoPlanner};

pub struct TodoConnector {
    file_writer: FileWriter,
    planner: TodoPlanner, // Store TodoPlanner as a member
}

impl TodoConnector {
    pub fn new(base_directory: &str, planner: TodoPlanner) -> TodoConnector {
        let file_writer = FileWriter::new(base_directory);
        TodoConnector {
            file_writer,
            planner, // Store TodoPlanner in the TodoConnector
        }
    }

    // ... (other methods as before)

    pub fn load_todo_lists_from_files(&mut self) {
        let files = self.list_todo_list_files();
        for file_name in files {
            if let Some(content) = self.read_todo_list_file(&file_name) {
                if let Some(todo_list_element) = self.deserialize_todo_list_element(&content) {
                    self.planner.todo_lists.push(todo_list_element); // Access TodoPlanner via self.planner
                } else {
                    eprintln!("Failed to load TodoList from file: {}", file_name);
                }
            } else {
                eprintln!("Failed to read TodoList file: {}", file_name);
            }
        }
    }

    pub fn save_todo_lists_to_files(&self) {
        println!("{}", self.planner.todo_lists.len());
        for todo_list_element in &self.planner.todo_lists {
            let content = self.serialize_todo_list_element(todo_list_element);
            let file_name = format!("{}.txt", todo_list_element.todo_list.title);
            self.write_todo_list_file(&file_name, &content);
        }
    }

    pub fn list_todo_list_files(&self) -> Vec<String> {
        self.file_writer.list_todo_list_files()
    }

    pub fn read_todo_list_file(&self, file_name: &str) -> Option<String> {
        self.file_writer.read_todo_list_file(file_name)
    }

    pub fn write_todo_list_file(&self, file_name: &str, content: &str) {
        self.file_writer.write_todo_list_file(file_name, content);
    }

    // Move the remaining helper functions here

    fn serialize_todo_list_element(&self, todo_list_element: &TodoListElement) -> String {
        let mut content = String::new();
        content.push_str(&todo_list_element.todo_list.title);
        content.push('\n');
        content.push_str(&todo_list_element.todo_list.description);
        content.push('\n');
        for entry in &todo_list_element.todo_list.entries {
            content.push_str(&format!("{}\n", self.serialize_todo_entry(entry.1)));
            // Pass the reference to TodoEntry
        }
        content
    }

    // Helper function to deserialize the content of a TodoListElement from a string
    fn deserialize_todo_list_element(&self, content: &str) -> Option<TodoListElement> {
        let mut lines = content.lines();
        if let (Some(title), Some(description)) = (lines.next(), lines.next()) {
            let mut todo_list = TodoList::new(title.to_string(), description.to_string());
            for entry in lines {
                if let Some(todo_entry) = self.deserialize_todo_entry(entry) {
                    todo_list.add_entry(todo_entry.id, todo_entry.title, todo_entry.description);
                }
            }
            Some(TodoListElement { todo_list })
        } else {
            None
        }
    }

    fn serialize_todo_entry(&self, todo_entry: &TodoEntry) -> String {
        format!(
            "{} {} {} {}",
            todo_entry.id, todo_entry.title, todo_entry.description, todo_entry.done
        )
    }

    fn deserialize_todo_entry(&self, content: &str) -> Option<TodoEntry> {
        let parts: Vec<&str> = content.trim().splitn(4, ' ').collect();
        if parts.len() == 4 {
            if let Ok(id) = parts[0].parse::<i32>() {
                Some(TodoEntry {
                    id,
                    title: parts[1].to_string(),
                    description: parts[2].to_string(),
                    done: parts[3].parse().unwrap_or(false),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn start(&mut self) {
        self.load_todo_lists_from_files();
        self.planner.start_planner();
        self.save_todo_lists_to_files();
    }
}
