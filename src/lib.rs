use std::{
    fs::{self},
    path::Path,
};

pub fn run(path: &Path) {
    println!("{}", path.display());
    print_dir_recursively(path, "");
}

fn print_dir_recursively(path: &Path, line_prefix: &str) {
    let mut dir_entries = fs::read_dir(path).unwrap().map(|d| d.unwrap()).peekable();
    while let Some(entry) = dir_entries.next() {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap();

        let new_line_prefix = if dir_entries.peek().is_none() {
            println!("{}└── {}", line_prefix, file_name_str);
            format!("{line_prefix}    ")
        } else {
            println!("{}├── {}", line_prefix, file_name_str);
            format!("{line_prefix}│   ")
        };
        if entry.file_type().unwrap().is_dir() {
            print_dir_recursively(&entry.path(), &new_line_prefix)
        }
    }
}
