use std::{env, process};

use crate::config::Config;
use crate::parser::schema::read_schema;
use crate::parser::sysctl::read_sysctl;

mod config;
mod file;
mod parser;

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Failed to get command line arguments. {}", err);
        process::exit(1);
    });

    println!("filename: {}", config.filename);

    let schema = config.schema_filename.map(|schema_filename| {
        println!("schema_filename: {}", schema_filename);
        read_schema(&schema_filename).unwrap_or_else(|err| {
            println!(
                "Failed to read schema file. filename: {}. {}",
                &config.filename, err
            );
            process::exit(1);
        })
    });

    let result = read_sysctl(&config.filename, schema).unwrap_or_else(|err| {
        println!(
            "Failed to read sysctl file. filename: {}. {}",
            &config.filename, err
        );
        process::exit(1);
    });
    println!("{:?}", result);
}
