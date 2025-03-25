pub struct Echo;

impl Echo {
    pub fn execute(&self, input: &str) {
        let rest = input.strip_prefix("echo ").unwrap_or(input);
        let mut result = Vec::new();
        let mut temp = String::new();
        let mut in_quotes = false;
        let mut chars = rest.chars().peekable();
        let mut prev_was_quote = false;

        while let Some(c) = chars.next() {
            match c {
                '\'' => {
                    if in_quotes {
                        // Closing quote
                        in_quotes = false;
                        prev_was_quote = true;
                    } else {
                        // Opening quote
                        in_quotes = true;
                        prev_was_quote = false;
                    }
                }
                ' ' if !in_quotes => {
                    if !temp.is_empty() {
                        result.push(temp.clone());
                        temp.clear();
                    }
                    prev_was_quote = false;
                }
                _ => {
                    if prev_was_quote {
                        // Adjacent quoted strings should be merged without space
                        if !temp.is_empty() {
                            result.push(temp.clone());
                            temp.clear();
                        }
                    }
                    temp.push(c);
                    prev_was_quote = false;
                }
            }
        }

        if !temp.is_empty() {
            result.push(temp);
        }

        println!("{}", result.join(" "));
    }
}
