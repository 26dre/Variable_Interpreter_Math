mod tokenizer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tokenizer_tests {
    
    // use super::*;
    // use tokenizer::Tokenizer;

    use crate::tokenizer::{self, build_tokenizer, Tokenizer, Token};
    #[test]
    fn identifier_test() {
        let mut test: Tokenizer = build_tokenizer("hello");
        let identify_token = test.identify_token();
        assert_eq!(identify_token, Token::IDENTIFIER("hello".to_string()) )
    }
    #[test]
    fn opening_paren_test() {
        let mut test: Tokenizer = build_tokenizer("( hello");
        let identify_token = test.identify_token();
        assert_eq!(identify_token, Token::OPEN_PAREN )
    }
    #[test]
    fn var_assignment_test() {
        let mut test: Tokenizer = build_tokenizer("<- ");
        let identify_token = test.identify_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::ASSIGNMENT );
    }
}

#[cfg(test)]
mod other_tests {

    use super::*;
    #[test]
    fn its_me_mario() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
