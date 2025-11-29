use crate::tokens::{Token};
use crate::tokens::TokenType;

#[derive(Debug)]
struct Scanner {
    source : String,
    tokens : Vec<Token>,
    start : usize,
    current : usize,
    line : usize, 
} 

impl Scanner {
    fn new ( source : String ) -> Self {
        Self {
            source : source,
            tokens : Vec::new(),
            start : 0,
            current : 0,
            line : 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        return self.tokens.clone();
    }

    pub fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }


    pub fn scan_token(&self) {
        unimplemented!();
    }
}


