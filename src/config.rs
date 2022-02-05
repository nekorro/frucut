pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub start_pattern: &'a str,
    pub end_pattern: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let mut query = "";
        let mut filename = "";
        let mut start_pattern = "";
        let mut end_pattern = "";

        for i in 1..args.len() {
            match args[i].as_str() {
                "-q" | "--query" => query = &args[i + 1],
                "-f" | "--file" => filename = &args[i + 1],
                "-s" | "--start" => start_pattern = &args[i + 1],
                "-e" | "--end" => end_pattern = &args[i + 1],
                _ => ()
            }
        }

        if query == "" || filename == "" {
            return Err("query/filename are not set");
        }

        Ok(Config { query, filename, start_pattern, end_pattern })
    }
}