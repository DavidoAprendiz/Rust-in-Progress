use crate::views;
use chrono::Local;
use std::{env, io, process};

// Color configuration
pub const CLOSE: &str = "\x1b[0m";
pub const BLUE: &str = "\x1b[0m\x1b[34m";
pub const CYAN_UNDERLINE: &str = "\x1b[0m\x1b[36;4m";
pub const CYAN_UNDERLINE_BOLD: &str = "\x1b[0m\x1b[36;1;4m";
pub const ERRO: &str = "\x1b[0m\x1b[31;3m";

/// Get user input for 'Menus'.
pub fn get_user_input() -> String {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => user_input.trim().to_string(),
        Err(_) => {
            user_input = String::from("");
            user_input
        }
    }
}

/// Clean the screen via clear.
pub fn clear_screen() {
    if env::consts::OS.contains("windows") {
        match process::Command::new("cmd").args(["/c", "cls"]).status() {
            Ok(_) => (),
            Err(e) => println!("{ERRO}Failed to clear the screen.\n{e}{CLOSE}\n"),
        }
    } else {
        match process::Command::new("clear").status() {
            Ok(_) => (),
            Err(e) => println!("{ERRO}Failed to clear the screen.\n{e}{CLOSE}\n"),
        }
    }
}

/// Boolean to call 'mainloop break' to exit the program.
pub fn exit_program(input: &str) -> bool {
    if input.trim().to_lowercase().starts_with('q') || input.trim().to_lowercase().starts_with('e')
    {
        views::start_menus("Exiting...");
        return true;
    }
    false
}

/// Get Operating System paths
pub fn get_os() -> String {
    if env::consts::OS.contains("windows") {
        "\\".to_string()
    } else {
        "/".to_string()
    }
}

/// Get current directory plus folder (as String)
pub fn get_file_path(user_file: String) -> String {
    format!(
        "{}{}{}",
        std::env::current_dir()
            .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n")
            .display(),
        get_os(),
        user_file
    )
}

/// Check if project folder exists in root folder.
pub fn create_folder(path: String) {
    match std::fs::create_dir_all(format!(
        "{}{}{}",
        std::env::current_dir()
            .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n")
            .display(),
        path,
        get_os()
    )) {
        Ok(_) => {}
        Err(e) => println!("{ERRO}Failed to create folder '{path}'.\n{e}{CLOSE}\n"),
    }
}

// Get all files from a specific folder
pub fn get_files_from_folder(user_path: String) {
    let path = get_file_path(user_path);
    match std::fs::read_dir(path) {
        Ok(dirs) => {
            for files in dirs {
                match files {
                    Ok(file) => {
                        if file.path().is_file() {
                            println!("{:#?}", file.file_name());
                        }
                    }
                    Err(e) => {
                        println!(
                            "{ERRO}Failed to read the name of the files in current folder.\nError: {e}{CLOSE}\n"
                        )
                    }
                }
            }
        }
        Err(e) => println!("{ERRO}Failed to read the current folder.\nError: {e}{CLOSE}\n"),
    }
    println!();
}

/// Calculate the Hours and Minutes from 'seconds_from_midnight'. Return a String and is used in 'save_task(), as example.'.
pub fn get_current_time() -> String {
    Local::now().format("%d-%m-%Y-%H_%M_%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _exit_program() {
        let input = "quit12345qwerty";
        let input2 = "exit12345qwerty";
        assert!(exit_program(input));
        assert!(exit_program(input2));
    }

    #[test]
    fn _get_os() {
        if env::consts::OS.contains("windows") {
            assert!(get_os().contains('\\'));
        } else {
            assert!(get_os().contains('/'));
        }
    }

    #[test]
    fn _get_current_time() {
        let current_time = get_current_time();
        assert!(!current_time.is_empty());
    }
}
