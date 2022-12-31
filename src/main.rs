use std::env;
use std::fs;
use std::io;

const TAPE_LENGTH: usize = 30000;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut code_string = fs::read_to_string(file_path).unwrap();
    let mut tape: [u8; TAPE_LENGTH] = [0; TAPE_LENGTH];
    code_string.retain(|c| r#"><+-.,[]"#.contains(c));
    let code: Vec<char> = code_string.chars().collect();
    let mut pointer: usize = 0;
    let code_length = code.len();
    let mut data_pointer: usize = 0;
    let mut brackets: Vec<usize> = Vec::new();
    while data_pointer < code_length {
        match code[data_pointer] {
            '>' => {
                if pointer == TAPE_LENGTH - 1 {
                    pointer = 0;
                } else {
                    pointer += 1
                }
            } // increment pointer
            '<' => {
                if pointer == 0 {
                    pointer = TAPE_LENGTH - 1;
                } else {
                    pointer -= 1
                }
            } // decrement pointer
            '+' => {
                if tape[pointer] == 255 {
                    tape[pointer] = 0;
                } else {
                    tape[pointer] += 1;
                }
            } // increment the byte
            '-' => {
                if tape[pointer] == 0 {
                    tape[pointer] = 255;
                } else {
                    tape[pointer] -= 1;
                }
            } // decrement the byte
            '.' => {
                print!("{}", tape[pointer] as char);
            } // print the crrent byte
            ',' => {
                // set current byte to terminal input
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                tape[pointer] = input.trim().chars().next().unwrap() as u8;
            }
            '[' => {
                let mut counter: u8 = 0;
                data_pointer += 1;
                brackets.push(data_pointer);
                if tape[pointer] == 0 {
                    loop {
                        let c = code[data_pointer];
                        if c == ']' && counter == 0 {
                            break;
                        } else if c == '[' {
                            counter += 1;
                        } else if c == ']' {
                            counter -= 1;
                        }
                        data_pointer += 1;
                        if data_pointer == TAPE_LENGTH {
                            panic!("Unbalanced brackets");
                        }
                    }
                }

                continue;
            }
            ']' => {
                let left_bracket: usize = brackets[brackets.len() - 1];
                if tape[pointer] != 0 {
                    data_pointer = left_bracket;
                    continue;
                } else {
                    brackets.pop();
                }
            }
            _ => (), // ignore other characters
        }
        data_pointer += 1;
    }
}
