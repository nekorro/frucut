pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename })
    }
}