use crate::args_handler::ArgsHandler;
use crate::text_util;
use crate::todo_item;
use colored::Colorize;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::path::PathBuf;



pub struct ToDoHandler {}

// TODO currently doesn't need to be struct, but may leave because fields may be required in future
impl ToDoHandler {
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
        let mut todos: Vec<todo_item::ToDo> = ron::de::from_reader(reader).unwrap();

        // early exit options
        if args_handler.help_flag {
            Self::print_help_message();
            return Ok(())
        }
        if args_handler.is_invalid {
            return Ok(())
        }
        if args_handler.is_blank {
            Self::print_todos(todos);
            return Ok(())
        }

        if args_handler.delete_flag_and_index.is_some() {
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
        } else if args_handler.change_flag_and_index.is_some() {
            let to_edit = &mut todos[args[args_handler.change_flag_and_index.unwrap() + 1]
                .parse::<usize>()
                .unwrap()];
            // 1 string arg means it edits due date, two means it replaces both
            let first_arg = args.get(args_handler.change_flag_and_index.unwrap() + 2);
            let second_arg = args.get(args_handler.change_flag_and_index.unwrap() + 3);

            // if second arg has value, then first is new task title and second is new due date
            // NOTE: Only way to change task title and delete due date is to do it in 2 separate commands
            if let Some(due_date) = second_arg {
                to_edit.task = first_arg.unwrap().clone();
                if due_date.len() != 0 {
                    to_edit.due_date = Some(due_date.clone().to_uppercase());
                }
            }
            else {
                to_edit.due_date = if let Some(due_date) = first_arg { Some(due_date.clone().to_uppercase()) } else { None }
            }
        } else {
            // add items to to-do
            let mut to_add = todo_item::ToDo {
                task: args[1].to_string(),
                due_date: None,
                char_marker: None,
            };
            if args.len() > 2 {
                to_add.due_date = Some(args[2].to_string().to_uppercase());
            }
            todos.push(to_add);
        }
        todos.sort_by(Self::todo_compare);
        let mut file = File::create(file_path)?;
        file.write_all(ron::ser::to_string(&todos).unwrap().as_bytes())?;
        Self::print_todos(todos);

        Ok(())
    }

    // TODO can def simplify and clean-up this
    fn print_todos(todos: Vec<todo_item::ToDo>) {
        if let Some((width, _)) = term_size::dimensions() {
            // Create a string of the terminal width filled with '=' characters, remove the last few because can mess with new lines
            println!("{}", "-".repeat(width - 2).bright_white());
        }
        for (i, todo) in todos.iter().enumerate() {
            let header = i.to_string();
            print!("{0:<3}", i);
            print!("{}", todo);
        }
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
  todo -c <index> <task> <date>  Changes todo item's task and due date
  todo -c <index> <task> <''>    Changes todo item's task and copies old due date
  todo -m <index> <char>         Dims a todo item and marks with the given char, char defaults to ⏳

Options:
  --help                         Displays this help message
  -d                             Delete the completed todo item
  -c                             Changes the description or due date of an item
  -m                             Marks a task to be dimmed out


Examples:
  todo "Buy groceries"           Add a todo item without a due date
  todo "Finish report" "friday"  Add a todo item with a due date
  todo "Talk to boss" "today"    Add a todo item with a special due date that will be highlighted
  todo -d 0                      Remove a todo item in the list with index 0
  todo -m 1 "?"                  Dims the todo item at index 1 and marks it with "?"
  todo -c 0 "monday"             Changes the due date of the item at index 0 to 'monday'
  todo -c 0 "shop" "tuesday"     Changes the task at 0 to "shop" with new due date 'tuesday'
  todo -c 0 "buy tickets" ""     Changes the task at 0 to "buy tickets" keeps old due date"#;
        println!("{}", message);
    }

    fn todo_compare(a: &todo_item::ToDo, b: &todo_item::ToDo) -> Ordering {
        match (a.char_marker.is_none(), b.char_marker.is_none()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => match (a.due_date.is_none(), b.due_date.is_none()) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                _ => {
                    if todo_item::HIGHLIGHTED_DUE_DATES.contains(&a.due_date.clone().unwrap_or("".to_string())) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }
        }
    }
}
