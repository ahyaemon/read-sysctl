use crate::file::read_lines;
use crate::parse::parse_line;
use crate::schema::SchemaDict;
use std::collections::HashMap;

type Dict = HashMap<String, String>;

pub fn read_sysctl(filename: &str, schema: Option<SchemaDict>) -> Result<Dict, String> {
    let lines = read_lines(filename).map_err(|e| e.to_string())?;
    let mut hashmap = HashMap::new();
    for line in lines.flatten() {
        if let Some((key, value)) = parse_line(&line, "=")? {
            hashmap.insert(key.clone(), value.clone());

            if let Some(schema) = &schema {
                if let Some(validator) = schema.get(&key) {
                    validator.validate(&value)?
                }
            }
        }
    }
    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::read_schema;

    #[test]
    fn create() {
        let filename = "resources/test/sysctl/sysctl.conf";
        let actual = read_sysctl(&filename, None);
        let expected = Ok([
            ("debug", "true"),
            ("endpoint", "localhost:3000"),
            ("log.file", "/var/log/console.log"),
        ]
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn duplicate() {
        let filename = "resources/test/sysctl/sysctl_duplicate.conf";
        let actual = read_sysctl(&filename, None);
        let expected = Ok([("key".to_string(), "value2".to_string())]
            .iter()
            .cloned()
            .collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn file_not_exists() {
        let filename = "resources/test/sysctl/xxx";
        match read_sysctl(&filename, None) {
            Ok(_) => {
                panic!()
            }
            Err(e) => {
                println!("{e}")
            }
        }
    }

    #[test]
    fn invalid_line() {
        let filename = "resources/test/sysctl/sysctl_invalid.conf";
        match read_sysctl(&filename, None) {
            Ok(_) => {
                panic!()
            }
            Err(e) => {
                println!("{e}")
            }
        }
    }

    #[test]
    fn invalid_line_with_hyphen() {
        let filename = "resources/test/sysctl/sysctl_invalid_hyphen.conf";
        let actual = read_sysctl(&filename, None);
        let expected = Ok(vec![("key1", "value1"), ("key2", "value2")]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect());
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_with_schema() {
        let schema_filename = "resources/test/sysctl/schema/schema.txt";
        let schema = read_schema(schema_filename).unwrap();

        let filename = "resources/test/sysctl/schema/sysctl_valid.conf";
        let actual = read_sysctl(&filename, Some(schema));

        let expected = Ok(HashMap::from([
            ("debug".to_string(), "true".to_string()),
            ("endpoint".to_string(), "localhost:3000".to_string()),
            ("log.file".to_string(), "/var/log/console.log".to_string()),
        ]));

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_with_schema() {
        let schema_filename = "resources/test/sysctl/schema/schema.txt";
        let schema = read_schema(schema_filename).unwrap();

        let filename = "resources/test/sysctl/schema/sysctl_invalid.conf";
        let actual = read_sysctl(&filename, Some(schema));

        match actual {
            Ok(_) => {
                panic!()
            }
            Err(e) => {
                println!("{e}")
            }
        }
    }
}
