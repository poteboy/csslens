use crate::token::{Token, TokenType};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn build(&mut self) {
        // mappings of css token types to regex patterns
        let patterns: Vec<(TokenType, Regex)> = vec![
            (
                TokenType::ClassSelector,
                Regex::new(r"^\.[a-zA-Z_][a-zA-Z0-9_-]*").unwrap(),
            ),
            (
                TokenType::IdSelector,
                Regex::new(r"^#[a-zA-Z_][a-zA-Z0-9_-]*").unwrap(),
            ),
            (
                TokenType::Ident,
                Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_-]*").unwrap(),
            ),
            (
                TokenType::Atkeyword,
                Regex::new(r"^@[a-zA-Z_][a-zA-Z0-9_-]*").unwrap(),
            ),
            (TokenType::String, Regex::new(r#"^"[^"]*""#).unwrap()),
            (TokenType::Number, Regex::new(r"^[0-9]+").unwrap()),
            (TokenType::Percentage, Regex::new(r"^[0-9]+%").unwrap()),
            (TokenType::Colon, Regex::new(r"^:").unwrap()),
            (TokenType::Semicolon, Regex::new(r"^;").unwrap()),
            (TokenType::CurlyLeft, Regex::new(r"^\{").unwrap()),
            (TokenType::CurlyRight, Regex::new(r"^\}").unwrap()),
            (TokenType::ParenLeft, Regex::new(r"^\(").unwrap()),
            (TokenType::ParenRight, Regex::new(r"^\)").unwrap()),
            (TokenType::BracketLeft, Regex::new(r"^\[").unwrap()),
            (TokenType::BracketRight, Regex::new(r"^\]").unwrap()),
            (TokenType::S, Regex::new(r"^[ \t\r\n\f]+").unwrap()),
            (
                TokenType::Comment,
                Regex::new(r"^/\*[^*]*\*+([^/*][^*]*\*+)*/").unwrap(),
            ),
        ];

        // iterate over the source string to store tokens when matched with a regex pattern
        while self.current < self.source.len() {
            let remaining = &self.source[self.current..];
            let mut matched = false;

            for (token_type, regex) in &patterns {
                if let Some(mat) = regex.find(remaining) {
                    let lexeme = &remaining[mat.start()..mat.end()];

                    self.tokens.push(Token::new(
                        token_type.clone(),
                        lexeme.to_string(),
                        self.line,
                    ));

                    // increment line count if token type is whitespace & contains newline
                    if token_type == &TokenType::S {
                        self.line += lexeme.matches('\n').count();
                    }

                    self.current += lexeme.len();
                    matched = true;
                    break;
                }
            }

            if !matched {
                // skip to the next character if no regex pattern is matched
                self.current += 1;
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        // Arrange
        let source = r#"
            .container {
                display: flex;
                justify-content: center;
                align-items: center;
            }
        "#;
        let mut lexer = Lexer::new(source.to_string());
        lexer.build();
        let tokens = lexer.tokens();

        // Act: get the last token
        let last_token = tokens.last().cloned().unwrap().token_type;
        // Assert: the last token should be Eof
        assert_eq!(last_token, TokenType::Eof);

        // Act: get the first class selector token
        let class_selector = tokens
            .iter()
            .find(|t| t.token_type == TokenType::ClassSelector)
            .unwrap();
        // Assert: the first class selector token should be .container
        assert_eq!(class_selector.lexeme, ".container");

        // Act: get all ident tokens
        let idents: Vec<Token> = tokens
            .iter()
            .filter_map(|t| {
                if t.token_type == TokenType::Ident {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect();
        // Assert: there should be 6 ident tokens
        assert_eq!(idents.len(), 6);
    }
}