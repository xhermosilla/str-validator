#[cfg(test)]
mod test {
    use str_validator::{contains, contains_with_options as contains_opt, ContainsOptions};

    #[test]
    fn test_contains() {
        assert_eq!(contains("foo", "foo"), true);
        assert_eq!(contains("foobar", "foo"), true);
        assert_eq!(contains("bazfoo", "foo"), true);
        assert_eq!(!contains("bar", "foo"), true);
        assert_eq!(!contains("fobar", "foo"), true);
    }

    #[test]
    fn test_contains_ignore_case() {
        let ignore_case = ContainsOptions { ignore_case: Some(true), min_occurences: None }; 

        assert_eq!(contains_opt("Foo", "foo", &ignore_case), true);
        assert_eq!(contains_opt("FOObar", "foo", &ignore_case), true);
        assert_eq!(contains_opt("BAZfoo", "foo", &ignore_case), true);
    }

    #[test]
    fn test_contains_min_occurrences() {
        let ignore_case = ContainsOptions { ignore_case: None, min_occurences: Some(2) }; 

        assert_eq!(contains_opt("foofoofoo", "foo", &ignore_case), true);
        assert_eq!(contains_opt("12foo124foo", "foo", &ignore_case), true);
        assert_eq!(contains_opt("fofooofoooofoooo", "foo", &ignore_case), true);
        assert_eq!(contains_opt("foo1foo", "foo", &ignore_case), true);

        assert_eq!(contains_opt("foo", "foo", &ignore_case), false);
        assert_eq!(contains_opt("foobar", "foo", &ignore_case), false);
        assert_eq!(contains_opt("Fooofoo", "foo", &ignore_case), false);
        assert_eq!(contains_opt("foofo", "foo", &ignore_case), false);
    }
}
