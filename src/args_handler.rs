use colored::Colorize;

pub struct ArgsHandler {
    pub help_flag: bool, // displays help mesage
    pub delete_flag: bool, // deleting task
    pub is_invalid_or_blank: bool, // if invalid or default print
    pub
    args_count: usize, // args count
}

impl ArgsHandler {
    const HELP_FLAG: &'static str = "-help";
    const D_FLAG: &'static str = "-d";

    pub fn new(args_count: usize, args: &Vec<String>) -> ArgsHandler {
        let mut handler = ArgsHandler { help_flag: false, delete_flag: false, is_invalid_or_blank: false, args_count };

        if args.iter().any(|arg| *arg == Self::HELP_FLAG) {
            handler.help_flag = true;
        }
        else if args.iter().any(|arg| *arg == Self::D_FLAG) && handler.args_count <= 2 {
            println!("{}", "Error: passing only flag is not allowed".red());
            handler.is_invalid_or_blank = true;
        }
        else if args.iter().any(|arg| *arg == Self::D_FLAG) {
            let index = args.iter().enumerate().find(|(_, x)| *x == Self::D_FLAG).unwrap().0;
            if args.iter().nth(index + 1).unwrap().parse::<usize>().is_err() { // if the argument after flag is not a number
                println!("{}", "Error: flag requires number".red());
            }
            handler.delete_flag = true;
        }
        else if handler.args_count == 1 {
            handler.is_invalid_or_blank = true;
        }

        handler
    }
}