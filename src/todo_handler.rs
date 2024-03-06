use crate::args_handler::ArgsHandler;
use crate::text_util;
use colored::Colorize;
use phf::{phf_set, Set};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct ToDo {
    pub task: String,
    pub due_date: Option<String>,
    pub char_marker: Option<char>,
}

static HIGHLIGHTED_DUE_DATES: Set<&'static str> = phf_set! {
    "TODAY",
    "NOW",
};

pub struct ToDoHandler {}

// TODO currently doesn't need to be struct, but may leave because fields may be required in future
impl ToDoHandler {
    const LINE_LENGTH_LIMIT: u16 = 60;
    const NO_DUE_DATE_TEXT: &'static str = "<no due date>";
    const PADDING_BETWEEN_TASK_AND_DATE: &'static str = "  |  ";
    const TITLE_BEFORE_DUE_DATE: &'static str = "DUE ";
    const DEFAULT_MARKET_CHAR: char = '⌛';

    // I am aware of how cluttered and hard-coded this is, the goal was to make this asap for my use, not make it pretty
    pub fn process(
        args_handler: ArgsHandler,
        args: &Vec<String>,
        current_path: PathBuf,
    ) -> io::Result<()> {
        let file_path = text_util::get_or_create_directory_file_path(current_path)?;

        let file = File::open(file_path.clone())?;
        let reader = BufReader::new(file);
        let mut todos: Vec<ToDo> = ron::de::from_reader(reader).unwrap();

        if args_handler.help_flag {
            Self::print_help_message();
        } else if args_handler.is_invalid_or_blank {
            // does nothing, not sure if better way to handle this.  Needs to fall through
        } else if args_handler.delete_flag_and_index.is_some() {
            let index: usize = args[args_handler.delete_flag_and_index.unwrap() + 1]
                .parse()
                .unwrap();
            if todos.len() < 1 {
                panic!("Error: LIST EMPTY");
            }
            let removed = todos.remove(index);
            print!("You Completed:");
            print!("\n{}", removed.task.bright_purple());
            println!("  ✔️");
            let mut file = File::create(file_path)?;
            file.write_all(ron::ser::to_string(&todos).unwrap().as_bytes())?;
        } else if args_handler.marker_flag_and_index.is_some() {
            let to_edit = &mut todos[args[args_handler.marker_flag_and_index.unwrap() + 1]
                .parse::<usize>()
                .unwrap()];
            if to_edit.char_marker.is_some() {
                to_edit.char_marker = None;
            } else {
                to_edit.char_marker =
                    match args.get(args_handler.marker_flag_and_index.unwrap() + 2) {
                        Some(arg) => Some(arg.chars().nth(0).unwrap()),
                        None => Some(Self::DEFAULT_MARKET_CHAR),
                    }
            }
            let mut file = File::create(file_path)?;
            file.write_all(ron::ser::to_string(&todos).unwrap().as_bytes())?;
        } else if args_handler.change_flag_and_index.is_some() {
            let to_edit = &mut todos[args[args_handler.change_flag_and_index.unwrap() + 1]
                .parse::<usize>()
                .unwrap()];
            let new_due_date = args.get(args_handler.change_flag_and_index.unwrap() + 2);
            to_edit.due_date = if new_due_date.is_some() {
                Some(new_due_date.unwrap().clone().to_uppercase())
            } else {
                None
            };
            todos.sort_by(Self::todo_compare);
            let mut file = File::create(file_path)?;
            file.write_all(ron::ser::to_string(&todos).unwrap().as_bytes())?;
        } else {
            // add items to to-do
            let mut to_add = ToDo {
                task: args[1].to_string(),
                due_date: None,
                char_marker: None,
            };
            if args.len() > 2 {
                to_add.due_date = Some(args[2].to_string().to_uppercase());
            }
            todos.push(to_add);
            todos.sort_by(Self::todo_compare);
            let mut file = File::create(file_path)?;
            file.write_all(ron::ser::to_string(&todos).unwrap().as_bytes())?;
        }

        if !args_handler.help_flag {
            Self::print_todos(todos);
        }
        Ok(())
    }

