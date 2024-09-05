use std::{borrow::Borrow, string::String};

#[derive(PartialEq, Debug)]
pub enum Token {
    Colon,
    Dash,
    Number,
    Book(String),
}

pub fn is_bible_book(input: &str) -> bool {
    let it = input.to_lowercase();
    it == "1 chronicles"
    || it == "1 corinthians"
    || it == "1 john"
    || it == "1 kings"
    || it == "1 peter"
    || it == "1 samuel"
    || it == "1 thessalonians"
    || it == "1 timothy"
    || it == "2 chronicles"
    || it == "2 corinthians"
    || it == "2 john"
    || it == "2 kings"
    || it == "2 peter"
    || it == "2 samuel"
    || it == "2 thessalonians"
    || it == "2 timothy"
    || it == "3 john"
    || it == "acts"
    || it == "amos"
    || it == "colossians"
    || it == "daniel"
    || it == "deuteronomy"
    || it == "ecclesiastes"
    || it == "ephesians"
    || it == "esther"
    || it == "exodus"
    || it == "ezekiel"
    || it == "ezra"
    || it == "galatians"
    || it == "genesis"
    || it == "habakuk"
    || it == "haggai"
    || it == "hebrews"
    || it == "hosea"
    || it == "isaiah"
    || it == "james"
    || it == "jeremiah"
    || it == "job"
    || it == "joel"
    || it == "john"
    || it == "jonah"
    || it == "joshua"
    || it == "jude"
    || it == "judges"
    || it == "lamentations"
    || it == "leviticus"
    || it == "luke"
    || it == "malachi"
    || it == "mark"
    || it == "matthew"
    || it == "micah"
    || it == "nahum"
    || it == "nehemiah"
    || it == "numbers"
    || it == "philemon"
    || it == "philippians" 
    || it == "proverbs"
    || it == "psalms"
    || it == "revelation"
    || it == "romans"
    || it == "ruth"
    || it == "songs of solomon"
    || it == "titus"
    || it == "zechariah"
    || it == "zephaniah"
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
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn parse_test() {
        let it = scan("1 Timothy 2:1-2");

        let actual = it.get(0).unwrap();

        let expected = Token::Book("1 Timothy".to_string());

        assert_eq!(*actual, expected);
    }
}
