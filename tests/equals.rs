#[cfg(test)]
mod test {
    use str_validator::equals;

    // it('should validate strings against an expected value', () => {
    //     test({
    //       validator: 'equals', args: ['abc'], valid: ['abc'], invalid: ['Abc', '123'],
    //     });
    //   });

    #[test]
    fn test_equals() {
        assert_eq!(equals("abc", "abc"), true);
        assert_eq!(equals("abc", "Abc"), false);
        assert_eq!(equals("abc", "123"), false);
    }
}
