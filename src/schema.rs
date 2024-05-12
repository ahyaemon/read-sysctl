use std::collections::HashMap;
use crate::Dict;
use crate::file::read_lines;
use crate::parse::parse_line;

pub fn read_schema(filename: &str) -> Result<Dict, String> {
    let lines = read_lines(filename).map_err(|e| e.to_string())?;
    let mut hashmap = HashMap::new();
    for line in lines.flatten() {
        if let Some((key, value)) = parse_line(&line, "->")? {
            hashmap.insert(key, value);
        }
    }
    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let filename = "resources/schema.txt";
        let actual = read_schema(&filename);
        let expected = Ok(
            [
                ("endpoint", "string"),
                ("debug", "bool"),
                ("log.file", "string")
            ]
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect()
        );
        assert_eq!(actual, expected);
    }
}
