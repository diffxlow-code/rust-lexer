//! The scanner module is responsible for turning source code into a sequence of
//! tokens..This is similarly written like the lox interpreter tokenizer
//! Halka english ta thokny paryo ni
use crate::tokens::Literal;
use crate::err;
use crate::tokens::Token;
use crate::tokens::TokenType;

// github ma vako issue hera ani lifetime kina add gareko bujhxau
#[derive(Debug)] // yo pretty print grada halka sajilo hunxa
pub struct Scanner<'a> {
    pub source: &'a str, // yo chai function call garni bela banako original padheko file ko
                         // content ho
    pub tokens: Vec<Token<'a>>, // yo hamro output ho .. yeima iterate garera tokens haru print
                                // garne ho
    pub start: usize, 
    pub current: usize, // string slice ko kun character ma aayim vanxa
    pub line: usize,
}

impl<'a> Scanner<'a> {
    // naya scanner banaune
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

    // scanner banayepaxi token scan garna use hune function
    pub fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof,"EOF", None, self.line));
        self.tokens.clone()
    }
    
    // source_str padkim kinai ?
    pub fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }
    

    // hya samma comment padhiraxau vane legend
    pub fn scan_token(&mut self) {
        let  character = self.advance();
        match character {
            ' ' | '\r' | '\t'  => {}, // whitespace,tab ko lagi

            '(' => {
                // malai yo function ma rakhna jhau matra lako ho
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
            '\n' => self.line +=  1 ,// new line vayepaxi self.line ma 1 add garne
            'a'..='z' | 'A'..='Z' => self.identifier(), // yema keyword match garxam ani macth
                                                        // vayena vane identifier ho vanne declare
                                                        // gardinxum
            '0'..='9' => self.number(), // sabbai soucre code string ma padkheko vayera yesto garnu
                                        // parkeo parsed_num le yo fix gardihalxa
            '"' => self.scan_string_literal(), // yo "" ma vayeko text extract garna
            _ => {
                err(self.line, "No such character exists"); // character ascii range ma
                                                                     // xaina ,wa invalid token ho
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
        // is_ascii_alphanumeric le valid alphabetic wa space wa kei ho vanera chinxa
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' || self.peek() == '"'{ 
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current]; // aile samma detect garkeo
                                                                   // keyword
        // keyword match garne aba (ramailo part)
        match lexeme {


            // yo ta bujhxau hola ni
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
        // yedi parsed_num valid vayo vane vayo natra err report garne
        if let Ok(n) = parsed_num.parse::<f64>() {
            self.tokens.push(Token::new(TokenType::Number, 
                    parsed_num, 
                    Some(Literal::Number(n)), 
                    self.line));
        } else {
            err(self.line, "Err parsing num"); 
        }
    }
    

    // aba next aune character lai read garne but not consume
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

