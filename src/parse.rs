pub fn parse_line(mut line: &str, sep: &str) -> Result<Option<(String, String)>, String> {
    if line.is_empty() {
        return Ok(None)
    }

    let mut is_ignore_invalid_line = false;
    if let Some(first_char) = line.chars().next() {
        if first_char == '-' {
            is_ignore_invalid_line = true;
            line = &line[1..].trim();
        }
    }

    if line.starts_with("#") {
        return Ok(None)
    }

    if line.starts_with(";") {
        return Ok(None)
    }

    let sp: Vec<&str> = line.splitn(2, sep).collect();
    if sp.len() != 2 {
        return if is_ignore_invalid_line {
            Ok(None)
        } else {
            Err(format!("不正な行が検知されました。[{}]", line))
        }
    }

    let first = sp[0].trim();
    if first.is_empty() {
        return if is_ignore_invalid_line {
            Ok(None)
        } else {
            Err(format!("不正な行が検知されました。[{}]", line))
        }
    }

    let second = sp[1].trim();
    Ok(Some((first.to_string(), second.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_comment_hash() {
        assert_eq!(
            parse_line("#comment", "="),
            Ok(None)
        );
    }

    #[test]
    fn parse_line_comment_semicolon() {
        assert_eq!(
            parse_line(";comment", "="),
            Ok(None)
        );
    }

    #[test]
    fn parse_line_empty() {
        assert_eq!(
            parse_line("", "="),
            Ok(None)
        );
    }

    #[test]
    fn parse_line_invalid() {
        assert_eq!(
            parse_line("xxx", "="),
            Err("不正な行が検知されました。[xxx]".to_string())
        );
    }

    #[test]
    fn parse_line_invalid_with_hyphen() {
        assert_eq!(
            parse_line("- xxx", "="),
            Ok(None)
        );
    }

    #[test]
    fn parse_line_invalid_key_is_empty() {
        assert_eq!(
            parse_line("=xxx", "="),
            Err("不正な行が検知されました。[=xxx]".to_string())
        );
    }

    #[test]
    fn parse_line_valid() {
        assert_eq!(
            parse_line("endpoint = localhost:3000", "="),
            Ok(Some(("endpoint".to_string(), "localhost:3000".to_string())))
        );
    }

    #[test]
    fn parse_line_valid_with_hyphen() {
        assert_eq!(
            parse_line("- endpoint = localhost:3000", "="),
            Ok(Some(("endpoint".to_string(), "localhost:3000".to_string())))
        );
    }

    #[test]
    fn parse_line_valid_with_space() {
        assert_eq!(
            parse_line("  endpoint  =  localhost:3000   ", "="),
            Ok(Some(("endpoint".to_string(), "localhost:3000".to_string())))
        );
    }

    #[test]
    fn parse_line_valid_with_multiple_equals() {
        assert_eq!(
            parse_line("A = B = C", "="),
            Ok(Some(("A".to_string(), "B = C".to_string())))
        );
    }

    #[test]
    fn separate_with_arrow() {
        assert_eq!(
            parse_line("A -> B", "->"),
            Ok(Some(("A".to_string(), "B".to_string())))
        );
    }
}
