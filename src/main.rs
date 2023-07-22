mod console_utils;
mod file_writer;
mod todo_connector;
mod todo_list;
mod todo_planner;

use todo_connector::TodoConnector;
use todo_planner::TodoPlanner;

fn main() {
    let base_directory = "./todo_files";
    let mut planner = TodoPlanner::new();
    let mut todo_connector = TodoConnector::new(base_directory, planner);

    todo_connector.start();
}
