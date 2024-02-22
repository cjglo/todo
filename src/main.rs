mod text_util;

use colored::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use term_size;

const D_FLAG: &'static str = "-d";
const SPECIAL_SEPARATOR: &'static str = "|||";

// I am aware of how cluttered and hard-coded this is, the goal was to make this asap for my use, not make it pretty
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let current_path = env::current_exe().unwrap();
    let path = current_path.parent().unwrap().to_str().unwrap().to_string() + "/internal_todos.txt";

    let file = File::open(&path).unwrap_or_else(|_| File::create(&path).unwrap());
    let mut contents = String::new();
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut contents).unwrap_or_else(|_| {
        println!("{}", "No File Found, starting new ToDo List".bright_yellow());
        0
    });

    if args.len() == 2 && args[1] == D_FLAG {
        println!("{}", "Error: passing only -d flag is not allowed".red());
    } else if args.len() > 1 && args[1] != D_FLAG {
        if contents.len() != 0 && contents.chars().nth(contents.len()-1).unwrap() != '\n' {
            contents.push_str("\n");
        }
        contents.push_str(&args[1]);
        contents.push_str(SPECIAL_SEPARATOR);
        contents.push_str(" ");
        if args.len() > 2 {
            contents.push_str(&args[2].to_uppercase());
        }
        let mut file = File::create(&path)?;
        file.write_all(contents.as_bytes())?;
    } else if args.len() > 1 && args[1] == D_FLAG {
        let index: usize = args[2].parse().unwrap();
        let mut todos: Vec<&str> = contents.split("\n").collect();
        if todos.len() < 1 {
            panic!("Error: LIST EMPTY");
        }
        let removed = todos.remove(index);
        let index = removed.find(SPECIAL_SEPARATOR).unwrap();
        print!("You Completed:");
        print!("\n{}", removed[0..index].bright_purple());
        println!("  ✔️");
        contents = todos.join("\n");
        let mut file = File::create(&path)?;
        file.write_all(contents.as_bytes())?;
    }
    let mut line = String::new();
    if let Some((width, _)) = term_size::dimensions() {
        // Create a string of the terminal width filled with '=' characters, remove the last few because can mess with new lines
        line = "=".repeat(width-2);
        println!("{}", line.bright_white());
    }
    for (line_number, raw_line) in contents.split("\n").enumerate() {
        let line = format!("{line_number}. ") + raw_line;
        let index = line.find(SPECIAL_SEPARATOR);
        if let Some(i) = index {
            let task_lines = text_util::handle_text_wrap(&line[0..i], 60);
            if line_number != 0 {
                println!("{}", "");
            }
            print!("{0: <60}", task_lines[0].bright_blue());
            if i+4 >= line.len() {  // if there is no due date, listed after the ||| string, starting at i
                println!("  |  {}", "<no due date>".green().dimmed())
            }
            else {
                println!("  |  DUE {0: <60}", &line[(i + 3)..].bright_yellow());
            }
            if task_lines.len() > 1 {
                task_lines
                    .iter()
                    .skip(1)
                    .for_each(|x| println!("{0: <50}", x.bright_blue()));
            }
        }
    }
    println!("{}", line.bright_white());
    Ok(())
}