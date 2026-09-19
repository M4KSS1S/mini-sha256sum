use crate::hash::sha256_hex;
// use std::fmt::{Debug, Display};
use std::fs::{self, read_to_string};
use std::error::Error;
use clap::{Parser};

#[derive(Parser)]
#[command(name="hashcheck", version,about="Sha256 file hash-checker")]
pub struct Args
{
    #[arg(short = 'f',long = "file")]
    /// files to hash
    pub hash_files : Option<Vec<String>>,

    #[arg(short= 'c', long = "check")]
    /// files containes hashes to check (sha256sum format)
    pub check_file : Option<String>
}


fn parse_sum_line(str : &str) -> Option<(&str, &str)>
{
    let (hash, path) = str.split_once(' ')?;
    let path = path.trim_start_matches([' ','*']);
    Some((path, hash))
}

pub fn check_output(file: &str, fail : &mut u32) -> Result<Vec<String>, Box<dyn Error>>
{
    let file_lines = read_to_string(file)?;
    let mut last_res : Vec<String> = Vec::new();
    for lines in file_lines.lines()
    {
        if lines.is_empty()
        {
            last_res.push("skip".to_string());
            continue;
        }
        let (path, hash) = match parse_sum_line(lines)
        {
            Some(e) => e,
            None => continue,
        };
        let data = fs::read(path)?;

        let real_hash = sha256_hex(&data);
        let res = if real_hash == hash {"OK"} else {*fail+=1;"FAILED"};
        last_res.push(format!("{path}: {res}"))
    }
    Ok(last_res)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn parses_standard_line() {
        let (path, hash) = parse_sum_line("abc123  /tmp/hi").unwrap();
        assert_eq!(path, "/tmp/hi");
        assert_eq!(hash, "abc123");
    }

    #[test]
    fn strips_binary_marker() {
        let (path, _) = parse_sum_line("abc123 *file.bin").unwrap();
        assert_eq!(path, "file.bin");
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_sum_line("nothash").is_none());
    }
}