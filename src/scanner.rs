// inspired from the lox language tokenizer
use crate::tokens::Literal;
use crate::err;
use crate::tokens::Token;
use crate::tokens::TokenType;

// added liftimes to fix github issue
#[derive(Debug)] 
pub struct Scanner<'a> {
    pub source: &'a str, 
    pub tokens: Vec<Token<'a>>, // output tokens
    pub start: usize, 
    pub current: usize,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof,"EOF", None, self.line));
        self.tokens.clone()
    }
    
    // did we read source_str ?
    pub fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }
    

    pub fn scan_token(&mut self) {
        let  character = self.advance();
        match character {
            ' ' | '\r' | '\t'  => {}, // whitespace,tab

            '(' => {
                self.tokens.push(Token::new(TokenType::LeftParen, "(", None, self.line)); 
                                                                                                        
            }

            ')' => {
                self.tokens.push(Token::new(TokenType::RightParen, "(", None, self.line));
            }

            '{' => {
                self.tokens.push(Token::new(TokenType::LeftBrace, "{", None, self.line));
            }

            '}' => {
                self.tokens.push(Token::new(TokenType::RightBrace, "}", None, self.line));
            }

            ',' => {
                self.tokens.push(Token::new(TokenType::Comma, ",", None, self.line));
            }

            '-' => {
                self.tokens.push(Token::new(TokenType::Minus, "_", None, self.line));
            }

            '+' => {
                self.tokens.push(Token::new(TokenType::Plus, "+", None, self.line));
            }

            ';' => {
                self.tokens.push(Token::new(TokenType::Semicolon, ";", None, self.line));
            }

            '*' => {
                self.tokens.push(Token::new(TokenType::Star, "*", None, self.line));
            }

            '=' => {
                if self.peek() == '=' {
                self.tokens.push(Token::new(TokenType::Equal, "=", None, self.line));
                } else {
                    self.tokens.push(Token::new(TokenType::EqualEqual, "==", None, self.line));
                } 
            }

            '!' => {
                if self.peek() == '=' {
                self.tokens.push(Token::new(TokenType::BangEqual, "!=", None, self.line));
                } else {
                    self.tokens.push(Token::new(TokenType::Bang, "==", None, self.line));
                } 
            }

            '>' => {
                if self.peek() == '=' {
                self.tokens.push(Token::new(TokenType::GreaterEqual, ">=", None, self.line));
                } else {
                    self.tokens.push(Token::new(TokenType::Greater, ">", None, self.line));
                } 
            }

            '<' => {
                if self.peek() == '=' {
                self.tokens.push(Token::new(TokenType::LessEqual, "!=", None, self.line));
                } else {
                    self.tokens.push(Token::new(TokenType::Less, "<=", None, self.line));
                } 
            }
            '\n' => self.line +=  1 ,
            'a'..='z' | 'A'..='Z' => self.identifier(), // all keyword and identifier code goes here
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
        // yema chai string "" vitra hunxa ani tyo quotes haru katdinxa 
        let source_str = &self.source[self.start + 1..self.current - 1]; 
        
        // string literal push garne token ma
        self.tokens.push(Token::new(TokenType::Str, source_str, Some(Literal::Str(source_str)), self.line));
    }

    pub fn identifier(&mut self ) {
       
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' || self.peek() == '"'{ 
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current]; // string slice of current lexeme
        // will be efficient to use a static string map
        match lexeme {
            "yedi" => {
                self.tokens.push(Token::new(TokenType::If, 
                        lexeme,
                        None,
                        self.line));
            },

            "ra" => {
                self.tokens.push(Token::new(TokenType::And, 
                        lexeme,
                        None,
                        self.line));
            },

            "natra" => {
                self.tokens.push(Token::new(TokenType::Else, 
                        lexeme,
                        None,
                        self.line));
            },

            "jhut" => {
                self.tokens.push(Token::new(TokenType::False, 
                        lexeme,
                        None,
                        self.line));
            },

            "sathi" => {
                self.tokens.push(Token::new(TokenType::Fun, 
                        lexeme,
                        None,
                        self.line));
            },

            "forr" => {
                self.tokens.push(Token::new(TokenType::For, 
                        lexeme,
                        None,
                        self.line));
            },

            "khali" => {
                self.tokens.push(Token::new(TokenType::Nil, 
                        lexeme,
                        None,
                        self.line));
            },

            "wa" => {
                self.tokens.push(Token::new(TokenType::Or, 
                        lexeme,
                        None,
                        self.line));
            },

            "lekh" => {
                self.tokens.push(Token::new(TokenType::Print, 
                        lexeme,
                        None,
                        self.line));
            },

            "pathau" => {
                self.tokens.push(Token::new(TokenType::Return, 
                        lexeme,
                        None,
                        self.line));
            },

            "satya" => {
                self.tokens.push(Token::new(TokenType::True, 
                        lexeme,
                        None,
                        self.line));
            },

            "value" => {
                self.tokens.push(Token::new(TokenType::Var, 
                        lexeme,
                        None,
                        self.line));
            },

            "jabasamma" => {
                self.tokens.push(Token::new(TokenType::While, 
                        lexeme,
                        None,
                        self.line));
            },

            _ => {
                self.tokens.push(Token::new(TokenType::Identifier, 
                        lexeme, 
                        Some(Literal::Str(lexeme)), 
                        self.line));
            }
        }
    }


    pub fn number(&mut self ) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        let parsed_num = &self.source[self.start..self.current];
        // the sliced string is instantly parsed into a number
        if let Ok(n) = parsed_num.parse::<f64>() {
            self.tokens.push(Token::new(TokenType::Number, 
                    parsed_num, 
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

