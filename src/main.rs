use gtk4::glib;
use gtk4::prelude::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gtk4::{
    Application, ApplicationWindow, Builder, Button, FileChooserAction, FileChooserDialog,
    ResponseType, TextView,
};

const TAPE_LENGTH: usize = 30000;
fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.text_viewer"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

pub fn build_ui(application: &Application) {
    let ui_src = include_str!("text_viewer.ui");
    let builder = Builder::new();
    builder
        .add_from_string(ui_src)
        .expect("Couldn't add from string");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let open_button: Button = builder.object("open_button").expect("Couldn't get builder");
    let text_view: TextView = builder.object("text_view").expect("Couldn't get text_view");
    text_view
        .buffer()
        .set_text("Hello World program written in brainfuck:\n--<-<<+[+[<+>--->->->-<<<]>]<<--.<++++++.<<-..<<.<+.>>.>>.<<<.+++.>>.>>-.<<<+.");
    open_button.connect_clicked(glib::clone!(@weak window, @weak text_view => move |_| {

        let file_chooser = FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            FileChooserAction::Open,
            &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );

        file_chooser.connect_response(move |d: &FileChooserDialog, response: ResponseType| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");

                let filename = file.path().expect("Couldn't get file path");
                let file = File::open(filename).expect("Couldn't open file");

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                text_view.buffer().set_text(&contents);
            }

            d.close();
        });

        file_chooser.show();
    }));

    let button_bottom: Button = builder
        .object("button_bottom")
        .expect("Couldn't get button_bottom");

    button_bottom.connect_clicked(glib::clone!(@weak text_view => move |_| {
        let buffer = text_view.buffer();
        let start = buffer.start_iter();
        let end = buffer.end_iter();
        let text = buffer.text(&start, &end, false);
        println!("Running program...");
        buffer.set_text(&process((&text).to_string()));
        println!("Completed");
    }));

    window.show();
}
fn process(mut data_string: String) -> String {
    let mut tape: [u8; TAPE_LENGTH] = [0; TAPE_LENGTH];
    data_string.retain(|c| r#"><+-.,[]"#.contains(c));
    let data: Vec<char> = data_string.chars().collect();
    let mut cur_line: Vec<char> = Vec::new();
    let mut pointer: usize = 0;
    let data_length = data.len();
    let mut data_pointer: usize = 0;
    let mut brackets: Vec<usize> = Vec::new();
    while data_pointer < data_length {
        match data[data_pointer] {
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
                if tape[pointer] != 0 {
                    if tape[pointer] as char == ' ' {
                        for _ in 0..2 {
                            cur_line.push(' ');
                        }
                    } else {
                        cur_line.push(tape[pointer] as char);
                    }
                }
                // print!("{}", tape[pointer] as char);
            } // print the crrent byte
            ',' => {
                // not implemented in gui
                // set current byte to terminal input
                // let mut input = String::new();
                // io::stdin().read_line(&mut input).unwrap();
                // tape[pointer] = input.trim().chars().next().unwrap() as u8;
            }
            '[' => {
                let mut counter: u8 = 0;
                data_pointer += 1;
                brackets.push(data_pointer);
                if tape[pointer] == 0 {
                    loop {
                        let c = data[data_pointer];
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
                if brackets.len() == 0 {
                    panic!("Unbalanced brackets");
                }
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

    return String::from_iter(cur_line);
}
