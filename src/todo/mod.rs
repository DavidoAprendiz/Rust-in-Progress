use crate::{utils, views};
use std::env;
mod new_task;
mod remove_task;
mod show_task;
mod view_task;

/// Todo Manager
///
/// Start menu layout, begin loop, ask user input or exit program
pub fn main() {
    utils::create_folder(tasks_folder());
    views::start_menu_todo();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => new_task::add_task(),
            "2" => remove_task::remove_task(),
            "3" => show_task::show_task(),
            "4" => view_task::view_task(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_todo();
    }
    utils::clear_screen();
}

/// Get folder Tasks path
fn tasks_folder() -> String {
    if env::consts::OS.contains("windows") {
        "\\Project\\Tasks\\".to_string()
    } else {
        "/Project/Tasks/".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _tasks_folder() {
        assert_eq!(
            tasks_folder(),
            if env::consts::OS.contains("windows") {
                "\\Project\\Tasks\\".to_string()
            } else {
                "/Project/Tasks/".to_string()
            }
        );
    }
}
