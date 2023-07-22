use crate::console_utils::{get_input, get_input_i32};
use crate::todo_list::{TodoEntry, TodoList};

pub struct TodoListElement {
    pub todo_list: TodoList, // Change to store the TodoList directly
}

pub struct TodoPlanner {
    pub todo_lists: Vec<TodoListElement>,
}

const COMMAND_DESCRIPTIONS: [&'static str; 8] = [
    "1. Add a new TodoList",
    "2. Delete a TodoList",
    "3. Add a new TodoEntry",
    "4. Delete a TodoEntry",
    "5. Mark a TodoEntry as done",
    "6. Print all TodoEntries",
    "7. Print all TodoLists",
    "8. Exit",
];

// Messages
const WELCOME_MESSAGE: &'static str = "+ - Welcome to the Todo Planner - +";
const INVALID_ID_MESSAGE: &'static str = "+ - Invalid TodoList ID. - +";
const ENTER_TITLE_MESSAGE: &'static str = "+ - Please enter a title for the TodoList - +";
const ENTER_DESCRIPTION_MESSAGE: &'static str =
    "+ - Please enter a description for the TodoList - +";
const ENTER_TODO_ENTRY_DESCRIPTION_MESSAGE: &'static str =
    "+ - Please enter a description for the TodoEntry - +";
const ENTER_TODO_ENTRY_ID_MESSAGE: &'static str =
    "+ - Please enter the id of the TodoEntry you want to - +";
const PRINT_ALL_TODO_ENTRIES_MESSAGE: &'static str = "Printing all TodoEntries:";
const PRINT_ALL_TODO_LISTS_MESSAGE: &'static str = "Printing all TodoLists:";
const GOODBYE_MESSAGE: &'static str = "Goodbye";

// Create an enum for 8 different commands
enum Command {
    AddTodoList,
    DeleteTodoList,
    AddTodoEntry,
    DeleteTodoEntry,
    MarkTodoEntryAsDone,
    PrintAllTodoEntries,
    PrintAllTodoLists,
    Exit,
}

impl TodoPlanner {
    pub fn new() -> TodoPlanner {
        TodoPlanner {
            todo_lists: Vec::new(),
        }
    }
    fn find_todo_list_by_id(&self, id: i32) -> Option<&TodoList> {
        self.todo_lists
            .iter()
            .find(|tl| tl.todo_list.entries.len() as i32 == id)
            .map(|tl| &tl.todo_list)
    }

    fn delete_todo_list_with_id(&mut self, id: i32) {
        // Check if the provided ID corresponds to an existing TodoList
        if let Some(index) = self
            .todo_lists
            .iter()
            .position(|tl| tl.todo_list.entries.len() as i32 == id)
        {
            self.todo_lists.remove(index);
        } else {
            println!("{}", INVALID_ID_MESSAGE);
        }
    }
    fn add_todo_entry(&mut self, id: i32) {
        println!("{}", ENTER_TODO_ENTRY_DESCRIPTION_MESSAGE);
        let description = get_input();
        // Access the appropriate TodoList based on the given id
        for (index, todo_list_element) in self.todo_lists.iter_mut().enumerate() {
            if todo_list_element.todo_list.entries.len() as i32 == id {
                let id_to_use = todo_list_element.todo_list.entries.len() as i32;
                todo_list_element.todo_list.add_entry(
                    id_to_use,
                    TodoEntry::id_to_roman_numeral(id_to_use + 1),
                    description,
                );
                return;
            }
        }
        println!("{}", INVALID_ID_MESSAGE);
    }

    fn print_all_todo_entries(&self, id: i32) {
        if let Some(todo_list) = self.find_todo_list_by_id(id) {
            println!("{}", PRINT_ALL_TODO_ENTRIES_MESSAGE);
            todo_list.print_all();
        } else {
            println!("{}", INVALID_ID_MESSAGE);
        }
    }
    fn add_todo_list(&mut self) {
        println!("{}", ENTER_TITLE_MESSAGE);
        let title = get_input();
        println!("{}", ENTER_DESCRIPTION_MESSAGE);
        let description = get_input();
        self.add_new_todo_list(title, description);
    }

