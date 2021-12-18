
/// Check if the string matches the comparison.
///
/// * `str` - Text.
/// * `comparison` - Text to compare
/// 
/// # Examples
/// 
/// ```
/// use str_validator::equals;
/// 
/// println!("Equals: {}", equals("abc", "abc"));
/// ```
pub fn equals(str: &str, comparison: &str) -> bool {
    str == comparison
}
