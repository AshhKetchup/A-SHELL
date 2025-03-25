pub struct Echo;

impl Echo {
    pub fn execute(&self, input: &str) {
        let rest = input.strip_prefix("echo ").unwrap_or(input);
        let mut result = String::new();
        let mut temp = String::new();
        let mut in_quotes = false;
        let mut last_was_quoted = false;
        let mut space_buffer = false;
        let mut chars = rest.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\'' => {
                    if in_quotes {
                        // Closing a quoted section
                        in_quotes = false;
                        result.push_str(&temp);
                        temp.clear();
                        last_was_quoted = true;
                    } else {
                        // Opening a quoted section
                        in_quotes = true;
                        if !result.is_empty() && !last_was_quoted {
                            result.push(' '); // Ensure single space before new quoted section
                        }
                    }
                }
                ' ' if !in_quotes => {
                    if last_was_quoted {
                        result.push(' '); // Ensure single space after quoted strings
                        last_was_quoted = false;
                    }
                    space_buffer = true;
                }
                _ if in_quotes => {
                    temp.push(c);
                }
                _ => {
                    if space_buffer && !result.is_empty() {
                        result.push(' '); // Collapse spaces outside quotes
                    }
                    space_buffer = false;
                    result.push(c);
                    last_was_quoted = false;
                }
            }
        }

        println!("{}", result);
    }
}
