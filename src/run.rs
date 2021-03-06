use crate::config::Config;
use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, BufRead, BufReader},
    path::Path,
};

pub fn run(config: &Config) -> io::Result<()> {
    for filepath in &config.filelist {
        println!("\nProcessing file: {:?}", filepath.display());

        let file = File::open(filepath)?;
        let mut f = BufReader::new(
            DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1251))
                .build(file),
        );
        let re = Regex::new(config.query).unwrap();
        let re_start = Regex::new(config.start_pattern).unwrap();
        let re_end = Regex::new(config.end_pattern).unwrap();
        let mut string_buffer = String::new();
        let mut msg_bound = false;
        let mut msg_found = false;
        let mut bytes_read: usize = f.read_line(&mut string_buffer)?;
        let filename = filepath.file_name().unwrap();
        let outfile_name = format!("cut_{}_{}.txt", config.query, filename.to_str().unwrap());

        if Path::new(&outfile_name).exists() {
            fs::remove_file(&outfile_name)
                .expect(format!("Existing outfile delete failed {}", &outfile_name).as_str());
        }

        let mut outfile = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&outfile_name)
            .unwrap();

        while bytes_read > 0 {
            let last_string = &string_buffer[&string_buffer.len() - bytes_read..];

            if re_start.is_match(last_string) {
                if config.end_pattern != "" {
                    msg_bound = true;
                } else if msg_found {
                    let _ = &outfile
                        .write_all(&string_buffer[0..&string_buffer.len() - bytes_read].as_bytes());
                    println!("Pattern match found");
                    msg_found = false;
                }
                let first_string = last_string.to_string();
                let _ = &string_buffer.clear();
                string_buffer.push_str(&first_string);
            } else if config.end_pattern != "" && re_end.is_match(last_string) {
                if msg_found {
                    let _ = &outfile.write_all(&string_buffer.as_bytes());
                    println!("Pattern match found");
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
            let _ = &outfile.write_all(&string_buffer.as_bytes());
        }

        if Path::new(&outfile_name).exists() {
            if fs::metadata(&outfile_name)?.len() == 0 {
                fs::remove_file(&outfile_name)
                    .expect(format!("File delete failed {}", &outfile_name).as_str());
            } else {
                println!(
                    "{} written {} bytes",
                    &outfile_name,
                    fs::metadata(&outfile_name)?.len()
                );
            }
        }
    }

    Ok(())
}