    fn delete_todo_entry(&mut self, id: i32) {
        self.todo_lists[id as usize].todo_list.delete_entry(id);
    }

    fn mark_todo_entry_as_done(&mut self, id: i32) {
        self.todo_lists[id as usize].todo_list.mark_done(id);
    }

    fn print_all_todo_lists(&self) {
        println!("{}", PRINT_ALL_TODO_LISTS_MESSAGE);
        self.print_all();
    }

    // Function to handle each command
    fn handle_command(&mut self, id: i32, command: Command) {
        match command {
            Command::AddTodoList => self.add_todo_list(),
            Command::DeleteTodoList => self.delete_todo_list_with_id(id),
            Command::AddTodoEntry => self.add_todo_entry(id),
            Command::DeleteTodoEntry => self.delete_todo_entry(id),
            Command::MarkTodoEntryAsDone => self.mark_todo_entry_as_done(id),
            Command::PrintAllTodoEntries => self.print_all_todo_entries(id),
            Command::PrintAllTodoLists => self.print_all_todo_lists(),
            Command::Exit => {
                println!("{}", GOODBYE_MESSAGE);
                return;
            }
        }
        self.start_planner();
    }

    // Function to convert input to a valid command number
    fn parse_command_input(input: i32) -> Option<Command> {
        if input >= 1 && input <= 8 {
            Some(match input {
                1 => Command::AddTodoList,
                2 => Command::DeleteTodoList,
                3 => Command::AddTodoEntry,
                4 => Command::DeleteTodoEntry,
                5 => Command::MarkTodoEntryAsDone,
                6 => Command::PrintAllTodoEntries,
                7 => Command::PrintAllTodoLists,
                8 => Command::Exit,
                _ => unreachable!(), // We've already checked the range, this should not happen
            })
        } else {
            None
        }
    }

    // Start function for the planner
    pub fn start_planner(&mut self) {
        println!("{}", WELCOME_MESSAGE);

        // Print the command descriptions
        for (i, description) in COMMAND_DESCRIPTIONS.iter().enumerate() {
            // Skip the commands that require TodoLists if there are none
            if i >= 1 && i <= 6 && self.todo_lists.is_empty() {
                continue;
            }
            println!("{}: {}", i + 1, description);
        }

        let mut id: i32 = -1;
        // If input is not 1, 7, 6, or 8, get another input for id
        let input = get_input_i32("");

        if input != 1 && input != 7 && input != 8 {
            // Skip getting the ID if there are no TodoLists
            if self.todo_lists.is_empty() {
                println!("+ - Invalid Command. - +");
                self.start_planner();
                return;
            }
            println!("{}", ENTER_TODO_ENTRY_ID_MESSAGE);
            id = get_input_i32("");
        }

        // Convert the input into a valid command number
        match TodoPlanner::parse_command_input(input) {
            Some(command) => self.handle_command(id, command),
            None => {
                println!("+ - Please enter a valid command - +");
                self.start_planner();
            }
        }
    }

    pub fn add_new_todo_list(&mut self, title: String, description: String) {
        let new_todo_list = TodoListElement {
            todo_list: TodoList::new(title, description),
        };
        self.todo_lists.push(new_todo_list); // Push the new_todo_list directly into the vector
        print!("hahahah {}", self.todo_lists.len());
    }

    fn delete_todo_list(&mut self, id: i32) {
        let index = self
            .todo_lists
            .iter()
            .position(|todo_list| todo_list.todo_list.entries.len() as i32 == id);

        if let Some(index) = index {
            self.todo_lists.remove(index);
        }
    }

    fn print_all(&self) {
        for todo_list in &self.todo_lists {
            println!(
                "{}: {}",
                todo_list.todo_list.title, todo_list.todo_list.description
            );
            todo_list.todo_list.print_all();
        }
    }
}
