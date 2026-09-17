/// Splits a comma-separated string into trimmed, non-empty parts.
pub fn split_comma(s: &str) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_comma_trims_parts() {
        assert_eq!(split_comma("a, b ,c"), vec!["a", "b", "c"]);
    }

    #[test]
    fn split_comma_skips_empty_parts() {
        assert_eq!(split_comma("a,,b,"), vec!["a", "b"]);
    }

    #[test]
    fn split_comma_returns_empty_for_blank() {
        assert!(split_comma("").is_empty());
    }
}
