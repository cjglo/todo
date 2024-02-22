
const MAX_ITERATIONS: usize = 12;

pub fn handle_text_wrap<'a>(line: &'a str, line_length: usize) -> Vec<&'a str> {
    let mut vec: Vec<&str> = vec![];

    let mut index = line_length;
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