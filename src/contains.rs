#[derive(Clone, Copy)]
pub struct ContainsOptions {
    pub ignore_case: Option<bool>,
    pub min_occurences: Option<usize>,
}

/// Check if the string contains the seed.
///
/// * `str` - Text to check if it contains the seed.
/// * `pat` - Text with seed
///
pub fn contains(str: &str, pat: &str) -> bool {
    contains_with_options(str, pat, None)
}

/// Check if the string contains the seed.
///
/// * `str` - Text to check if it contains the seed.
/// * `pat` - Text with seed
/// * `options` - Options
///
/// options is an object that defaults to { ignoreCase: false, minOccurrences: 1 }.
/// Options:
///     ignore_case: Ignore case when doing comparison, default false
///     min_ccurences: Minimum number of occurrences for the seed in the string. Defaults to 1.
pub fn contains_with_options(str: &str, pat: &str, options: Option<&ContainsOptions>) -> bool {
    let mut ignore_case = false;
    let mut min_occurrences = 1;
    match options {
        Some(opts) => {
            if opts.ignore_case.is_some() { ignore_case = opts.ignore_case.unwrap(); }
            if opts.min_occurences.is_some() { min_occurrences = opts.min_occurences.unwrap(); }
        }
        _ => (),
    };

    if ignore_case {
        str.to_lowercase().matches(&pat.to_lowercase()).count() >= min_occurrences
    } else {
        str.matches(pat).count() >= min_occurrences
    }
}