    // TODO can def simplify and clean-up this
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
            // This will check if to-do should be marked as dim, then color the number of the line, and break it it up to format it correctly
            if todo.char_marker.is_some() {
                print!("{0: <63}", task_lines[0].bright_black());
                if let Some(date) = &todo.due_date {
                    println!(
                        "{}{}",
                        (Self::PADDING_BETWEEN_TASK_AND_DATE.to_string()
                            + Self::TITLE_BEFORE_DUE_DATE
                            + date
                            + Self::PADDING_BETWEEN_TASK_AND_DATE)
                            .bright_black(),
                        todo.char_marker.unwrap()
                    );
                } else {
                    println!(
                        "{}{}",
                        (Self::PADDING_BETWEEN_TASK_AND_DATE.to_string()
                            + Self::NO_DUE_DATE_TEXT
                            + Self::PADDING_BETWEEN_TASK_AND_DATE)
                            .bright_black(),
                        todo.char_marker.unwrap()
                    );
                }
            } else {
                print!(
                    "{0}{1: <60}",
                    task_lines[0].get(0..header_length).unwrap().bright_yellow(),
                    task_lines[0]
                        .chars()
                        .skip(header_length)
                        .collect::<String>()
                        .bright_blue()
                );
                if let Some(date) = &todo.due_date {
                    let colored_date = if HIGHLIGHTED_DUE_DATES.contains(date) {
                        date.bright_red()
                    } else {
                        date.bright_yellow()
                    };
                    println!(
                        "{}{}{}",
                        Self::PADDING_BETWEEN_TASK_AND_DATE,
                        Self::TITLE_BEFORE_DUE_DATE,
                        colored_date
                    );
                } else {
                    println!(
                        "{}{}",
                        Self::PADDING_BETWEEN_TASK_AND_DATE,
                        Self::NO_DUE_DATE_TEXT.green().dimmed()
                    );
                }
            }

            Self::print_remaining_task_lines(task_lines, todo.char_marker.is_some());
        }

        println!("{}", line.bright_white());
    }

    fn print_help_message() {
        let message = r#"
        todo - Manage Your To-Dos in the CLI

Usage:
  todo                           Display all todo items and indices
  todo <item>                    Add a todo item with no due date
  todo <item> <date>             Add a todo item with a due date
  todo <item> <'today'|'now'>    Add a todo item with special due date that highlights and puts at top of list
  todo -d <index>                Remove a todo item by index on the todo list
  todo -c <index> <date>         Changes todo item's due date
  todo -m <index> <char>         Dims a todo item and marks with the given char, char defaults to ⏳

Options:
  --help                         Displays this help message
  -d                             Delete the completed todo item
  -c                             Changes the due date of an item
  -m                             Marks a task to be dimmed out


Examples:
  todo "Buy groceries"           Add a todo item without a due date
  todo "Finish report" "friday"  Add a todo item with a due date
  todo "Talk to boss" "today"    Add a todo item with a special due date that will be highlighted
  todo -d 0                      Remove a todo item in the list with index 0
  todo -m 1 "?"                  Dims the todo item at index 1 and marks it with "?"
  todo -c 0 "monday"             Changes the due date of the item at index 0 to 'monday'"#;
        println!("{}", message);
    }

    fn todo_compare(a: &ToDo, b: &ToDo) -> Ordering {
        match (a.due_date.is_none(), b.due_date.is_none()) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => {
                if HIGHLIGHTED_DUE_DATES.contains(&a.due_date.clone().unwrap_or("".to_string())) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }

    fn print_remaining_task_lines(task_lines: Vec<&str>, is_marked: bool) {
        let color_lambda = |x: &str| {
            if is_marked {
                x.bright_black()
            } else {
                x.bright_blue()
            }
        };
        if task_lines.len() > 1 {
            task_lines
                .iter()
                .skip(1)
                .for_each(|x| println!("{0: <50}", color_lambda(x)));
        }
    }
}
