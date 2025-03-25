pub struct Echo;

impl Echo {
    pub fn execute(&self, input: &str) {
        let rest = input.strip_prefix("echo ").unwrap_or(input);
        let mut result = Vec::new();
        let mut temp = String::new();
        let mut chars = rest.chars().peekable();

        let mut in_single_quotes = false;
        let mut in_double_quotes = false;
        let mut escaped = false;

        while let Some(c) = chars.next() {
            if escaped {
                // Preserve the character as-is after a backslash
                temp.push(c);
                escaped = false;
                continue;
            }

            match c {
                '\\' => {
                    if !in_single_quotes && !in_double_quotes {
                        escaped = true; // Escape next character outside of single quotes
                    } else {
                        temp.push(c);
                    }
                }
                '\'' if !in_double_quotes => {
                    in_single_quotes = !in_single_quotes; // Toggle single-quoted state
                }
                '"' if !in_single_quotes => {
                    in_double_quotes = !in_double_quotes; // Toggle double-quoted state
                }
                ' ' if !in_single_quotes && !in_double_quotes && temp.is_empty() => {
                    continue; // Ignore leading spaces
                }
                ' ' if !in_single_quotes && !in_double_quotes => {
                    // Push accumulated word
                    result.push(temp.clone());
                    temp.clear();
                }
                _ => {
                    temp.push(c);
                }
            }
        }

        if !temp.is_empty() {
            result.push(temp);
        }

        println!("{}", result.join(" "));
    }
}
