use std::fs::{self, DirEntry};

pub fn run() {
    let path = "./";

    print_dir_recursively(path, "");
}

fn print_dir_recursively(path: &str, line_prefix: &str) {
    let dir_contents = fs::read_dir(path).unwrap();
    let mut dirs: Vec<DirEntry> = vec![];
    let mut files: Vec<DirEntry> = vec![];

    for dir_entry_result in dir_contents {
        let dir_entry = dir_entry_result.unwrap();
        let file_type = dir_entry.file_type().unwrap();
        if file_type.is_dir() {
            dirs.push(dir_entry);
        } else if file_type.is_file() {
            files.push(dir_entry);
        }
    }

    let files_count = files.len();
    let dirs_count = dirs.len();
    for (index, file) in files.iter().enumerate() {
        if index == files_count - 1 && dirs_count == 0 {
            println!("{}└── {}", line_prefix, file.file_name().to_str().unwrap());
        } else {
            println!("{}├── {}", line_prefix, file.file_name().to_str().unwrap());
        }
    }

    for (index, dir) in dirs.iter().enumerate() {
        let file_name = dir.file_name();
        let file_name_str = file_name.to_str().unwrap();

        let new_line_prefix = if index == dirs_count - 1 {
            println!("{}└── {}", line_prefix, file_name_str);
            format!("{line_prefix}    ")
        } else {
            println!("{}├── {}", line_prefix, file_name_str);
            format!("{line_prefix}│   ")
        };

        print_dir_recursively(dir.path().to_str().unwrap(), &new_line_prefix)
    }
}
