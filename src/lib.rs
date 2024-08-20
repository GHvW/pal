use std::string::String;

pub enum TokenType {
    SemiColon,
    Book,
    Number 
}

pub struct ParseState {
    pub start: usize,
    pub current: usize,
    pub line: i32
}

impl ParseState {
    pub fn new(start: usize, current: usize, line: i32) -> Self {
        ParseState {
            start, 
            current, 
            line
        }
    }

    pub fn advance(&self) -> Self {
        ParseState {
            current: self.current + 1,
            ..self
        }
    }

    pub fn advance_line(&self) -> Self {
        ParseState {
            current: self.current + 1,
            line: self.line + 1,
            start: self.start
        }
    }

    pub fn get_current_selection(&self, input: &str) -> String {
        input[self.start..self.current].to_string()
    }

    pub fn is_at_end(&self, input: &str) -> bool {
        self.current >= input.len()
    }


    pub fn current_char(&self, input: &str) -> char {
        input.[self.current]
    }
}


pub struct Lines {
    start: u32,
    end: Option<u32>
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

pub fn is_at_end(state: &ParseState, input: &str) -> bool {
    state.current >= input.len()
}


pub fn current_char(state: &ParseState, input: &str) -> char {
    input[state.current]
}

pub fn scan(input: &str) -> Vec<Verse> {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
