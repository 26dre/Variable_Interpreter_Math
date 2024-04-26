pub mod parser;
pub mod tokenizer;
///Im a little retarded
// pub fn add(left: usize, right: usize) -> usize {
    
//     left + right
// }


// #[cfg(test)]
// mod other_tests {

//     use super::*;
//     // #[test]
//     fn its_me_mario() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
#[cfg(test)]
mod tokenizer_tests {

    use crate::tokenizer::{build_tokenizer, Token, Tokenizer};
    #[test]
    fn identifier_test() {
        let mut test: Tokenizer = build_tokenizer("hello");
        let identify_token = test.identify_next_token();
        assert_eq!(identify_token, Token::IDENTIFIER("hello".to_string()))
    }
    #[test]
    fn number_test() {
        let mut test: Tokenizer = build_tokenizer("1234");
        let identify_token = test.identify_next_token();
        assert_eq!(identify_token, Token::NUMBER(1234))
    }
    #[test]
    fn opening_paren_test() {
        let mut test: Tokenizer = build_tokenizer("( hello");
        let identify_token = test.identify_next_token();
        assert_eq!(identify_token, Token::OPEN_PAREN)
    }
    #[test]
    fn var_assignment_test() {
        let mut test: Tokenizer = build_tokenizer("<- ");
        let identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::ASSIGNMENT);
    }
    #[test]
    fn var_test() {
        let mut test: Tokenizer = build_tokenizer("var");
        let identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::VAR);
    }
    #[test]
    fn computation_test() {
        let mut test: Tokenizer = build_tokenizer("computation");
        let identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::COMPUTATION);
    }
    #[test]
    fn computation_test_capital() {
        let mut test: Tokenizer = build_tokenizer("comPUtaTion");
        let identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::COMPUTATION);
    }
    #[test]
    fn semicolon_test() {
        let mut test: Tokenizer = build_tokenizer(";");
        let identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::SEMICOLON);
    }
    #[test]
    fn period_test() {
        let mut test: Tokenizer = build_tokenizer(".");
        let identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::PERIOD);
    }

    #[test]
    fn whitespace_testing() {
        let mut test: Tokenizer = build_tokenizer("                 cargo    ");
        let identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::IDENTIFIER("cargo".to_string()));
    }
    #[test]
    fn two_tokens() {
        let mut test: Tokenizer = build_tokenizer("                 cargo   check ");
        let mut identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::IDENTIFIER("cargo".to_string()));
        identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::IDENTIFIER("check".to_string()));
    }
    #[test]
    fn two_tokens_no_whitespace() {
        let mut test: Tokenizer = build_tokenizer("                 cargo+ ");
        let mut identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::IDENTIFIER("cargo".to_string()));
        identify_token = test.identify_next_token();
        println!("curr token result = {:?}", identify_token);
        assert_eq!(identify_token, Token::PLUS);
    }
    #[test]
    fn lots_of_tokens() {
        let mut test: Tokenizer =
            build_tokenizer("                 cargo+-hello var computation * / . ");
        let mut token_vec: Vec<Token> = Vec::new();
        let mut identify_token = test.identify_next_token();
        while identify_token != Token::NONE {
            token_vec.push(identify_token);
            identify_token = test.identify_next_token();
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
        let mut identify_token = test.identify_next_token();
        while identify_token != Token::NONE {
            token_vec.push(identify_token);
            identify_token = test.identify_next_token();
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
        let mut identify_token = test.identify_next_token();
        while identify_token != Token::NONE {
            token_vec.push(identify_token);
            identify_token = test.identify_next_token();
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
        let mut identify_token = test.identify_next_token();
        while identify_token != Token::NONE {
            token_vec.push(identify_token);
            identify_token = test.identify_next_token();
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
        let mut identify_tok = test.identify_next_token(); 
        identify_tok = test.peek_token();
        assert_eq!(identify_tok, Token::MINUS);
        identify_tok = test.peek_token();
        assert_eq!(identify_tok, Token::MINUS);
    }

}

#[cfg(test)]
mod parser_test {
    use crate::parser::*;
   
    
    #[test]
    fn parse_basic_addition() {
        let mut test_parser = init_parser(&"computation 140 + 14.".to_string());
        let x = test_parser.parse_input();

        assert_eq!(x[0], 154);
    } 
    #[test]
    fn parse_basic_mult() {
        let mut test_parser = init_parser(&"computation 10 * 14.".to_string());
        let x = test_parser.parse_input();

        assert_eq!(x[0], 140);
    } 
    #[test]
    fn parse_harder_typshit() {
        let mut test_parser = init_parser(&"computation 10 * 14 + 100.".to_string());
        let x = test_parser.parse_input();

        assert_eq!(x[0], 240);
    } 
    #[test]
    fn parse_basic_several() {
        let mut test_parser = init_parser(&"computation 10 * 14 + 100; 10  + 12".to_string());
        let x = test_parser.parse_input();

        assert_eq!(x[0], 240);
        assert_eq!(x[1], 22)
    } 
    #[test]
    fn parse_with_variables() {
        let mut test_parser = init_parser(&"computation var x <- 10; x * 14 + 100; 10  + 12.".to_string());
        let x = test_parser.parse_input();

        assert_eq!(x[0], 240);
        assert_eq!(x[1], 22)
    } 
    #[test]
    fn parse_several_vars() {
        let mut test_parser = init_parser(&"computation 10 * 14 + 100; var x <- 10; var y <- 100; x*y.".to_string());
        let x = test_parser.parse_input();

        assert_eq!(x[0], 240);
        assert_eq!(x[1], 1000);
    } 
    #[test]
    fn parse_several_vars_2() {
        let mut test_parser = init_parser(&"computation 10 * 14 + 100; var x <- 10; var y <- 100; (x + 14)*y.".to_string());
        let x = test_parser.parse_input();

        assert_eq!(x[0], 240);
        assert_eq!(x[1], 2400);
    } 
}
