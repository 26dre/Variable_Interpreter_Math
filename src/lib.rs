mod tokenizer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
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
