use crate::args_handler::ArgsHandler;
use crate::text_util;
use colored::{ColoredString, Colorize};
use std::fs::File;
use std::{fs, io};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ToDo {
    pub task: String,
    pub due_date: Option<String>
}

pub struct ToDoHandler {}

// TODO currently doesn't need to be struct, but may leave because fields may be required in future
impl ToDoHandler {
    const TODO_FILE_NAME: &'static str = "/todo_data.ron";

    const LINE_LENGTH_LIMIT: u16 = 60;
    const NO_DUE_DATE_TEXT: &'static str  = "<no due date>";
    const SEPERATOR_BETWEEN_TASK_AND_DATE: &'static str = "  |  ";
    const TITLE_BEFORE_DUE_DATE: &'static str = "DUE ";

    const SPECIAL_VIP_DATES: &'static [&'static str] = &["TODAY", "NOW"];

    // I am aware of how cluttered and hard-coded this is, the goal was to make this asap for my use, not make it pretty
    pub fn process(
        args_handler: ArgsHandler,
        args: &Vec<String>,
        current_path: PathBuf,
    ) -> io::Result<()> {
        let path =
            current_path.parent().unwrap().to_str().unwrap().to_string() + Self::TODO_FILE_NAME;

        if !fs::metadata(&path).is_ok() {
            fs::write(path.clone(), "[]".to_string())?;
            println!("{}", "Created new RON file for To-Do Data.".bright_yellow());
        }

        let file = File::open(path.clone())?;
        let reader = BufReader::new(file);

        // Read and deserialize the RON data
        let mut todos: Vec<ToDo> = ron::de::from_reader(reader).unwrap();

        if args_handler.help_flag {
            unimplemented!();
        } else if args_handler.is_invalid_or_blank {
            // does nothing, not sure if better way to handle this.  Needs to fall through
        } else if args_handler.delete_flag {
            let index: usize = args[2].parse().unwrap();
            if todos.len() < 1 {
                panic!("Error: LIST EMPTY");
            }
            let removed = todos.remove(index);
            print!("You Completed:");
            print!("\n{}", removed.task.bright_purple());
            println!("  ✔️");
            let mut file = File::create(path)?;
            file.write_all(ron::ser::to_string(&todos).unwrap().as_bytes())?;
        } else {
            // add items to to-do
            let mut to_add = ToDo { task: args[1].to_string(), due_date: None };
            if args.len() > 2 {
                to_add.due_date = Some(args[2].to_string().to_uppercase());
            }
            todos.push(to_add);
            todos.sort_by(|a,b| a.due_date.is_none().cmp(&b.due_date.is_none()));
            let mut file = File::create(path)?;
            file.write_all(ron::ser::to_string(&todos).unwrap().as_bytes())?;
        }

        Self::print_todos(todos);
        Ok(())
    }

    fn print_todos(todos: Vec<ToDo>) {
        let mut line = String::new();
        if let Some((width, _)) = term_size::dimensions() {
            // Create a string of the terminal width filled with '=' characters, remove the last few because can mess with new lines
            line = "=".repeat(width - 2);
            println!("{}", line.bright_white());
        }

        for (i, todo) in todos.iter().enumerate() {
            let header = format!("{i}. ");
            let header_length = header.len();
            let task = header + &todo.task;
            let task_lines = text_util::handle_text_wrap(&task, Self::LINE_LENGTH_LIMIT);
            if i != 0 {
                println!();
            }
            print!("{0}{1: <60}", task_lines[0].get(0..header_length).unwrap().bright_yellow(), task_lines[0].chars().skip(header_length).collect::<String>().bright_blue());
            if let Some(date) = &todo.due_date {
                let colored_date = if Self::SPECIAL_VIP_DATES.iter().any(|&x| x == *date) { date.bright_red() } else { date.bright_yellow() };
                println!("{}{}{}", Self::SEPERATOR_BETWEEN_TASK_AND_DATE, Self::TITLE_BEFORE_DUE_DATE, colored_date);
            }
            else {
                println!("{}{}", Self::SEPERATOR_BETWEEN_TASK_AND_DATE, Self::NO_DUE_DATE_TEXT.green().dimmed());
            }
            if task_lines.len() > 1 {
                task_lines
                    .iter()
                    .skip(1)
                    .for_each(|x| println!("{0: <50}", x.bright_blue()));
            }
        }

        println!("{}", line.bright_white());
    }
}
