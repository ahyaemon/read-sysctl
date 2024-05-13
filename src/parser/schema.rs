use std::collections::HashMap;

use crate::file::read_lines;
use crate::parser::parse::parse_line;
use crate::parser::validator::Validator;

pub type SchemaDict = HashMap<String, Validator>;

pub fn read_schema(filename: &str) -> Result<SchemaDict, String> {
    let lines = read_lines(filename).map_err(|e| e.to_string())?;
    let mut hashmap = HashMap::new();
    for line in lines.map_while(Result::ok) {
        if let Some((key, value)) = parse_line(&line, "->")? {
            let validator = Validator::from(&value)?;
            hashmap.insert(key, validator);
        }
    }
    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let filename = "resources/test/schema/schema.txt";
        let actual = read_schema(filename);
        let expected = Ok(HashMap::from([
            ("endpoint".to_string(), Validator::String),
            ("debug".to_string(), Validator::Bool),
            ("log.file".to_string(), Validator::String),
        ]));
        assert_eq!(actual, expected);
    }
}
