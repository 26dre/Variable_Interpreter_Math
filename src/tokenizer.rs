// use std::collections::HashSet;

// const RESERVED_KEYWORD_SET: [&str; 2] = ["var", "computation"];
#[derive(Debug, PartialEq)]
pub enum Token {
    IDENTIFIER (String), 
    NUMBER (usize), 
    OPEN_PAREN, 
    CLOSE_PAREN, 
    MINUS, 
    PLUS, 
    TIMES,
    DIVIDE, 
    SEMICOLON, 
    PERIOD,
    COMPUTATION, 
    VAR, 
    ASSIGNMENT
}
fn reserved_keyword(input_str: &str) -> Option<Token> {
    let tmp = input_str.to_ascii_lowercase();
    if tmp == "var" {
        Some(Token::VAR)
    } else if tmp == "computation" {
        Some(Token::COMPUTATION)
    } else {
        None
    }
}

#[derive(Debug, Default)]
pub struct Tokenizer {
    vec_string: Vec<char>,
    position: usize,
}

pub fn build_tokenizer(input_str: &str) -> Tokenizer {
    Tokenizer {
        vec_string: input_str.chars().collect(),
        position: 0,
    }
}

impl Tokenizer {
    fn next(&mut self) -> Option<char> {
        //returns the char previous to going to the next character
        let tmp = self.peek_curr_char();
        self.position += 1;
        tmp
    }

    fn backtrack(&mut self) {
        self.position -= 1;
    }

    fn peek_curr_char(&self) -> Option<char> {
        if self.position < self.vec_string.len() {
            Some(self.vec_string[self.position])
        } else {
            None
        }
    }

    fn peek_curr_char_unsafe(&self) -> char {
        self.vec_string[self.position]
    }
    
    fn build_identifier(&mut self) -> String{
        let mut ret_str = String::new(); 
        while self.peek_curr_char().unwrap_or('\0').is_alphanumeric(){
            ret_str.push(self.peek_curr_char_unsafe());
            self.next();
        }

        ret_str
    }

    fn build_number(&mut self) -> usize {
        let mut ret_num: usize = 0; 
        while self.peek_curr_char().unwrap_or('\0').is_alphanumeric(){
            ret_num *= 10; 
            // ret_num += ((self.peek_curr_char_unsafe()).to_digit(10)).unwrap();
            ret_num += self.peek_curr_char_unsafe().to_digit(10).unwrap() as usize;

        }
        ret_num
    }

    fn skip_whitespace(&mut self) {
        while self.peek_curr_char().unwrap_or('\0').is_whitespace(){
            self.position += 1;
        }
    }
    
    pub fn identify_token (&mut self) -> Token {
        self.skip_whitespace();
        let in_char = self.peek_curr_char().unwrap_or('\0');

        match in_char {
            '(' => Token::OPEN_PAREN,
            ')' => Token::CLOSE_PAREN,
            '/' => Token::DIVIDE,
            '*' => Token::TIMES,
            ';' => Token::SEMICOLON,
            '+' => Token::PLUS, 
            '-' => Token::MINUS, 
            _ => self.identify_longer_token()
                }
    }

    fn identify_longer_token (&mut self) -> Token {
        let mut in_char = self.peek_curr_char().unwrap_or('\0') ;
        if in_char.is_alphabetic() {
            let ident_string = self.build_identifier();
            if reserved_keyword(&ident_string).is_some() {
                return reserved_keyword(&ident_string).unwrap();
            } else {
                return Token::IDENTIFIER(ident_string);
            }
        } else if in_char.is_numeric() {
            Token::NUMBER(self.build_number())
        } else if in_char == '<' {
            self.next();
            in_char = self.peek_curr_char().unwrap_or('\0');
            println!("Next = {}", in_char);
            if in_char == '-' {
                Token::ASSIGNMENT
            } else {
                panic!("Invalid variable")
            }
        } else {
            panic!("Invalid type shit")
        }
    }
}