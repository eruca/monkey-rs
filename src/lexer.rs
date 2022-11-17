use std::{iter::Peekable, str::Chars};

use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub deposit: Vec<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            deposit: vec![],
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.chars.next();
                    continue;
                }
                _ => break,
            }
        }
    }

    fn read_identifier(&mut self, leading: char) -> String {
        let mut chars = vec![leading];
        loop {
            if let Some(c) = self.chars.peek() {
                match c {
                    'a'..='z' | 'A'..='Z' => chars.push(self.chars.next().unwrap()),
                    _ => break,
                }
            }
        }
        chars.into_iter().collect()
    }

    fn read_number(&mut self, leading: char) -> String {
        let mut chars = vec![leading];
        loop {
            if let Some(c) = self.chars.peek() {
                match c {
                    '0'..='9' => chars.push(self.chars.next().unwrap()),
                    _ => break,
                }
            }
        }
        chars.into_iter().collect()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.next() {
            Some(ch) => match ch {
                '=' => Token::new(TokenType::ASSIGN, ch),
                ';' => Token::new(TokenType::SEMICOLON, ch),
                '(' => Token::new(TokenType::LPAREN, ch),
                ')' => Token::new(TokenType::RPAREN, ch),
                ',' => Token::new(TokenType::COMMA, ch),
                '+' => Token::new(TokenType::PLUS, ch),
                '{' => Token::new(TokenType::LBRACE, ch),
                '}' => Token::new(TokenType::RBRACE, ch),
                'a'..='z' | 'A'..='Z' => {
                    let ident = self.read_identifier(ch);
                    println!("ident: {:?}", ident);
                    Token::new(lookup_ident(&ident).unwrap_or(TokenType::IDENT), ident)
                }
                '0'..='9' => Token::new(TokenType::INT, self.read_number(ch)),
                _ => panic!("unknonw: |{ch}|"),
            },
            None => Token {
                typ: TokenType::EOF,
                literal: "".into(),
            },
        }
    }
}

mod tests {
    use super::{Lexer, TokenType};
    struct Suite {
        expected_type: TokenType,
        expected_literal: &'static str,
    }

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);"#;

        let tests = [
            Suite {
                expected_type: TokenType::LET,
                expected_literal: "let",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "five",
            },
            Suite {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=",
            },
            Suite {
                expected_type: TokenType::INT,
                expected_literal: "5",
            },
            Suite {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";",
            },
            Suite {
                expected_type: TokenType::LET,
                expected_literal: "let",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "ten",
            },
            Suite {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=",
            },
            Suite {
                expected_type: TokenType::INT,
                expected_literal: "10",
            },
            Suite {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";",
            },
            Suite {
                expected_type: TokenType::LET,
                expected_literal: "let",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "add",
            },
            Suite {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=",
            },
            Suite {
                expected_type: TokenType::FUNCTION,
                expected_literal: "fn",
            },
            Suite {
                expected_type: TokenType::LPAREN,
                expected_literal: "(",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "x",
            },
            Suite {
                expected_type: TokenType::COMMA,
                expected_literal: ",",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "y",
            },
            Suite {
                expected_type: TokenType::RPAREN,
                expected_literal: ")",
            },
            Suite {
                expected_type: TokenType::LBRACE,
                expected_literal: "{",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "x",
            },
            Suite {
                expected_type: TokenType::PLUS,
                expected_literal: "+",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "y",
            },
            Suite {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";",
            },
            Suite {
                expected_type: TokenType::RBRACE,
                expected_literal: "}",
            },
            Suite {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";",
            },
            Suite {
                expected_type: TokenType::LET,
                expected_literal: "let",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "result",
            },
            Suite {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "add",
            },
            Suite {
                expected_type: TokenType::LPAREN,
                expected_literal: "(",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "five",
            },
            Suite {
                expected_type: TokenType::COMMA,
                expected_literal: ",",
            },
            Suite {
                expected_type: TokenType::IDENT,
                expected_literal: "ten",
            },
            Suite {
                expected_type: TokenType::RPAREN,
                expected_literal: ")",
            },
            Suite {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";",
            },
            Suite {
                expected_type: TokenType::EOF,
                expected_literal: "",
            },
        ];

        let mut l = Lexer::new(input);

        for i in 0..tests.len() {
            let tok = l.next_token();

            assert_eq!(tok.typ, tests[i].expected_type, "the index: {i}");
            assert_eq!(tok.literal.as_str(), tests[i].expected_literal, "the literal index: {i}");
        }
    }
}
