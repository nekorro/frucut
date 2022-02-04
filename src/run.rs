use crate::config::Config;
use regex::Regex;
use std::{
    io::{self, BufRead, BufReader},
    fs::File,
};

pub fn run(config: Config) -> io::Result<()> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = BufReader::new(File::open(config.filename)?);
    let re = Regex::new(config.query).unwrap();
    let re_start = Regex::new("--START").unwrap();
    let re_end = Regex::new("--END").unwrap();
    let mut string_buffer = String::new();
    let mut msg_bound = false;
    let mut msg_found = false;
    let mut bytes_read: usize = f.read_line(&mut string_buffer)?;

    while bytes_read > 0 {
        let last_string = &string_buffer[&string_buffer.len() - bytes_read..];
        if re_start.is_match(last_string) {
            msg_bound = true;
        } else if re_end.is_match(last_string) {
            msg_bound = false;
        } else if re.is_match(last_string) {
            if msg_bound {
                msg_found = true;
            }
        }

        if !msg_bound {
            if msg_found {
                print!("{}", &string_buffer);
                msg_found = false;
            }
            let _ = &string_buffer.clear();
        }

        bytes_read = f.read_line(&mut string_buffer)?;
    }

    Ok(())
}