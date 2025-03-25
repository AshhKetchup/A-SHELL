pub fn parse_input(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();
    let mut buffer = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;

    while let Some(c) = chars.next() {
        if escaped {
            // Preserve the next character literally
            buffer.push(c);
            escaped = false;
            continue;
        }

        match c {
            '\\' if !in_single_quotes => {
                escaped = true; // Escape next character (except in single quotes)
            }
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes; // Toggle single-quoted state
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes; // Toggle double-quoted state
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !buffer.is_empty() {
                    result.push(buffer.clone());
                    buffer.clear();
                }
            }
            _ => {
                buffer.push(c);
            }
        }
    }

    if !buffer.is_empty() {
        result.push(buffer);
    }

    result
}
