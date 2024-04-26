use std::collections::HashMap;
// use colored::Colorize;
// use stduse colored::Colorize;
use colored::Colorize;

use crate::tokenizer::{self, build_tokenizer, Token, Tokenizer};

#[derive(Debug, PartialEq)]
pub struct Parser {
    internal_tokenizer: Tokenizer,
    variable_assignments: HashMap<String, isize>,
    curr_token: Token, //    string_iter:
}
///Initializes the parser from an input string references
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

pub fn match_similar<T: std::fmt::Debug + Eq>(lhs: &T, rhs: &T) -> bool {
    // println!("{:?}", lhs);

    if rhs == lhs {
        return true;
    } else {
        panic!("{}{:?} is not equivalent to {:?}", "Syntax incorrect ".red(),lhs, rhs);
    }
}


impl Parser {
    pub fn parse_input(&mut self) -> Vec<isize> {
        let mut ret_vec = Vec::new();

        self.get_and_set_next_token();

        match_similar(&self.curr_token, &Token::COMPUTATION); 
        self.computation_handler();
        let mut val:isize = 0; 

        while self.curr_token != Token::NONE {
            
            match self.peek_token() {
                Token::VAR => {
                    self.var_preprocessor();
                    self.var_handler();
                }, 
                Token::SEMICOLON => {
                    self.get_and_set_next_token();
                }, 
                Token::PERIOD => {
                    self.set_next_token(Token::NONE);
                    // self.get_and_set_next_token();
                    // ret_vec.push(val);
                    // println!("Result : {val}");
                    // val = 0; 

                    // if self.curr_token != Token::NONE{
                    //     match_similar(&self.peek_token(), &Token::COMPUTATION);
                    //     self.get_and_set_next_token();
                    // }

                }, 
                _ => {
                    val += self.handle_expression();
                    ret_vec.push(val);
                    println!("{val}");
                    val = 0; 
                }
            }
        }
        // if self.curr_token == Token::NONE {
        //     ret_vec.push(val);
        // }

        ret_vec
    }

 
    fn handle_expression(&mut self) -> isize {
        // -9999
        // self.get_and_set_next_token();
        println!("Evaluating curr token {:?}", self.curr_token);
        let mut ret_val: isize = self.handle_term() as isize;


        while self.curr_token == Token::PLUS || self.curr_token == Token::MINUS {
            let prev_token = self.peek_token();

            self.get_and_set_next_token();

            println!("\tPrevious token = {:?}, curr token = {:?}", prev_token, self.curr_token);

            match prev_token {
                Token::PLUS => {
                    ret_val += self.handle_term() as isize;
                }
                _ => {
                    ret_val -= self.handle_term() as isize;
                }
            }
        }
        ret_val
    }

    fn handle_term(&mut self) -> isize {
        let mut ret_val = self.handle_factor();

        while self.curr_token == Token::TIMES || self.curr_token == Token::DIVIDE {
            let prev_token = self.peek_token();
            
            self.get_and_set_next_token();

            

            if prev_token == Token::TIMES {
                ret_val *= self.handle_factor();
            } else {
                ret_val /= self.handle_factor();
            }
        }

        ret_val
    }

    fn handle_factor(&mut self) -> isize {

        match &self.curr_token {
            Token::IDENTIFIER(var_name) => {
                let actual_val = self.variable_assignments.get(var_name);
                if actual_val.is_some() {
                    self.set_next_token(Token::NUMBER(*actual_val.unwrap() as usize));
                } else {
                    panic!("Variable name invalid");
                }
            },
            _ => {}

        }
        match self.curr_token {
            Token::NUMBER(num_val) => {
                self.get_and_set_next_token();
                return num_val as isize;
            }
            Token::OPEN_PAREN => {
                // self.get_and_set_next_token();
                println!("\t\tChanged next token to {:?} from OPEN_PAREN", self.curr_token);
                self.get_and_set_next_token();
                let temp = self.handle_expression();
                match_similar(&self.curr_token, &Token::CLOSE_PAREN);
                self.get_and_set_next_token();
                return temp;
            },

            
            _ => {
                println!("\t\t{}{} {:?} {}", "Factor ".blue().bold(), "Evaluating current token of".bright_red(), self.curr_token,  "before panicking".bright_red());
                panic!("{}", "Syntax incorrect, expected number or opening paren, not this ".red());
            }
        }
    }

