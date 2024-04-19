mod tokenizer;
mod parser;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod other_tests {

    use super::*;
    // #[test]
    fn its_me_mario() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
