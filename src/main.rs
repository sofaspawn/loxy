use std::env;
use std::fs;
use std::io::{self, Write};

/*
enum Lexemes{
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    EOF
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                tokenize(file_contents);
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(file_contents: String) {
    let mut error = false;
    let mut lno = 1;
    for line in file_contents.split('\n') {
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '(' => println!("LEFT_PAREN {c} null"),
                ')' => println!("RIGHT_PAREN {c} null"),
                '{' => println!("LEFT_BRACE {c} null"),
                '}' => println!("RIGHT_BRACE {c} null"),
                '.' => println!("DOT {c} null"),
                ',' => println!("COMMA {c} null"),
                '*' => println!("STAR {c} null"),
                '+' => println!("PLUS {c} null"),
                '-' => println!("MINUS {c} null"),
                ';' => println!("SEMICOLON {c} null"),
                '=' => {
                    if let Some('=') = chars.peek() {
                        chars.next();
                        println!("EQUAL_EQUAL == null");
                    } else {
                        println!("EQUAL {c} null");
                    }
                }
                '!' => {
                    if let Some('=') = chars.peek() {
                        chars.next();
                        println!("BANG_EQUAL != null");
                    } else {
                        println!("BANG {c} null");
                    }
                }
                '<' => {
                    if let Some('=') = chars.peek() {
                        chars.next();
                        println!("LESS_EQUAL <= null");
                    } else {
                        println!("LESS {c} null");
                    }
                }
                '>' => {
                    if let Some('=') = chars.peek() {
                        chars.next();
                        println!("GREATER_EQUAL >= null");
                    } else {
                        println!("GREATER {c} null");
                    }
                }
                '/' => {
                    if let Some('/') = chars.peek() {
                        while let Some(c) = chars.next() {
                            if c == '\n' {
                                break;
                            }
                        }
                    } else {
                        println!("SLASH {c} null");
                    }
                }
                '"' => {
                    let mut string = String::new();
                    let mut unterminated = true;
                    while let Some(x) = chars.next() {
                        if x == '"' {
                            unterminated = false;
                            break;
                        }
                        string.push(x);
                    }
                    if unterminated {
                        error = true;
                        eprintln!("[line {lno}] Error: Unterminated string.");
                    } else {
                        println!("STRING \"{}\" {}", string, string);
                    }
                }
                d => {
                    let mut dig = String::new();
                    if d.is_ascii_digit() {
                        dig.push(d);
                        while let Some(x) = chars.next() {
                            if x.is_ascii_digit() || x == '.' {
                                dig.push(x);
                            } else {
                                break;
                            }
                        }
                    }
                    let num = dig.parse::<f64>().unwrap();
                    println!("NUMBER {dig} {num:?}");
                }
                '\t' | ' ' | '\r' => {}
                _ => {
                    eprintln!("[line {lno}] Error: Unexpected character: {c}");
                    error = true;
                }
            }
        }
        lno += 1;
    }
    println!("EOF  null");
    if error {
        std::process::exit(65);
    }
}
