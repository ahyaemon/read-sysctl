use std::env;

pub struct Config {
    pub filename: String,
    pub schema_filename: Option<String>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename =
            match args.next() {
                Some(arg) => arg,
                None => return Err(
                    "読み取り対象ファイル名の取得に失敗しました。ファイル名を指定してください。",
                ),
            };

        let schema_filename = args.next();

        Ok(Config {
            filename,
            schema_filename,
        })
    }
}
