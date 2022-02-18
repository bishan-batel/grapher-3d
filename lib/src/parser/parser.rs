use js_sys::SyntaxError;
use wasm_bindgen::JsValue;

use crate::parser::Operator;

use super::{
    native::{NativeFunc, NATIVE_FUNCTIONS},
    ParseNode, ParserError, Token,
};

pub struct Parser {
    toks: Vec<Token>,
    idx: usize,
}

// TODO ammend extra methods
impl Parser {
    // constructor
    pub fn new(toks: Vec<Token>) -> Self {
        Self { toks, idx: 0 }
    }

    pub fn parse(&mut self) -> Result<ParseNode, ParserError> {
        // first token must be a function name
        let func_name = if let Some(tok) = self.curr_tok() {
            match tok {
                Token::Identifier(name) => {
                    self.advance();
                    Ok(name)
                }
                _ => Err(ParserError::SyntaxError(String::new())),
            }
        } else {
            // return parse node that doesnt graph anything if input is empty
            return Ok(ParseNode::FunctionDefine(
                "default".into(),
                vec![],
                Box::new(ParseNode::Factor(f32::MIN)),
            ));
        }?;

        // assure ( comes after
        if let Some(tok) = self.curr_tok() {
            match tok {
                Token::LeftParen => {
                    self.advance();
                }
                _ => return Err(ParserError::SyntaxError("Missing (".into())),
            }
        } else {
            return Err(ParserError::SyntaxError("Missing (".into()));
        }

        let mut args = vec![];

        while let Some(tok) = self.curr_tok() {
            match tok {
                Token::Identifier(name) => {
                    args.push(name);
                    self.advance();
                }
                _ => return Err(ParserError::SyntaxError("Invalid arg".into())),
            }

            if let Some(tok) = self.curr_tok() {
                match tok {
                    Token::ArgumentSeperator => {
                        self.advance();
                    }
                    Token::RightParen => {
                        self.advance();
                        break;
                    }
                    _ => return Err(ParserError::SyntaxError("Missing ,".into())),
                }
            } else {
                return Err(ParserError::SyntaxError("Unexpected End".into()));
            }
        }

        // assure = comes after
        if let Some(tok) = self.curr_tok() {
            match tok {
                Token::Equals => {
                    self.advance();
                }
                _ => return Err(ParserError::SyntaxError("Missing (".into())),
            }
        } else {
            return Err(ParserError::SyntaxError("Missing (".into()));
        }

        // returns node tree
        Ok(ParseNode::FunctionDefine(
            func_name,
            args,
            Box::new(self.add_term()?),
        ))
    }

    fn add_term(&mut self) -> Result<ParseNode, ParserError> {
        let mut node = self.mul_term()?;

        // loop until the current token is nonexistant OR is
        // not a + or -
        while let Some(tok) = self.curr_tok() {
            match tok {
                Token::BinOp(op) => match op {
                    // valid for + or -
                    Operator::Add | Operator::Sub => {
                        self.advance(); // move forward

                        // gets term for the right hand side
                        let rhs = Box::new(self.mul_term()?);

                        // moves previous node down the tree
                        node = ParseNode::BinOp(Box::new(node), op, rhs);
                    }

                    // if not + or - operator then break out of loop
                    _ => break,
                },
                // if not a operator then break out of loop
                _ => break,
            }
        }

        // returns node tree
        Ok(node)
    }

    fn mul_term(&mut self) -> Result<ParseNode, ParserError> {
        let mut node = self.pow_term()?;

        // loop until the current token is nonexistant OR is
        // not a * or /
        while let Some(tok) = self.curr_tok() {
            match tok {
                Token::BinOp(op) => match op {
                    // valid for * or /
                    Operator::Mul | Operator::Div => {
                        self.advance(); // move forward

                        // gets term for the right hand side
                        let rhs = Box::new(self.pow_term()?);

                        // moves previous node down the tree
                        node = ParseNode::BinOp(Box::new(node), op, rhs);
                    }

                    // if not * or / operator then break out of loop
                    _ => break,
                },
                // if not a operator then break out of loop
                _ => break,
            }
        }

        // returns node tree
        Ok(node)
    }

    fn pow_term(&mut self) -> Result<ParseNode, ParserError> {
        let mut node = self.factor()?;

        // loop until the current token is nonexistant OR is
        // not a ^
        while let Some(tok) = self.curr_tok() {
            match tok {
                Token::BinOp(op) => match op {
                    // valid for ^
                    Operator::Pow => {
                        self.advance(); // move forward

                        // gets term for the right hand side
                        let rhs = Box::new(self.factor()?);

                        // moves previous node down the tree
                        node = ParseNode::BinOp(Box::new(node), op, rhs);
                    }

                    // if not ^ operator then break out of loop
                    _ => break,
                },
                // if not a operator then break out of loop
                _ => break,
            }
        }

        // returns node tree
        Ok(node)
    }

