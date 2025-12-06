//! The scanner module is responsible for turning source code into a sequence of
//! tokens..This is similarly written like the lox interpreter tokenizer
use crate::tokens::Literal;
use crate::err;
use crate::tokens::Token;
use crate::tokens::TokenType;


#[derive(Debug)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    #[must_use]
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof,String::from("EOF"), None, self.line));
        self.tokens.clone()
    }

    pub fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        let  character = self.advance();
        match character {
            ' ' | '\r' | '\t'  => {},
            '(' => {
                self.tokens.push(Token::new(TokenType::LeftParen, String::from("("), None, self.line));
            }

            ')' => {
                self.tokens.push(Token::new(TokenType::RightParen, String::from("("), None, self.line));
            }
            '{' => {
                self.tokens.push(Token::new(TokenType::RightBrace, String::from("{"), None, self.line));
            }
            '}' => {
                self.tokens.push(Token::new(TokenType::LeftBrace, String::from("}"), None, self.line));
            }
            ',' => {
                self.tokens.push(Token::new(TokenType::Comma, String::from(","), None, self.line));
            }
            '-' => {
                self.tokens.push(Token::new(TokenType::Minus, String::from("_"), None, self.line));
            }
            '+' => {
                self.tokens.push(Token::new(TokenType::Plus, String::from("+"), None, self.line));
            }
            ';' => {
                self.tokens.push(Token::new(TokenType::Semicolon, String::from(";"), None, self.line));
            }
            '*' => {
                self.tokens.push(Token::new(TokenType::Star, String::from("*"), None, self.line));
            }
            '\n' => self.line +=  1 ,
            'a'..='z' | 'A'..='Z' => self.identifier(),
            '0'..='9' => self.number(),
            '"' => self.scan_string_literal(),
            _ => {
                err(self.line, "No such character exists");
            }
        }
    }
    
    pub fn scan_string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n' { self.line += 1 }
            self.advance();
        }

        if self.is_at_end() {
            err(self.line, "Unterminated String");
            return;
        }

        self.advance();

        let source_str = &self.source[self.start + 1..self.current - 1].to_string();

        self.tokens.push(Token::new(TokenType::Str, source_str.clone(), Some(Literal::Str(source_str.clone())), self.line));
    }
    pub fn identifier(&mut self ) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' || self.peek() == '"'{
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];
         match lexeme {
             "yedi" => {
                 self.tokens.push(Token::new(TokenType::If, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "ra" => {
                 self.tokens.push(Token::new(TokenType::And, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "natra" => {
                 self.tokens.push(Token::new(TokenType::Else, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "false" => {
                 self.tokens.push(Token::new(TokenType::False, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "jhut" => {
                 self.tokens.push(Token::new(TokenType::Fun, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "forr" => {
                 self.tokens.push(Token::new(TokenType::For, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "khali" => {
                 self.tokens.push(Token::new(TokenType::Nil, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "wa" => {
                 self.tokens.push(Token::new(TokenType::Or, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "lekh" => {
                 self.tokens.push(Token::new(TokenType::Print, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "pathau" => {
                 self.tokens.push(Token::new(TokenType::Return, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "satya" => {
                 self.tokens.push(Token::new(TokenType::True, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "value" => {
                 self.tokens.push(Token::new(TokenType::Var, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             "jabasamma" => {
                 self.tokens.push(Token::new(TokenType::While, 
                         lexeme.to_string(),
                         None,
                         self.line));
             },

             _ => {
                 self.tokens.push(Token::new(TokenType::Identifier, 
                         lexeme.to_string(), 
                         Some(Literal::Str(lexeme.to_string())), 
                         self.line));
             }
         }
    }
    
    
    pub fn number(&mut self ) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        let parsed_num = &self.source[self.start + 1 ..self.current - 1];
        if let Ok(n) = parsed_num.parse::<f64>() {
        self.tokens.push(Token::new(TokenType::Number, 
                parsed_num.to_string(), 
                Some(Literal::Number(n)), 
                self.line));
        } else {
           err(self.line, "Err parsing num"); 
        }
    }

    pub fn peek(&mut self ) -> char  {
        if self.is_at_end() {
            '\0'
        } else {
            let byte = self.source.as_bytes()[self.current];
            byte as char
        }
    }

    pub fn advance (&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let byte = self.source.as_bytes()[self.current];
        self.current += 1;
        byte as char
    }

}
