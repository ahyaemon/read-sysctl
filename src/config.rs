use std::env;

pub struct Config {
    pub filename: String,
}

impl Config {

    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("一つ目の引数の取得に失敗しました。ファイル名を指定してください。"),
        };

        Ok(Config { filename })
    }
}
