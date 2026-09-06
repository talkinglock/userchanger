use walkdir::WalkDir;
use std::path::{PathBuf, Path};
use text_io::read;
use std::fs;
use std::io::Error;

const TARGET_FILE: &'static str = "steam_emu.ini";
pub fn find_file() -> Option<PathBuf> {
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.file_name() == TARGET_FILE {
            // this is the file
            return Some(entry.path().to_path_buf())
        }
    }
    None
}
fn main() {
    let Some(file) = find_file() else {
        println!("Cannot find the proper file. Make sure *this* file is inside the game folder with its exe.");
        return;
    };
    println!("File found");
    let data = match fs::read_to_string(file.clone()) {
        Ok(data) => data,
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Invalid permissions to modify file. Run as superuser (or administator, if on windows).");
                }
                _ => {
                    panic!("File system error: {}", err.kind().to_string());
                }
            }
        }
    };
    println!("What username do you want: ");
    let desired_name: String = read!();
    println!("What account id do you want: ");
    let desired_userid: usize = read!();

    println!("Assembling parse list");
    let characters = data.chars();
    let mut peekable = characters.peekable();
    let mut lines = Vec::new();
    let mut buffer = String::new();
    while let Some(char) = peekable.next() {
        match char {
            '\n' => {
                lines.push(buffer);
                buffer = String::new();
            }
            _ => {
                buffer.push(char);
            }
        }
    }
    println!("Clearing existing entries");
    let mut new_lines = Vec::new();
    for line in lines.into_iter() {
        if line.contains("AccountId") || line.contains("UserName") {
            continue;
        }
        new_lines.push(line);
    }
    println!("Adding new entries");
    let mut final_lines = Vec::new();
    for line in new_lines {
        final_lines.push(line.clone());
        if line.contains("[Settings]") {
            println!("Settings block identified");
            final_lines.push(format!("AccountId={}", desired_userid));
            println!("Added account id block");
            final_lines.push(format!("UserName={}", desired_name));
            println!("Added account name block");
        }
    }
    println!("Finished modifications");
    let mut final_str = String::new();
    for line in final_lines {
        final_str.push_str(&line);
        final_str.push('\n');
    }
    println!("Stringified parsed data");
    println!("Attempting file write");
    let write_result = fs::write(file, final_str);
    let data = match write_result {
        Ok(data) => data,
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Invalid permissions to modify file. Run as superuser (or administator, if on windows).");
                }
                _ => {
                    panic!("File system error: {}", err.kind().to_string());
                }
            }
        }
    };
    println!("File write successful.");
    println!("Matthew complete. Have a very safe day!");
}
