mod text_util;
mod args_handler;
mod todo_handler;

use colored::*;
use std::env;
use std::io;
use std::io::{BufRead, Read, Write};
use term_size;

use args_handler::ArgsHandler;
use crate::todo_handler::ToDoHandler;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let current_path = env::current_exe().unwrap();
    let args_handler = ArgsHandler::new(args.len(), &args);
    ToDoHandler::process(args_handler, &args, current_path)?;
    Ok(())
}