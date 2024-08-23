use std::{borrow::Borrow, string::String};

pub enum Token {
    Colon,
    Dash,
    Number,
    Book(String),
}

pub fn is_bible_book(input: &str) -> bool {
    let it = input.to_lowercase();
    it == "1 Chronicles"
    || it == "1 Corinthians"
    || it == "1 John"
    || it == "1 Kings"
    || it == "1 Peter"
    || it == "1 Samuel"
    || it == "1 Thessalonians"
    || it == "1 Timothy"
    || it == "2 Chronicles"
    || it == "2 Corinthians"
    || it == "2 John"
    || it == "2 Kings"
    || it == "2 Peter"
    || it == "2 Samuel"
    || it == "2 Thessalonians"
    || it == "2 Timothy"
    || it == "3 John"
    || it == "Acts"
    || it == "Amos"
    || it == "Colossians"
    || it == "Daniel"
    || it == "Deuteronomy"
    || it == "Ecclesiastes"
    || it == "Ephesians"
    || it == "Esther"
    || it == "Exodus"
    || it == "Ezekiel"
    || it == "Ezra"
    || it == "Galatians"
    || it == "Genesis"
    || it == "Habakuk"
    || it == "Haggai"
    || it == "Hebrews"
    || it == "Hosea"
    || it == "Isaiah"
    || it == "James"
    || it == "Jeremiah"
    || it == "Job"
    || it == "Joel"
    || it == "John"
    || it == "Jonah"
    || it == "Joshua"
    || it == "Jude"
    || it == "Judges"
    || it == "Lamentations"
    || it == "Leviticus"
    || it == "Luke"
    || it == "Malachi"
    || it == "Mark"
    || it == "Matthew"
    || it == "Micah"
    || it == "Nahum"
    || it == "Nehemiah"
    || it == "Numbers"
    || it == "Philemon"
    || it == "Philippians" 
    || it == "Proverbs"
    || it == "Psalms"
    || it == "Revelation"
    || it == "Romans"
    || it == "Ruth"
    || it == "Songs of Solomon"
    || it == "Titus"
    || it == "Zechariah"
    || it == "Zephaniah"
}


pub struct ParseState<'a> {
    pub start: usize,
    pub current: usize,
    pub line: i32,
    pub input: &'a str
}

impl<'a> ParseState<'a> {
    pub fn new(input: &'a str) -> Self {
        ParseState {
            start: 0, 
            current: 0, 
            line: 0,
            input: input
        }
    }

    pub fn advance(self) -> Self {
        ParseState {
            current: self.current + 1,
            ..self
        }
    }

    pub fn advance_to(self, index: usize) -> Self {
        ParseState {
            current: index,
            ..self
        }
    }

    pub fn advance_line(self) -> Self {
        ParseState {
            current: self.current + 1,
            line: self.line + 1,
            start: self.start,
            input: self.input
        }
    }

    pub fn get_current_selection(&self) -> &str {
        &self.input[self.start..=self.current]
    }

    pub fn get_selection_up_to(&self, next: usize) -> &str {
        &self.input[self.start..next]
    }

    pub fn get_selection_through(&self, next: usize) -> &str {
        &self.input[self.start..=next]
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    pub fn token_complete(&self, index: usize) -> Self {
        ParseState {
            start: index,
            current: index,
            line: self.line,
            input: self.input
        }
    }

    pub fn check_for_token(self, next_in: (usize, char)) -> (Option<Token>, ParseState<'a>) {
        match next_in.1 {
            ':' => (Some(Token::Colon), self.token_complete(next_in.0)),
            '-' => (Some(Token::Dash), self.token_complete(next_in.0)),
            ' ' => {
                let sub_str = self.get_current_selection();

                match sub_str {
                    "1" => (None, self.advance_to(next_in.0)),
                    "2" => (None, self.advance_to(next_in.0)),
                    _ => {
                        if is_bible_book(sub_str) {
                            (Some(Token::Book(sub_str.to_string())), self.token_complete(next_in.0))
                        } else {
                            (None, self.advance_to(next_in.0))
                        }
                    }
                }
            },
            _ => (None, self.advance_to(next_in.0))
        }
    }
}


pub struct Lines {
    pub start: u32,
    pub end: Option<u32>
}

impl Lines {
    pub fn new(start: u32, end: Option<u32>) -> Self {
        Lines { start, end }
    }
}


pub struct Verse {
    pub book: String,
    pub chapter: u32,
    pub lines: Lines
}




pub fn scan(input: &str) -> Vec<Token> {
    let mut state: ParseState = ParseState::new(input);

    let mut tokens = Vec::new();

    for i in input.char_indices() {
        let (it, next_state) = state.check_for_token(i);
        if let Some(result) = it {
            tokens.push(result);
        }
        state = next_state;
    }

    tokens
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
