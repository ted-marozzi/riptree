use colored::*;
use std::{
    fs::{self},
    path::Path,
};

pub fn run(path: &Path) {
    if path.exists() {
        println!("{}", path.display().to_string().green().bold());
        let (dirs_count, files_count) = print_tree(path);
        println!(
            "\n{} directories, {} files",
            dirs_count.to_string().green().bold(),
            files_count.to_string().green().bold()
        );
    } else {
        println!("The path provided doesn't exist.\nPath: {}", path.display());
    }
}

fn print_tree(path: &Path) -> (u32, u32) {
    let (dirs_count, files_count) = print_dir_recursively(path, "");
    return (dirs_count + 1, files_count);
}

fn print_dir_recursively(path: &Path, line_prefix: &str) -> (u32, u32) {
    let mut dirs_count = 0;
    let mut files_count = 0;
    let mut dir_entries = match fs::read_dir(path) {
        Ok(entries) => entries.peekable(),
        Err(_) => return (dirs_count, files_count),
    };

    while let Some(entry_result) = dir_entries.next() {
        let Ok(entry) = entry_result else {
            continue;
        };

        let file_name_os_string = entry.file_name();
        let Some(file_name) = file_name_os_string.to_str() else {
            continue;
        };

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };

        let is_dir = file_type.is_dir();
        let is_file = file_type.is_file();

        let file_name_colored = if is_dir {
            file_name.blue().bold()
        } else {
            file_name.white()
        };

        let new_line_prefix = if dir_entries.peek().is_some() {
            println!("{}├── {}", line_prefix, file_name_colored);
            format!("{line_prefix}│   ")
        } else {
            println!("{}└── {}", line_prefix, file_name_colored);
            format!("{line_prefix}    ")
        };

        if is_dir {
            dirs_count += 1;
            let (child_dirs_count, child_files_count) =
                print_dir_recursively(&entry.path(), &new_line_prefix);
            dirs_count += child_dirs_count;
            files_count += child_files_count;
        } else if is_file {
            files_count += 1;
        }
    }
    return (dirs_count, files_count);
}
