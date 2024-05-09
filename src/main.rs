use std::{env, process};
use std::collections::HashMap;
use std::io::Error;
use crate::config::Config;
use crate::file::read_lines;
use crate::parse::parse_line;

mod config;
mod file;
mod parse;

fn main() {
    let args = env::args();
    let config = Config::new(args)
        .unwrap_or_else(|err| {
            println!("コマンドライン引数の取得に失敗しました: {}", err);
            process::exit(1);
        });
    let result = create_hashmap(&config.filename)
        .unwrap_or_else(|err| {
            println!("ハッシュマップの作成に失敗しました: {}, filename: {}", err, &config.filename);
            process::exit(1);
        });
    println!("{:?}", result);
}

fn create_hashmap(filename: &str) -> Result<HashMap<String, String>, Error> {
    let hashmap = read_lines(filename)?
        .flatten()
        .filter_map(|line| parse_line(&line).unwrap() )
        .collect();
    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let filename = "resources/sysctl.conf";
        let actual = create_hashmap(&filename).unwrap();
        let expected = vec![
            ("debug", "true"),
            ("endpoint", "localhost:3000"),
            ("log.file", "/var/log/console.log")
        ]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn duplicate() {
        let filename = "resources/sysctl_duplicate.conf";
        let actual = create_hashmap(&filename).unwrap();
        let expected = [
            ("key".to_string(), "value2".to_string())
        ]
            .iter()
            .cloned()
            .collect();
        assert_eq!(actual, expected)
    }

    #[test]
    #[should_panic]
    fn file_not_exists() {
        let filename = "resources/xxx";
        create_hashmap(&filename).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_line() {
        let filename = "resources/sysctl_invalid.conf";
        create_hashmap(&filename).unwrap();
    }

    #[test]
    fn invalid_line_with_hyphen() {
        let filename = "resources/sysctl.conf";
        let actual = create_hashmap(&filename).unwrap();
        let expected = vec![
            ("key1", "value1"),
            ("key1", "value2"),
        ]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        assert_eq!(actual, expected);
    }
}
