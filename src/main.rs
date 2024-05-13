use std::{env, process};

use crate::config::Config;
use crate::schema::read_schema;
use crate::sysctl::read_sysctl;

mod config;
mod file;
mod parse;
mod schema;
mod sysctl;
mod validator;

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("コマンドライン引数の取得に失敗しました: {}", err);
        process::exit(1);
    });

    println!("filename: {}", config.filename);

    let schema = config.schema_filename.map(|schema_filename| {
        println!("schema_filename: {}", schema_filename);
        read_schema(&schema_filename).unwrap_or_else(|err| {
            println!(
                "Failed to read schema file: {} filename: {}",
                err, &config.filename
            );
            process::exit(1);
        })
    });

    let result = read_sysctl(&config.filename, schema).unwrap_or_else(|err| {
        println!(
            "ハッシュマップの作成に失敗しました: {} filename: {}",
            err, &config.filename
        );
        process::exit(1);
    });
    println!("{:?}", result);
}
