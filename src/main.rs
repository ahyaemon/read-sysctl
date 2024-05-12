use std::{env, process};
use std::collections::HashMap;
use crate::config::Config;
use crate::file::read_lines;
use crate::parse::parse_line;
use crate::sysctl::read_sysctl;

mod config;
mod file;
mod parse;
mod sysctl;

type Dict = HashMap<String, String>;

fn main() {
    let args = env::args();
    let config = Config::new(args)
        .unwrap_or_else(|err| {
            println!("コマンドライン引数の取得に失敗しました: {}", err);
            process::exit(1);
        });

    println!("filename: {}", config.filename);

    let schema = config.schema_filename.map(|schema_filename| {
        println!("schema_filename: {}", schema_filename);
        create_schema(&schema_filename).unwrap_or_else(|err| {
            println!("Failed to read schema file: {} filename: {}", err, &config.filename);
            process::exit(1);
        })
    });

    println!("{:?}", schema.unwrap());

    let result = read_sysctl(&config.filename)
        .unwrap_or_else(|err| {
            println!("ハッシュマップの作成に失敗しました: {} filename: {}", err, &config.filename);
            process::exit(1);
        });
    println!("{:?}", result);
}

fn create_schema(filename: &str) -> Result<Dict, String> {
    let lines = read_lines(filename).map_err(|e| e.to_string())?;
    let mut hashmap = HashMap::new();
    for line in lines.flatten() {
        if let Some((key, value)) = parse_line(&line, "->")? {
            hashmap.insert(key, value);
        }
    }
    Ok(hashmap)
}
