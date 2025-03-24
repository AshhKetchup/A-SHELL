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

    fn parse(&mut self) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut start = None;
        let mut in_quotes = false;

        while let Some(&c) = self.iter.peek() {
            match c {
                '\'' => {
                    self.iter.next(); // Consume the quote
                    if in_quotes {
                        // If closing quote, store the collected part (excluding quotes)
                        if let Some(s) = start {
                            result.push(&self.input[s..self.input.len() - self.iter.clone().count() - 1]);
                            start = None;
                        }
                    } else {
                        // If opening quote, start collecting from next character
                        start = Some(self.input.len() - self.iter.clone().count());
                    }
                    in_quotes = !in_quotes;
                }
                ' ' if !in_quotes => {
                    if let Some(s) = start {
                        result.push(&self.input[s..self.input.len() - self.iter.clone().count()]);
                        start = None;
                    }
                    self.iter.next(); // Consume the space
                }
                _ => {
                    if start.is_none() {
                        start = Some(self.input.len() - self.iter.clone().count());
                    }
                    self.iter.next(); // Consume the character
                }
            }
        }

        // Push the last segment if it's not empty
        if let Some(s) = start {
            result.push(&self.input[s..]);
        }

        result
    }
}

pub fn parse_input(input: &str) -> Vec<&str> {
    InputParser::new(input).parse()
}
