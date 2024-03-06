use std::fs;
use std::path::PathBuf;
use colored::Colorize;

const MAX_ITERATIONS: usize = 12;
const TODO_DIRECTORY: &'static str = "/todo-program-info";
const TODO_FILE_NAME: &'static str = "/todo_data.ron";

pub fn handle_text_wrap<'a>(line: &'a str, line_length: u16) -> Vec<&'a str> {
    let mut vec: Vec<&str> = vec![];

    let mut index = line_length as usize;
    let mut start = 0;
    while line.len() as i32 - index as i32 > 0 {
        let mut iterations = 0;
        while line.chars().nth(index).unwrap() != ' ' && index > 2 && index > start && iterations < MAX_ITERATIONS {
            index -= 1;
        }
        vec.push(&line[start..index]);
        start = index + ((line.len() as i32 - index as i32) != 0) as usize; // plus 1 will skip the whitespace
        index += 50;
    }
    vec.push(&line[start..]);

    vec
}

pub fn get_or_create_directory_file_path(current_path: PathBuf) -> std::io::Result<(String)> {
    let path =
        current_path.parent().unwrap().to_str().unwrap().to_string() + TODO_DIRECTORY;

    let directory_path = std::path::Path::new(&path);
    if !directory_path.is_dir() {
        fs::create_dir_all(directory_path)?;
    }
    let file_path = path + TODO_FILE_NAME;
    if !fs::metadata(&file_path).is_ok() {
        fs::write(file_path.clone(), "[]".to_string())?;
        println!("{}", "Created new RON file for To-Do Data.".bright_yellow());
    }
    Ok(file_path)
}