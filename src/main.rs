use std::{env, process};
use crate::config::Config;

mod config;
mod file;
mod parse;

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("コマンドライン引数の取得に失敗しました: {}", err);
        process::exit(1);
    });
    println!("{:?}", config.filename);
}
