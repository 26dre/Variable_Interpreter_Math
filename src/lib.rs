mod tokenizer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tokenizer_tests {
    
    // use super::*;
    // use tokenizer::Tokenizer;

    use crate::tokenizer::{self, build_tokenizer, Tokenizer};

    #[test]
    fn it_works() {
        let mut test: Tokenizer = build_tokenizer("hello");
        let identify_token = test.identify_token();
        assert_eq!(identify_token, tokenizer::Token::IDENTIFIER("hello".to_string()) )


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
