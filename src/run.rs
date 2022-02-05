use regex::Regex;
use crate::config::Config;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn run(config: &Config) -> io::Result<()> {
    let mut f = BufReader::new(File::open(config.filename)?);
    let re = Regex::new(config.query).unwrap();
    let re_start = Regex::new(config.start_pattern).unwrap();
    let re_end = Regex::new(config.end_pattern).unwrap();
    let mut string_buffer = String::new();
    let mut msg_bound = false;
    let mut msg_found = false;
    let mut bytes_read: usize = f.read_line(&mut string_buffer)?;

    while bytes_read > 0 {
        let last_string = &string_buffer[&string_buffer.len() - bytes_read..];

        if re_start.is_match(last_string) {
            if config.end_pattern != "" {
                msg_bound = true;
            } else if msg_found {
                print!("{}", &string_buffer[0..&string_buffer.len() - bytes_read]);
                msg_found = false;
            }
            let first_string = last_string.to_string();
            let _ = &string_buffer.clear();
            string_buffer.push_str(&first_string);
        } else if config.end_pattern != "" && re_end.is_match(last_string) {
            if msg_found {
                print!("{}", &string_buffer);
                msg_found = false;
            }
            let _ = &string_buffer.clear();
            msg_bound = false;
        } else if re.is_match(last_string) {
            if config.end_pattern != "" {
                if msg_bound {
                    msg_found = true;
                }
            } else {
                msg_found = true;
            }
        }

        bytes_read = f.read_line(&mut string_buffer)?;
    }

    if msg_found {
        print!("{}", &string_buffer);
    }

    Ok(())
}