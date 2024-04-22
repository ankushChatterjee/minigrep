use std::{env::args, fs};
use regex::{Regex, RegexBuilder};

use clap::Parser;
use error::GrepError;

mod error;

#[derive(Parser, Debug, Clone)]
#[command(author="Ankush", version="0.1.0", about = None, long_about = None)]
struct Args {
    query: String,
    input_file: String,
    #[arg(short, long)]
    recursive: bool,
    #[arg(short='v', long)]
    invert_match: bool,
    #[arg(short='i', long)]
    ignore_case: bool,
}

fn main() -> Result<(), GrepError> {
    let agrs = Args::parse();
    let query = agrs.clone().query;
    let input_file = agrs.clone().input_file;
    let res: String;
    if agrs.recursive {
        res = process_query_dir(&query, &input_file, agrs.invert_match, agrs.ignore_case)?;
    } else {
        res = process_query_file(&query, &input_file, agrs.invert_match, agrs.ignore_case)?;
    }
    print!("{}", res);
    Ok(()) 
}

fn process_query_dir(query: &str, dir_path: &str, invert: bool, ignore_case: bool) -> Result<String, GrepError> {
    let mut result = String::new();
    let mut read_path = dir_path;
    if read_path == "*" {
        read_path = ".";
    }
    let dir_content = match fs::read_dir(read_path) {
        Ok(content) => content,
        Err(e) => return Err(GrepError{message:e.to_string()}),
    };

    for entry in dir_content {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return Err(GrepError{message:e.to_string()}),
        };
        let file_name = entry.file_name();
        let file_name = match file_name.to_str() {
            Some(name) => name,
            None => {
                continue;
            }
        };
        let file_path = format!("{}/{}", read_path, file_name);
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(e) => return Err(GrepError{message:e.to_string()}),
        };
        if file_type.is_dir() {
            let res = process_query_dir(query, &file_path, invert, ignore_case)?;
            result.push_str(&res);
        } else {
            let res = process_query_file(query, &file_path, invert, ignore_case);
            if res.is_err() {
                continue;
            }
            for line in res.unwrap().lines() {
                result.push_str(&file_path);
                result.push_str(" : "); 
                result.push_str(line);
                result.push_str("\n");
            }
        }
    }
    return Ok(result);
}

fn process_query_file(query: &str, file_name: &str, invert: bool, ignore_case: bool) -> Result<String, GrepError> {
    let file_content_result = fs::read_to_string(file_name);
    let file_content = match file_content_result {
        Ok(content) => content,
        Err(e) => return Err(GrepError{message:e.to_string()}),
    };

    if query.len() == 0 || file_content.len() == 0 {
        return Ok(file_content);
    }

    let mut result = String::new();
    let lines = file_content.lines();
    for line in lines {
        let query_regex = match RegexBuilder::new(query).case_insensitive(ignore_case).build() {
            Ok(regex) => regex,
            Err(e) => return Err(GrepError{message:e.to_string()}),
        };

        match query_regex.find(&line)  {
            Some(_) => {
                if !invert {
                    result.push_str(line);
                    result.push_str("\n");
                }
            },
            None => {
                if invert {
                    result.push_str(line);
                    result.push_str("\n");
                }
            },
        }
    }
    Ok(result)
}