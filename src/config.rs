use std::env;

pub struct Config {
    pub filename: String,
    pub schema_filename: Option<String>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Failed to get the file name to read. Please specify a file name."),
        };

        let schema_filename = args.next();

        Ok(Config {
            filename,
            schema_filename,
        })
    }
}
