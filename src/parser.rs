
use std::collections::HashMap;
use crate::tokenizer::{self, Tokenizer};
#[derive(Debug, PartialEq)]
struct Parser <'a> {
   internal_tokenizer: Tokenizer, 
   variable_assignments: HashMap<String, isize>, 
   input_str: &'a String
}

pub fn init_parser(in_str: &String) -> Parser{
    Parser {
        internal_tokenizer : tokenizer::build_tokenizer(&in_str),
        variable_assignments : HashMap::new(),
        input_str : in_str
    }
}

impl <'a> Parser <'a>{

}

