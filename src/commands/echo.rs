use std::io::Write;
use std::fs::OpenOptions;

pub struct Echo;

impl Echo {
    pub fn execute(&self, input: &str) {
        let rest = input.strip_prefix("echo ").unwrap_or(input);
        let args = self.split_arguments(rest);

        let mut omit_newline = false;
        let mut enable_escapes = false;
        let mut texts = Vec::new();

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with('-') && arg != "-" {
                let options = arg.chars().skip(1);
                for c in options {
                    match c {
                        'n' => omit_newline = true,
                        'e' => enable_escapes = true,
                        'E' => enable_escapes = false,
                        _ => {}
                    }
                }
                i += 1;
            } else {
                break;
            }
        }

        texts.extend_from_slice(&args[i..]);

        // code portion for slicing upto '>'
        let index = texts.iter().position(|s| s == ">" || s == "1>").unwrap_or_else(|| texts.len().saturating_sub(1));

        let new_in = &texts[0..index];

        let (processed_output, omit_due_to_c) = self.process_texts(new_in, enable_escapes);
        let output = processed_output.join(" ");

        if let Some(filename) = args.get(index+1){
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true).open(filename).expect("unable to open/create file");

            write!(file, "{}", output).expect("unable to write to file");
        }
        else if !output.is_empty() {
            print!("{}", output);
        }

        if !omit_newline && !omit_due_to_c {
            println!();
        }
    }

    fn split_arguments(&self, input: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut in_single = false;
        let mut in_double = false;
        let mut escape = false;

        for c in input.chars() {
            if escape {
                current_arg.push(c);
                escape = false;
            } else if in_single {
                if c == '\'' {
                    in_single = false;
                } else {
                    current_arg.push(c);
                }
            } else if in_double {
                if c == '\\' {
                    escape = true;
                } else if c == '"' {
                    in_double = false;
                } else {
                    current_arg.push(c);
                }
            } else {
                match c {
                    ' ' | '\t' | '\n' => {
                        if !current_arg.is_empty() {
                            args.push(current_arg.clone());
                            current_arg.clear();
                        }
                    }
                    '\\' => escape = true,
                    '\'' => in_single = true,
                    '"' => in_double = true,
                    _ => current_arg.push(c),
                }
            }
        }

        if !current_arg.is_empty() {
            args.push(current_arg);
        }

        args
    }

    fn process_texts(&self, texts: &[String], enable_escapes: bool) -> (Vec<String>, bool) {
        let mut processed = Vec::new();
        let mut omit_due_to_c = false;

        for text in texts {
            if enable_escapes {
                let (processed_text, omit) = self.process_escapes(text);
                processed.push(processed_text);
                if omit {
                    omit_due_to_c = true;
                    break;
                }
            } else {
                processed.push(text.clone());
            }
        }

        (processed, omit_due_to_c)
    }

    fn process_escapes(&self, s: &str) -> (String, bool) {
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        let mut omit = false;

        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(esc) = chars.next() {
                    match esc {
                        '\\' => result.push('\\'),
                        'a' => result.push('\x07'),
                        'b' => result.push('\x08'),
                        'e' => result.push('\x1b'),
                        'f' => result.push('\x0c'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        'v' => result.push('\x0b'),
                        'c' => {
                            omit = true;
                            break;
                        }
                        '0' => {
                            let mut octal = String::new();
                            for _ in 0..3 {
                                if let Some(&d) = chars.peek() {
                                    if d.is_digit(8) {
                                        octal.push(d);
                                        chars.next();
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            if let Ok(val) = u32::from_str_radix(&octal, 8) {
                                if let Some(ch) = char::from_u32(val) {
                                    result.push(ch);
                                }
                            }
                        }
                        'x' => {
                            let mut hex = String::new();
                            for _ in 0..2 {
                                if let Some(&d) = chars.peek() {
                                    if d.is_digit(16) {
                                        hex.push(d);
                                        chars.next();
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            if let Ok(val) = u32::from_str_radix(&hex, 16) {
                                if let Some(ch) = char::from_u32(val) {
                                    result.push(ch);
                                }
                            }
                        }
                        _ => result.push(esc),
                    }
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }

        (result, omit)
    }
}