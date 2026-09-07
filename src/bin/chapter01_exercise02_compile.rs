#[derive(Debug)]
struct StrSplit<'s, 'p> {
    remainder: Option<&'s str>,
    delimiter: &'p str,
}

impl<'s, 'p> StrSplit<'s, 'p> {
    fn new(haystack: &'s str, delimiter: &'p str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delimiter) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delimiter];
            *remainder = &remainder[next_delimiter + self.delimiter.len()..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    let delimiter = format!("{}", c);
    StrSplit::new(s, &delimiter)
        .next()
        .expect("StrSplit always gives at least one result")
}

fn main() {
    let haystack = String::from("Hello World!");
    let firstword = until_char(&haystack, ' ');
    println!("FirstWord: {}", firstword);
}
