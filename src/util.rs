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

    fn parse(&mut self) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        while let Some(&c) = self.iter.peek() {
            match c {
                '\'' => {
                    self.iter.next(); // Consume quote
                    in_quotes = !in_quotes; // Toggle quote state
                }
                ' ' if !in_quotes => {
                    if !current.is_empty() {
                        result.push(current.clone());
                        current.clear();
                    }
                    self.iter.next(); // Consume space
                }
                _ => {
                    current.push(c);
                    self.iter.next(); // Consume character
                }
            }
        }

        // Push last argument if exists
        if !current.is_empty() {
            result.push(current);
        }

        result
    }
}

pub fn parse_input(input: &str) -> Vec<String> {
    InputParser::new(input).parse()
}
