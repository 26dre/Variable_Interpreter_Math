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
    VAR, 




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



    




}
