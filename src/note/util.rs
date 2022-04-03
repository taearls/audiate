pub fn uppercase_first_char(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
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
}
