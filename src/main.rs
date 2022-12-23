use std::env;
use std::io;
use std::fs;

const TAPE_LENGTH: usize = 200000;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut code = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut tape: [u8; TAPE_LENGTH] = [0; TAPE_LENGTH];
    code.retain(|c| r#"><+-.,[]"#.contains(c));
    let mut pointer: usize = 0;
    let code_length = code.chars().count();
    let mut data_pointer: usize = 0;
    while data_pointer < code_length {
        match code.chars().nth(data_pointer).unwrap(){
            '>' => pointer += 1, // increment pointer
            '<' => pointer -= 1, // decrement pointer
            '+' => tape[pointer] += 1, // increment the byte
            '-' => tape[pointer] -= 1, // decrement the byte
            '.' => print!("{}", tape[pointer] as char), // print the crrent byte
            ',' => {
                    // set current byte to terminal input
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    tape[pointer] = input.trim().chars().next().expect("Input must not be empty") as u8;
            },
            '[' => {
                let mut counter : i16 = 0;
                data_pointer += 1;
                    if tape[pointer] as u16 == 0 {
                        loop {
                            let c = code.chars().nth(data_pointer).unwrap();
                            if c == ']' && counter == 0 {
                                break;
                            } else if c == '[' {
                                counter += 1; 
                            } else if c == ']' {
                                counter -= 1;
                            }
                            data_pointer += 1;
                        }
                }
                continue;
            },
            ']' => {
                let mut counter : i16 = 0;
                    if tape[pointer] as u16 != 0 {
                        data_pointer -= 1;
                        loop {
                            let c = code.chars().nth(data_pointer).unwrap();
                            if c == '[' && counter == 0 {
                                break;
                            } else if c == ']' {
                                counter += 1; 
                            } else if c == '[' {
                                counter -= 1;
                            }
                            data_pointer -= 1;
                        }
                        continue;
                }
            }
            _ => continue,  // ignore other characters
        }
        data_pointer += 1;
    }
}


