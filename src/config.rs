use std::path::PathBuf;
use glob::glob;

pub struct Config<'a> {
    pub query: &'a str,
    pub filelist: Vec<PathBuf>,
    pub start_pattern: &'a str,
    pub end_pattern: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let mut query = "";
        let mut filemask = "";
        let mut start_pattern = "";
        let mut end_pattern = "";
        let mut filelist: Vec<PathBuf> = vec![];

        for i in 1..args.len() {
            match args[i].as_str() {
                "-q" | "--query" => query = &args[i + 1],
                "-f" | "--file" => filemask = &args[i + 1],
                "-s" | "--start" => start_pattern = &args[i + 1],
                "-e" | "--end" => end_pattern = &args[i + 1],
                _ => ()
            }
        }

        if query == "" || filemask == "" {
            return Err("query/filemask are not set");
        }

        for entry in glob(filemask).expect("Failed to read file pattern") {
            match entry {
                Ok(path) => filelist.push(path),
                Err(e) => println!("{:?}", e),
            }
        }

        Ok(Config { query, filelist, start_pattern, end_pattern })
    }
}