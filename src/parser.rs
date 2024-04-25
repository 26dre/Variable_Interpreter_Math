use std::collections::{binary_heap::Iter, HashMap};
use std::iter;
// use std::
use crate::tokenizer::{self, Token, Tokenizer};
const CLOSING_PAREN: char = ')';
const OPENING_PAREN: char = '(';
const PLUS: char = '+';
const TIMES: char = '*';
const DIVIDE: char = '/';
const MINUS: char = '-';
const PERIOD: char = '.';
#[derive(Debug, PartialEq)]
pub struct Parser {
    internal_tokenizer: Tokenizer,
    variable_assignments: HashMap<String, isize>,
    curr_token: Token, //    string_iter:
}

pub fn init_parser(in_str: &String) -> Parser {
    Parser {
        internal_tokenizer: tokenizer::build_tokenizer(&in_str),
        variable_assignments: HashMap::new(),
        curr_token: Token::NONE,
    }
}
// fn match_similar<T>(obj1: T , obj2: T) -> bool{
//     if (obj1 == obj2)
// }

/// .
///
/// # Panics
///
/// Panics if .
///

fn match_similar<T: std::fmt::Debug + Eq>(lhs: &T, rhs: &T) -> bool {
    // println!("{:?}", lhs);

    if rhs == lhs {
        return true;
    } else {
        panic!("Syntax incorrect {:?} is not equivalent to {:?}", lhs, rhs);
    }
}

// fn parse_input ()

impl Parser {
    fn parse_input(&self) -> Vec<isize> {
        let ret_vec = Vec::new();

        ret_vec
    }

    fn handle_expression(&mut self) -> isize {
        // -9999

        let mut ret_val : isize = self.handle_term() as isize;

        while (self.curr_token == Token::PLUS || self.curr_token == Token::MINUS) {

            let prev_token = self.curr_token.clone();

            self.get_and_set_next_token();

            match prev_token {
                Token::PLUS => {
                    ret_val += self.handle_term() as isize;
                }, 
                _ => {
                    ret_val -= self.handle_term() as isize;
                }
            }
            
        }
        -999
    }

    fn handle_term(&mut self) -> usize{

        9999
    }

    fn factor(&self) -> usize {
        9999
    }

    fn var_handler(&self) {}

    fn computation_handler(&self) {

        // if (self.curr_token != Token::NONE || self.curr_token != )

        // self.get_and_set_next_token()
    }

    fn assignment_handler(&mut self, var_name : &str) {}

    fn semicolon_handler(&mut self) {
        match_similar(&self.curr_token, &Token::SEMICOLON);
    }

    fn get_next_token(&mut self) -> Token {
        // Token::NONE
        self.internal_tokenizer.identify_token()
        //place holder value above

        //skip whitespace
        //get next token
        //work from there given what the token is
        //should have a token match here huh
    }
    fn get_and_set_next_token(&mut self){
        // Token::NONE
        // let ret_token = self.get_next_token();
        // self.set_next_token(ret_token);
        // ret_token
        let token = self.get_next_token();
        self.set_next_token(token);
        
    }
    fn set_next_token(&mut self, in_token: Token) {
        self.curr_token = in_token;
    }
    
}

#[cfg(test)]
mod parser_test {
    use super::*;
    // #[test]
    // fn parser_creation_test() {
    //     let
    // }
}