    fn var_preprocessor (&mut self) {
        match_similar(&Token::VAR, &self.curr_token);
        self.get_and_set_next_token();
        // match_similar(&Token::ASSIGNMENT, &self.curr_token);
        // self.get_and_set_next_token(); 
       
    }
    fn var_handler(&mut self) {

       

        let string: String = match &self.curr_token {
            Token::IDENTIFIER(input_str) => {
                input_str.to_string()
            },
            _ => panic!("{} {:?} {} {}", "Incorrect syntax in variable declaration received token".red(), self.curr_token,"but expected".red(), "IDENTIFIER".red().bold() )
        };

        self.get_and_set_next_token();
        match_similar(&self.curr_token, &Token::ASSIGNMENT);

        println!("{:?}", self.curr_token);

        self.get_and_set_next_token();
        
        let val = self.handle_expression();

        self.variable_assignments.insert(string, val);

        

        

        //next thing should be variable assignment

        // self.get_and_set_next_token(); 
        // match_similar(&self.curr_token, &Token::ASSIGNMENT);
        // self.get_and_set_next_token();
        // // match_similar(&Token::NUMBER(std::any), &Token::NUMBER(std::any));

        // let value = self.handle_expression();

        // self.variable_assignments.insert(*string, value);



    }

    fn computation_handler(&mut self) {

        self.get_and_set_next_token();

        

        // if (self.curr_token != Token::NONE || self.curr_token != )

        // self.get_and_set_next_token()
    }

    // fn function_directory (&mut self) {
    //     match self.curr_token {
    //         Token::VAR =>
    //     }
    // }

   

    // fn assignment_handler(&mut self, var_name: &str) {

    // }

    // fn semicolon_handler(&mut self) {
    //     match_similar(&self.curr_token, &Token::SEMICOLON);
    //     self.get_and_set_next_token();
    // }

    fn get_next_token(&mut self) -> Token {
        // Token::NONE
        self.internal_tokenizer.identify_next_token()
        //place holder value above

        //skip whitespace
        //get next token
        //work from there given what the token is
        //should have a token match here huh
    }
    fn get_and_set_next_token(&mut self) {
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

    fn peek_token(&self) -> Token {
        self.curr_token.clone()
    }
}

#[cfg(test)]

mod internal_parser_test{
    use super::*;
    #[test]
    fn handle_expression_addition_test() {
        let mut test_parser = init_parser(&"140 + 14".to_string());
        test_parser.get_and_set_next_token();

        let x = test_parser.handle_expression();

        assert_eq!(x, 154);
    }
    #[test]
    fn handle_expression_multiplication() {
        let mut test_parser = init_parser(&"140 * 10".to_string());
        test_parser.get_and_set_next_token();

        let x = test_parser.handle_expression();

        assert_eq!(x, 1400);
    }
    #[test]
    fn handle_expression_parentheses() {
        let mut test_parser = init_parser(&"10 * (11+2)".to_string());
        test_parser.get_and_set_next_token();

        let x = test_parser.handle_expression();

        assert_eq!(x, 130);
    }

    #[test]
    fn handle_expression_addl() {
        let mut test_parser = init_parser(&"(((100*20)+10)+(3+1)))".to_string());
        test_parser.get_and_set_next_token();
        let x = test_parser.handle_expression();
        assert_eq!(2014, x);
    }
    #[test]
    fn var_test() {
        let mut test_parser = init_parser(&"hello <- 10".to_string());
        // test_parser.get_and_set_next_token();
        test_parser.get_and_set_next_token();
        let x = test_parser.var_handler();
        println!("{:?}" , test_parser.variable_assignments);
        assert_eq!(*test_parser.variable_assignments.get(&"hello".to_string()).unwrap(), 10 as isize);
    }
    #[test]
    fn var_test_assignement() {
        let mut test_parser = init_parser(&"hello <- 10".to_string());
        test_parser.get_and_set_next_token();
        let x = test_parser.var_handler();
        println!("{:?}" , test_parser.variable_assignments);
        assert_eq!(*test_parser.variable_assignments.get(&"hello".to_string()).unwrap(), 10 as isize);

        test_parser.internal_tokenizer = build_tokenizer("hello + 11");
        test_parser.get_and_set_next_token();
        let y = test_parser.handle_expression();
        assert_eq!(y, 21);

    } 
}