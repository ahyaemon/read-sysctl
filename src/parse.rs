pub fn parse_line(line: &str) -> Option<(&str, &str)> {
    if line.is_empty() {
        return None
    }

    if line.starts_with("#") {
        return None
    }

    if line.starts_with(";") {
        return None
    }

    let sp: Vec<&str> = line.splitn(2, "=").collect();
    if sp.len() != 2 {
        return None
    }

    let first = sp[0].trim();
    if first.is_empty() {
        return None
    }

    let second = sp[1].trim();
    Some((first, second))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_comment_hash() {
        assert_eq!(
            parse_line("#comment"),
            None
        );
    }

    #[test]
    fn parse_line_comment_semicolon() {
        assert_eq!(
            parse_line(";comment"),
            None
        );
    }

    #[test]
    fn parse_line_empty() {
        assert_eq!(
            parse_line(""),
            None
        );
    }

    #[test]
    fn parse_line_invalid() {
        assert_eq!(
            parse_line("xxx"),
            None
        );
    }

    #[test]
    fn parse_line_invalid_key_is_empty() {
        assert_eq!(
            parse_line("=xxx"),
            None
        );
    }

    #[test]
    fn parse_line_valid() {
        assert_eq!(
            parse_line("endpoint = localhost:3000"),
            Some(("endpoint", "localhost:3000"))
        );
    }

    #[test]
    fn parse_line_valid_with_space() {
        assert_eq!(
            parse_line("  endpoint  =  localhost:3000   "),
            Some(("endpoint", "localhost:3000"))
        );
    }

    #[test]
    fn parse_line_valid_with_multiple_equals() {
        assert_eq!(
            parse_line("A = B = C"),
            Some(("A", "B = C"))
        );
    }
}
