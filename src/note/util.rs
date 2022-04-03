pub fn uppercase_first_char(input: &str) -> String {
    let input = input.trim();
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => {
            let mut upper = f.to_uppercase().collect::<String>();
            upper.push_str(&input[1..]);
            upper
        }
    }
}

////////////////
// UNIT TESTS //
////////////////

#[cfg(test)]
mod uppercase_first_char_tests {
    use super::*;

    fn test_case(input: &str, expected: &str) {
        let actual = uppercase_first_char(input);
        assert_eq!(actual, expected.to_string());
    }

    #[test]
    fn uppercase_first_char_returns_empty_str() {
        test_case("", "");
        test_case("   ", "");
    }

    #[test]
    fn uppercase_first_char_works_for_one_char_str() {
        test_case("a", "A");
    }

    #[test]
    fn uppercase_first_char_works_for_multi_char_str() {
        test_case("apple", "Apple");
        test_case("peach", "Peach");
        test_case("orange", "Orange");
    }

    #[test]
    fn uppercase_first_char_works_for_multi_char_str_nonalphabetic() {
        test_case("1  ", "1");
        test_case("12", "12");
        test_case("4 bananas", "4 bananas");
    }
}
