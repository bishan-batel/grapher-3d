use crate::{op_tok, parser::ParserError};

use super::Token;

// Macros ---------------------------------------------------------------------

// handle for main tokenization loop
macro_rules! handle {
    ($v: expr) => {
        if $v {
            continue;
        }
    };
}

// pattern for match statements
macro_rules! is_digit {
    () => {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    };
}

/// Lexer used to tokenize a math equation string
pub struct Lexer {
    tokens: Vec<Token>,
    index: usize,
    src: String,
}

/// macro for digit pattern, used for match statements  
impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src,
            index: 0,
            tokens: vec![],
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, ParserError> {
        // empties token list
        self.tokens = vec![];

        while let Some(curr) = self.curr_char() {
            // skip whitespace
            if curr.is_whitespace() {
                self.advance();
                continue;
            }

            // character handles, if any function returns false then it will step
            // forward (essentially an is else statement)
            handle!(self.handle_literal()?);
            handle!(self.handle_operator()?);
            handle!(self.handle_identifier()?);

            // if no handles picked char & is not whitespace
            // return a syntax error
            let err_msg = format!("Invalid char at '{}'", curr);
            return Err(ParserError::SyntaxError(err_msg.into()));
        }

        // returns tokens successfully
        Ok(self.tokens)
    }

    // character handles
    fn handle_identifier(&mut self) -> Result<bool, ParserError> {
        let mut identifier = String::new();

        // while can read character
        while let Some(curr) = self.curr_char() {

            // if current char is alphebetic or '_' then continue
            if curr.is_ascii_alphabetic() || curr == '_' {
                identifier.push(curr);
                self.advance();
                continue;
            }

            // else break the look
            break;
        }

        // if no identifier read than return false
        if identifier.len() == 0 {
            Ok(false)
        // else add identifier to tokens list
        } else {
            self.tokens.push(Token::Identifier(identifier));
            Ok(true)
        }
    }

    fn handle_operator(&mut self) -> Result<bool, ParserError> {
        if let Some(curr) = self.curr_char() {
            let mut success = true;

            // Maps character to token
            match curr {
                // Non BinOp
                '(' => self.tokens.push(Token::LeftParen),
                ')' => self.tokens.push(Token::RightParen),
                '=' => self.tokens.push(Token::Equals),

                ',' => self.tokens.push(Token::ArgumentSeperator),

                // Bin Operators
                '+' => self.tokens.push(op_tok!(Add)),
                '-' => self.tokens.push(op_tok!(Sub)),
                '*' => self.tokens.push(op_tok!(Mul)),
                '/' => self.tokens.push(op_tok!(Div)),
                '^' => self.tokens.push(op_tok!(Pow)),

                // If any other character that means success is false
                _ => {
                    success = false;
                }
            }

            // if found successfully advance pointer forward
            if success {
                self.advance();
            }
            Ok(success)
        } else {
            Ok(false)
        }
    }

    fn handle_literal(&mut self) -> Result<bool, ParserError> {
        let mut num_builder = String::new();
        let mut decimal_count = 0usize;

        // while current char exists
        while let Some(curr) = self.curr_char() {
            // checks if current char is a valid digit
            if match curr {
                '.' => {
                    // add to decimal count
                    decimal_count += 1;
                    true
                }
                is_digit!() => true,
                _ => false,
            } {
                // pushes character to num builder
                num_builder.push(curr);

                // advances
                self.advance()
            } else {
                // break out of read loop if it encounters a non digit valid character
                break;
            }
        }

        // if no number built then return false
        if num_builder.len() == 0 {
            Ok(false)
        }
        // if multiple decimals in result than return error
        else if decimal_count > 1 {
            Err(ParserError::SyntaxError("Multiple decimals".into()))

        // Appends token literal to tokens list
        } else {
            let num = num_builder.as_str().parse::<f32>().unwrap();
            let tok = Token::Literal(num);
            self.tokens.push(tok);
            Ok(true)
        }
    }

    // moves index forward by 1
    fn advance(&mut self) {
        self.index += 1;
    }

    // moves index back by 1
    fn regress(&mut self) {
        self.index -= 1;
    }

    /// Grabs character that index lies on
    fn curr_char(&mut self) -> Option<char> {
        self.src.chars().nth(self.index)
    }
}
