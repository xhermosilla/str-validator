#[cfg(test)]
mod test {
    use validator::{contains, contains_with_options as contains_opt, ContainsOptions};

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

        assert_eq!(contains_opt("Foo", "foo", Some(&ignore_case)), true);
        assert_eq!(contains_opt("FOObar", "foo", Some(&ignore_case)), true);
        assert_eq!(contains_opt("BAZfoo", "foo", Some(&ignore_case)), true);
    }

    #[test]
    fn test_contains_min_occurrences() {
        let ignore_case = ContainsOptions { ignore_case: None, min_occurences: Some(2) }; 

        assert_eq!(contains_opt("foofoofoo", "foo", Some(&ignore_case)), true);
        assert_eq!(contains_opt("12foo124foo", "foo", Some(&ignore_case)), true);
        assert_eq!(contains_opt("fofooofoooofoooo", "foo", Some(&ignore_case)), true);
        assert_eq!(contains_opt("foo1foo", "foo", Some(&ignore_case)), true);

        assert_eq!(contains_opt("foo", "foo", Some(&ignore_case)), false);
        assert_eq!(contains_opt("foobar", "foo", Some(&ignore_case)), false);
        assert_eq!(contains_opt("Fooofoo", "foo", Some(&ignore_case)), false);
        assert_eq!(contains_opt("foofo", "foo", Some(&ignore_case)), false);
    }
}
