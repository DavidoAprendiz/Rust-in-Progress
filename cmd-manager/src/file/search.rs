use crate::utils;
use std::fs;

pub fn main() {
    utils::clear_screen();
    start_menu();
    get_pattern();
}

/// Run the menu layout.
fn start_menu() {
    println!("###############################################");
    println!("#                                             #");
    println!("#               Search in File!               #");
    println!("#                                             #");
    println!("###############################################");
}

fn get_pattern() {
    println!("\n- Available files:\n");
    get_tasks_from_folder();
    println!("\n- Please insert the pattern you want to search:");
    get_file(utils::get_user_input())
}

fn get_file(user_pattern: String) {
    println!("\n- Please insert the file name: (without quotes)");
    let user_input = utils::get_user_input();
    get_results(user_pattern, user_input)
}

fn get_results(user_pattern: String, user_file: String) {
    let path = get_file_path(user_file);
    println!("\n###############################################");
    println!("- The results for file -> {}\n", &path);

    let mut count_total: u64 = 0;
    let mut count_line: u64 = 0;
    match fs::read_to_string(path) {
        Ok(lines) => {
            for line in lines.lines() {
                count_line += 1;
                if line.trim().contains(user_pattern.trim()) {
                    count_total += 1;
                    println!("  - Found in line {count_line}:    {}\n", line.trim());
                }
            }
            println!("- {count_total} occurrences in total.");
            println!("###############################################\n");
        }
        Err(e) => println!("Failed to open file.\n{e}"),
    }
    utils::get_user_input();
    utils::clear_screen()
}

fn get_tasks_from_folder() {
    let path = get_file_path("".to_string());
    match fs::read_dir(path) {
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
                            "Failed to read the name of the files in current folder.\nError: {e}\n"
                        )
                    }
                }
            }
        }
        Err(e) => println!("Failed to read the current folder.\nError: {e}\n"),
    }
    println!();
}

fn get_file_path(user_file: String) -> String {
    let path = format!(
        "{}\\{user_file}",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
    );
    path
}
