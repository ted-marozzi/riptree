use std::{
    fs::{self},
    path::Path,
};

pub fn run(path: &Path) {
    if path.exists() {
        println!("{}", path.display());
    } else {
        println!("The path provided doesn't exist.\nPath: {}", path.display());
        return;
    }
    print_dir_recursively(path, "");
}

fn print_dir_recursively(path: &Path, line_prefix: &str) {
    let mut dir_entries = match fs::read_dir(path) {
        Err(_) => return,
        Ok(entries) => entries.peekable()
    };

    while let Some(entry_result) = dir_entries.next() {
        let Ok(entry) = entry_result else {
            continue;
        };
    
        let file_name_os_string = entry.file_name();
        let Some(file_name) = file_name_os_string.to_str() else {
            continue;
        };

        let new_line_prefix = if dir_entries.peek().is_none() {
            println!("{}└── {}", line_prefix, file_name);
            format!("{line_prefix}    ")
        } else {
            println!("{}├── {}", line_prefix, file_name);
            format!("{line_prefix}│   ")
        };

        match entry.file_type() {
            Err(_) => {
                continue;
            },
            Ok(file_type) => {
                if file_type.is_dir()  {
                    print_dir_recursively(&entry.path(), &new_line_prefix);
                }
            }
        }
    }
    
}
