// use std::collections::HashSet;

// const RESERVED_KEYWORD_SET: [&str; 2] = ["var", "computation"];
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Token {
    IDENTIFIER(String),
    NUMBER(usize),
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
    ASSIGNMENT,
    NONE,
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

#[derive(Debug, Default, PartialEq)]
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

    fn build_identifier(&mut self) -> String {
        let mut ret_str = String::new();
        while self.peek_curr_char().unwrap_or('\0').is_alphanumeric() {
            ret_str.push(self.peek_curr_char_unsafe());
            self.next();
        }

        ret_str
    }

    fn build_number(&mut self) -> usize {
        let mut ret_num: usize = 0;
        while self.peek_curr_char().unwrap_or('\0').is_numeric() {
            ret_num *= 10;
            // ret_num += ((self.peek_curr_char_unsafe()).to_digit(10)).unwrap();
            ret_num += self.peek_curr_char_unsafe().to_digit(10).unwrap() as usize;
            self.next();
        }
        ret_num
    }

    fn skip_whitespace(&mut self) {
        while self.peek_curr_char().unwrap_or('\0').is_whitespace() {
            self.next();
        }
    }

    pub fn identify_token(&mut self) -> Token {
        self.skip_whitespace();
        let in_char = self.next().unwrap_or('\0');

        match in_char {
            '(' => Token::OPEN_PAREN,
            ')' => Token::CLOSE_PAREN,
            '/' => Token::DIVIDE,
            '*' => Token::TIMES,
            ';' => Token::SEMICOLON,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '.' => Token::PERIOD,
            '\0' => Token::NONE,
            _ => {
                self.backtrack();
                self.identify_longer_token()
            }
        }
    }

    pub fn peek_token (&mut self) -> Token {
        let curr_token = self.identify_token();
        match &curr_token {
            Token::NONE => {
                
            }, 
            Token::IDENTIFIER(identifier_str) => {
                for _i in 0..identifier_str.len() {
                    self.backtrack();
                }
            }, 
            Token::NUMBER(num) => {
                let tmp_num = num.to_string();
                for _i in 0..tmp_num.len() {
                    self.backtrack();
                }
            }, 
            Token::ASSIGNMENT => {
                for _i in 0..2 {
                    self.backtrack();
                }
            }, 
            Token::VAR => {
                for _i in 0..("var".len()){
                    self.backtrack();
                }
            }, 
            Token::COMPUTATION => {
                for _i in 0..("computation".len()){
                    self.backtrack();
                }
            }, 
            _ => {
                self.backtrack();
            } 
        }

        curr_token
    }

    fn identify_longer_token(&mut self) -> Token {
        let mut in_char = self.peek_curr_char().unwrap_or('\0');
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

#[cfg(test)]
mod tokenizer_tests {

    use crate::tokenizer::{build_tokenizer, Token, Tokenizer};
    #[test]
    fn identifier_test() {
        let mut test: Tokenizer = build_tokenizer("hello");
        let identify_token = test.identify_token();
        assert_eq!(identify_token, Token::IDENTIFIER("hello".to_string()))
    }
    #[test]
    fn number_test() {
        let mut test: Tokenizer = build_tokenizer("1234");
        let identify_token = test.identify_token();
        assert_eq!(identify_token, Token::NUMBER(1234))
    }
    #[test]
    fn opening_paren_test() {
        let mut test: Tokenizer = build_tokenizer("( hello");
        let identify_token = test.identify_token();
        assert_eq!(identify_token, Token::OPEN_PAREN)
    }
    #[test]
    fn var_assignment_test() {
        let mut test: Tokenizer = build_tokenizer("<- ");
        let identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::ASSIGNMENT);
    }
    #[test]
    fn var_test() {
        let mut test: Tokenizer = build_tokenizer("var");
        let identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::VAR);
    }
    #[test]
    fn computation_test() {
        let mut test: Tokenizer = build_tokenizer("computation");
        let identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::COMPUTATION);
    }
    #[test]
    fn computation_test_capital() {
        let mut test: Tokenizer = build_tokenizer("comPUtaTion");
        let identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::COMPUTATION);
    }
    #[test]
    fn semicolon_test() {
        let mut test: Tokenizer = build_tokenizer(";");
        let identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::SEMICOLON);
    }
    #[test]
    fn period_test() {
        let mut test: Tokenizer = build_tokenizer(".");
        let identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::PERIOD);
    }

    #[test]
    fn whitespace_testing() {
        let mut test: Tokenizer = build_tokenizer("                 cargo    ");
        let identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::IDENTIFIER("cargo".to_string()));
    }
    #[test]
    fn two_tokens() {
        let mut test: Tokenizer = build_tokenizer("                 cargo   check ");
        let mut identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::IDENTIFIER("cargo".to_string()));
        identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::IDENTIFIER("check".to_string()));
    }
    #[test]
    fn two_tokens_no_whitespace() {
        let mut test: Tokenizer = build_tokenizer("                 cargo+ ");
        let mut identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::IDENTIFIER("cargo".to_string()));
        identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::PLUS);
    }
    #[test]
    fn lots_of_tokens() {
        let mut test: Tokenizer =
            build_tokenizer("                 cargo+-hello var computation * / . ");
        let mut token_vec: Vec<Token> = Vec::new();
        let mut identify_token = test.identify_token();
        while identify_token != Token::NONE {
            token_vec.push(identify_token);
            identify_token = test.identify_token();
            println!("{:?}", identify_token);
        }

        println!("{:?}", token_vec);
        assert_eq!(token_vec.len(), 9);
    }
    #[test]
    #[should_panic]
    fn panic_num() {
        let mut test: Tokenizer =
            build_tokenizer("                 cargo+-949he var computation * / . ");
        let mut token_vec: Vec<Token> = Vec::new();
        let mut identify_token = test.identify_token();
        while identify_token != Token::NONE {
            token_vec.push(identify_token);
            identify_token = test.identify_token();
            println!("{:?}", identify_token);
        }

        println!("{:?}", token_vec);
        assert_eq!(token_vec.len(), 9);
    }

    #[test]
    #[should_panic]
    fn panic2() {
        let mut test: Tokenizer =
            build_tokenizer("                 cargo<+-hello var computation * / . ");
        let mut token_vec: Vec<Token> = Vec::new();
        let mut identify_token = test.identify_token();
        while identify_token != Token::NONE {
            token_vec.push(identify_token);
            identify_token = test.identify_token();
            println!("{:?}", identify_token);
        }

        println!("{:?}", token_vec);
        assert_eq!(token_vec.len(), 9);
    }
    #[test]
    #[should_panic]
    fn panic3() {
        let mut test: Tokenizer =
            build_tokenizer("                 [cargo+-hello var computation * / . ");
        let mut token_vec: Vec<Token> = Vec::new();
        let mut identify_token = test.identify_token();
        while identify_token != Token::NONE {
            token_vec.push(identify_token);
            identify_token = test.identify_token();
            println!("{:?}", identify_token);
        }

        println!("{:?}", token_vec);
        assert_eq!(token_vec.len(), 9);
    }

    #[test]
    fn test_peek () {
        let mut test: Tokenizer =
        build_tokenizer("                 cargo+-hello var computation * / . "); 
        let mut identify_tok = test.peek_token(); 
        assert_eq!(identify_tok, Token::IDENTIFIER("cargo".to_string()));
        identify_tok = test.peek_token();
        assert_eq!(identify_tok, Token::IDENTIFIER("cargo".to_string()));
    }
    #[test]
    fn test_peek_num() {
        let mut test: Tokenizer =
        build_tokenizer("                 1234-hello var computation * / . "); 
        let mut identify_tok = test.peek_token(); 
        assert_eq!(identify_tok, Token::NUMBER(1234));
        identify_tok = test.peek_token();
        assert_eq!(identify_tok, Token::NUMBER(1234));
    }
    #[test]
    fn test_peek_one_char() {
        let mut test: Tokenizer =
        build_tokenizer("                 1234-hello var computation * / . "); 
        let mut identify_tok = test.identify_token(); 
        identify_tok = test.peek_token();
        assert_eq!(identify_tok, Token::MINUS);
        identify_tok = test.peek_token();
        assert_eq!(identify_tok, Token::MINUS);
    }

}
