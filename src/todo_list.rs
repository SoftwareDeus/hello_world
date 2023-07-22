// todo_list.rs

use std::collections::HashMap;

pub struct TodoEntry {
    pub id: i32,
    pub description: String,
    pub title: String,
    pub done: bool,
}

impl TodoEntry {
    pub fn new(id: i32, title: String, description: String) -> TodoEntry {
        TodoEntry {
            id,
            title,
            description,
            done: false,
        }
    }

    pub fn id_to_roman_numeral(id: i32) -> String {
        if id <= 0 {
            return "Invalid".to_string(); // Handle invalid input
        }

        let roman_numerals = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        let mut result = String::new();
        let mut remaining = id;

        for (numeral, value) in roman_numerals {
            while remaining >= value {
                result.push_str(numeral);
                remaining -= value;
            }
        }

        result
    }
}

pub struct TodoList {
    pub title: String,
    pub description: String,
    pub entries: HashMap<i32, TodoEntry>,
}

impl TodoList {
    pub fn new(title: String, description: String) -> TodoList {
        TodoList {
            entries: HashMap::new(),
            title: title,
            description: description,
        }
    }

    pub fn add_entry(&mut self, id: i32, title: String, description: String) {
        self.entries
            .insert(id, TodoEntry::new(id, title, description));
    }

    pub fn delete_entry(&mut self, id: i32) {
        self.entries.remove(&id);
    }

    pub fn mark_done(&mut self, id: i32) {
        if let Some(entry) = self.entries.get_mut(&id) {
            entry.done = true;
        }
    }

    pub fn print_all(&self) {
        for (id, entry) in &self.entries {
            println!(
                "{}. [{}] {}",
                entry.title,
                if entry.done { "x" } else { " " },
                entry.description
            );
        }
    }
    pub fn from_string(content: &str) -> Option<TodoList> {
        let mut lines = content.lines();
        if let Some(title) = lines.next() {
            if let Some(description) = lines.next() {
                let mut todo_list = TodoList {
                    title: title.to_string(),
                    description: description.to_string(),
                    entries: HashMap::new(),
                };

                for line in lines {
                    if let Some(entry) = TodoList::parse_todo_entry(line) {
                        todo_list.entries.insert(entry.id, entry);
                    } else {
                        eprintln!("Failed to parse TodoEntry: {}", line);
                    }
                }

                return Some(todo_list);
            }
        }

        None
    }
    fn parse_todo_entry(line: &str) -> Option<TodoEntry> {
        // Split the line by whitespace and collect the parts into a vector
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() >= 2 {
            // The first part should be the ID, and the rest will be the description
            if let Ok(id) = parts[0].parse::<i32>() {
                let description = parts[1..].join(" ");
                return Some(TodoEntry {
                    id,
                    description: description.to_string(),
                    title: "".to_string(), // You can set the title to an empty string for now
                    done: false,           // By default, the new entry is not done
                });
            }
        }

        None
    }

    // Method to convert a TodoList into a string representation
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (id, entry) in &self.entries {
            result.push_str(&format!("{} {}\n", entry.title, entry.description));
        }
        result
    }
}
