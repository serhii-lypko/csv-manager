use anyhow::Result;

// NOTE:
// if a field contains a comma, that field should be enclosed in double quotes

// TODO:
// - handle quoted fields
// - handle quoted fields (with commas)
// - handle incomplete rows (some fields missing)

struct Parser {
    file_contents: String,
    with_header: bool,
    header: Option<Vec<String>>,
}

impl Parser {
    fn new(file_contents: String, with_header: bool) -> Self {
        Parser {
            file_contents,
            with_header,
            header: None,
        }
    }

    fn parse(&mut self) {
        if self.file_contents.is_empty() {
            return;
        }

        if self.with_header {
            if let Some(header_line) = self.file_contents.lines().next() {
                let header = self.parse_header(header_line);
                self.header = Some(header);
            }
        }

        for line in self.file_contents.lines() {
            //
        }

        dbg!(&self.header);
    }

    fn parse_header(&self, header_line: &str) -> Vec<String> {
        header_line.split(",").map(|s| s.to_string()).collect()
    }
}

pub fn parse(file_contents: String, with_header: bool) -> Result<()> {
    let mut parser = Parser::new(file_contents, with_header);

    let foo = parser.parse();

    Ok(())
}
