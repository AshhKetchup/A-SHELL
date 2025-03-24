pub fn parse_input(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();
    let mut buffer = String::new();
    let mut in_quotes = false;

    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
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
