use colored::Colorize;

pub struct ArgsHandler {
    pub help_flag: bool, // displays help mesage
    pub delete_flag_and_index: Option<usize>, // deleting task
    pub marker_flag_and_index: Option<usize>, // adds special mark to task
    pub change_flag_and_index: Option<usize>, // adds special mark to task
    pub is_invalid: bool, // if invalid
    pub is_blank: bool, // default request
    pub
    args_count: usize, // args count
}

impl ArgsHandler {
    const HELP_FLAG: &'static str = "--help";
    const D_FLAG: &'static str = "-d";
    const M_FLAG: &'static str = "-m";
    const C_FLAG: &'static str = "-c";

    pub fn new(args_count: usize, args: &Vec<String>) -> ArgsHandler {
        let mut handler = ArgsHandler { help_flag: false, delete_flag_and_index: None, marker_flag_and_index: None, change_flag_and_index: None, is_invalid: false, is_blank: false, args_count };

        if args.iter().any(|arg| *arg == Self::HELP_FLAG) {
            handler.help_flag = true;
        }
        else if args.iter().any(|arg| *arg == Self::D_FLAG) {
            let index = args.iter().enumerate().find(|(_, x)| *x == Self::D_FLAG).unwrap().0;
            if args.iter().nth(index + 1).unwrap().parse::<usize>().is_err() { // if the argument after flag is not a number
                println!("{}", "Error: d flag requires number".red());
                handler.is_invalid = true;
            }
            handler.delete_flag_and_index = Some(index);
        }
        else if args.iter().any(|arg| *arg == Self::M_FLAG) {
            let index = args.iter().enumerate().find(|(_, x)| *x == Self::M_FLAG).unwrap().0;
            if args.iter().nth(index + 1).unwrap().parse::<usize>().is_err() { // if the argument after flag is not a number
                println!("{}", "Error: m flag requires number".red());
                handler.is_invalid = true;
            }
            handler.marker_flag_and_index = Some(index);
        }
        else if args.iter().any(|arg| *arg == Self::C_FLAG) {
            let index = args.iter().enumerate().find(|(_, x)| *x == Self::C_FLAG).unwrap().0;
            if args.iter().nth(index + 1).unwrap().parse::<usize>().is_err() { // if the argument after flag is not a number
                println!("{}", "Error: c flag requires number".red());
                handler.is_invalid = true;
            }
            handler.change_flag_and_index = Some(index);
        }
        else if handler.args_count == 1 {
            handler.is_blank = true;
        }

        handler
    }
}