    fn factor(&mut self) -> Result<ParseNode, ParserError> {
        let tok = self.curr_tok();

        // ensure token is valid
        if tok.is_none() {
            return Err(ParserError::SyntaxError(
                "Unexpected end of equation".into(),
            ));
        }

        let tok = tok.unwrap();

        match tok {
            Token::Literal(val) => {
                self.advance();
                Ok(ParseNode::Factor(val))
            }
            Token::LeftParen => {
                self.advance();

                // parses interior of left paren
                let node = self.add_term()?;
                Ok(node)
            }

            Token::Identifier(name) => {
                self.advance();

                // attempts to read as function, skips if not
                if let Some(tok) = self.curr_tok() {
                    if matches!(tok, Token::LeftParen) {
                        self.advance();
                        Ok(ParseNode::Function(name, self.read_args()?))
                    } else {
                        Ok(ParseNode::Identifier(name))
                    }
                } else {
                    Ok(ParseNode::Identifier(name))
                }
            }

            // if any other token then throw error
            _ => Err(ParserError::SyntaxError(format!(
                "Unexpected Token {}",
                tok
            ))),
        }
    }

    fn read_args(&mut self) -> Result<Vec<ParseNode>, ParserError> {
        let mut args = vec![];
        while let Some(tok) = self.curr_tok() {
            if matches!(tok, Token::RightParen) {
                self.advance();
                break;
            }

            args.push(self.add_term()?);

            if let Some(tok) = self.curr_tok() {
                match tok {
                    Token::ArgumentSeperator => {
                        self.advance();
                    }
                    // end of argument list
                    Token::RightParen => {
                        self.advance();
                        break;
                    }
                    _ => return Err(ParserError::SyntaxError("Missing ,".into())),
                }
            } else {
                return Err(ParserError::SyntaxError("Unexpected End".into()));
            }
        }
        Ok(args)
    }

    fn advance(&mut self) {
        self.idx += 1;
    }

    /// returns current token parser is on
    fn curr_tok(&self) -> Option<Token> {
        let tok = self.toks.iter().nth(self.idx);

        // converts Option<&Token> type to Option<Token>
        // comes at the disadvantage of cloning the token but
        // the value is so small the performance detriment is neglible
        if let Some(tok) = tok {
            Some(tok.clone())
        } else {
            None
        }
    }

    pub fn validate(node: &ParseNode) -> Result<(), JsValue> {
        let dependicies = Self::get_function_dependicies(node);

        for func in dependicies.iter() {
            // if any functions are not native then return err
            if !NativeFunc::is_native(func) {
                let msg = format!(
                    "Function not defined {} with {} inputs",
                    func.0, // funcname
                    func.1  // input count
                );
                return Err(msg.into());
            }
        }

        Ok(())
    }

    pub fn get_function_dependicies(func: &ParseNode) -> Vec<(String, usize)> {
        let mut depends = vec![];

        match func {
            ParseNode::Function(name, args) => {
                depends.push((name.clone(), args.len()));

                // traverses down trees
                for arg in args.iter() {
                    let mut dependency = Self::get_function_dependicies(arg);
                    depends.append(&mut dependency);
                }
            }
            ParseNode::BinOp(lhs, _, rhs) => {
                // traverses down
                depends.append(&mut Self::get_function_dependicies(&**lhs));
                depends.append(&mut Self::get_function_dependicies(&**rhs));
            }
            ParseNode::UnaryOp(_, rhs) => {
                // traverses down
                depends.append(&mut Self::get_function_dependicies(&**rhs));
            }
            ParseNode::FunctionDefine(_, _, body) => {
                // traverses down body
                depends.append(&mut Self::get_function_dependicies(&**body));
            }
            // nothing to do for factors & identifiers
            ParseNode::Factor(..) | ParseNode::Identifier(..) => {}
        }

        return depends;
    }

    pub fn get_function_args(func: &ParseNode) -> Vec<String> {
        let mut args = vec![];

        match func {
            ParseNode::FunctionDefine(_, func_args, _) => {
                for arg in func_args.iter() {
                    args.push(arg.clone());
                }
            }
            _ => panic!("Input is not a function define"),
        }

        args
    }
}
