use std::collections::HashSet;
enum Token {
    IDENTIFIER (String), 
    NUMBER (isize), 
    OPEN_PAREN, 
    CLOSE_PAREN, 
    MINUS, 
    PLUS, 
    TIMES,
    DIVIDE, 
    SEMICOLON, 
    PERIOD,
    COMPUTATION, 
    VAR
}

#[derive(Debug, Default)]
struct Tokenizer {
    vec_string: Vec<char>,
    position: usize,
}

fn build_tokenizer(input_str: &str) -> Tokenizer {
    Tokenizer {
        vec_string: input_str.chars().collect(),
        position: 0,
    }
}





impl Tokenizer {
    fn next(&mut self) {
        self.position += 1;
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
            self.next()
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


    




}
