use std::fmt::write;
use crate::text_util;
use colored::{ColoredString, Colorize};
use phf::{phf_set, Set};
use serde::{Deserialize, Serialize};

pub static HIGHLIGHTED_DUE_DATES: Set<&'static str> = phf_set! {
    "TODAY",
    "NOW",
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToDo {
    pub task: String,
    pub due_date: Option<String>,
    pub char_marker: Option<char>,
}

impl ToDo {
    const LINE_LENGTH_LIMIT: u16 = 60;
    const NO_DUE_DATE_TEXT: &'static str = "<no due date>";
    const PADDING_BETWEEN_TASK_AND_DATE: &'static str = "  |  ";
    const TITLE_BEFORE_DUE_DATE: &'static str = "DUE ";


    fn formatted_due_date_string(&self) -> Vec<ColoredString> {
        let mut due_date_lines: Vec<ColoredString> = Vec::new();
        due_date_lines.push(Self::PADDING_BETWEEN_TASK_AND_DATE.to_string().white());

        if let Some(date) = &self.due_date {
            due_date_lines.push(Self::TITLE_BEFORE_DUE_DATE.white());
            due_date_lines.push(if HIGHLIGHTED_DUE_DATES.contains(date) { date.bright_red() } else { date.bright_yellow() });
        }
        else {
            due_date_lines.push(Self::NO_DUE_DATE_TEXT.bright_green());
        }

        if let Some(marker) = self.char_marker {
            due_date_lines.push(Self::PADDING_BETWEEN_TASK_AND_DATE.white());
            due_date_lines.push(marker.to_string().white());
            due_date_lines = due_date_lines.into_iter().map(|x| x.bright_black()).collect();
        }

        due_date_lines
    }

}

impl std::fmt::Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
        let task_lines = text_util::handle_text_wrap(&self.task, Self::LINE_LENGTH_LIMIT);
        let mut result = String::new();
        // This will check if to-do should be marked as dim, then color the number of the line, and break it it up to format it correctly
        if self.char_marker.is_some() {
            result = format!("{0: <60}", task_lines[0].bright_black());
        } else {
            result = format!("{0: <60}", task_lines[0].bright_blue());
        }
        let due_date = self.formatted_due_date_string();

        let color_lambda = |x: &str| {
            if self.char_marker.is_some() {
                x.bright_black()
            } else {
                x.bright_blue()
            }
        };

        write!(f, "{}", result)?;
        for each in due_date {
            write!(f, "{}", each)?;
        }
        write!(f, "{}", '\n')?;
        for each in task_lines.iter().skip(1) {
            write!(f, "{}", format!("{0: <50}\n", color_lambda(each)))?
        }
        Ok(())
    }
}
