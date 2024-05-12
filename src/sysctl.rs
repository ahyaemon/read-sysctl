use std::collections::HashMap;
use crate::{Dict};
use crate::file::read_lines;
use crate::parse::parse_line;

pub fn read_sysctl(filename: &str, schema: Option<Dict>) -> Result<Dict, String> {
    let lines = read_lines(filename).map_err(|e| e.to_string())?;
    let mut hashmap = HashMap::new();
    for line in lines.flatten() {
        if let Some((key, value)) = parse_line(&line, "=")? {
            hashmap.insert(key, value);
        }
    }
    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let filename = "resources/sysctl.conf";
        let actual = read_sysctl(&filename, None);
        let expected = Ok(
            [
                ("debug", "true"),
                ("endpoint", "localhost:3000"),
                ("log.file", "/var/log/console.log")
            ]
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect()
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn duplicate() {
        let filename = "resources/sysctl_duplicate.conf";
        let actual = read_sysctl(&filename, None);
        let expected = Ok(
            [
                ("key".to_string(), "value2".to_string()),
            ]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn file_not_exists() {
        let filename = "resources/xxx";
        match read_sysctl(&filename, None) {
            Ok(_) => { panic!() },
            Err(e) => { println!("{e}") }
        }
    }

    #[test]
    fn invalid_line() {
        let filename = "resources/sysctl_invalid.conf";
        match read_sysctl(&filename, None) {
            Ok(_) => { panic!() },
            Err(e) => { println!("{e}") }
        }
    }

    #[test]
    fn invalid_line_with_hyphen() {
        let filename = "resources/sysctl_invalid_hyphen.conf";
        let actual = read_sysctl(&filename, None);
        let expected = Ok(
            vec![
                ("key1", "value1"),
                ("key2", "value2"),
            ]
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect()
        );
        assert_eq!(actual, expected);
    }
}
