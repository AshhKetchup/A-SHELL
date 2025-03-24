struct InputParser<'a> {
    input: &'a str,
    iter: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> InputParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            iter: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for InputParser<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip leading whitespace
        while let Some(&c) = self.iter.peek() {
            if c.is_whitespace() {
                self.iter.next();
            } else {
                break;
            }
        }

        // Return None if there's no more input
        if self.iter.peek().is_none() {
            return None;
        }

        let start = self.input.len() - self.iter.clone().count();
        let mut in_single_quotes = false;
        let mut in_double_quotes = false;

        while let Some(c) = self.iter.next() {
            match c {
                '\'' if !in_double_quotes => in_single_quotes = !in_single_quotes,
                '"' if !in_single_quotes => in_double_quotes = !in_double_quotes,
                ' ' if !(in_single_quotes || in_double_quotes) => break,
                _ => {}
            }
        }

        let end = self.input.len() - self.iter.clone().count();
        Some(self.input[start..end].trim())
    }
}

pub fn parse_input(input: &str) -> Vec<&str> {
    InputParser::new(input).collect()
}
