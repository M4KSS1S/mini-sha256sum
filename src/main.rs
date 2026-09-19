// use std::env;
use hashcheck::check::*;
use hashcheck::hash::sha256_hex;
use std::{fs::read};
use std::process::exit;
use clap::{Parser};


fn main()
{
    let parser = Args::parse();
    if let Some(hash_check) = parser.check_file
    {
        let mut fails = 0;
        let err = check_output(&hash_check, &mut fails);
        match err {
            Ok(s) => {
                for elem in &s
                {
                    if *elem == "skip"
                    {
                        continue;
                    }
                    else{
                        println!("{elem}");
                    }
                }
            },
            Err(_e) => {
                eprintln!("{}: FAILED: {_e}",hash_check);
                exit (1);
            }
        }
        if fails > 0
        {
            eprintln!("hashcheck: WARNING: {} computed checksum did NOT match", fails);
            exit(1);
        }
    };
    if let Some(hash_files) = parser.hash_files
    {
        for path in hash_files
        {
            match read(&path)
            {
                Ok(s) => println!("{}  {}",sha256_hex(&s), path),
                Err(e) => {
                    eprintln!("hashcheck: {path} :{e}");
                    exit(1);
                }
            }
        }
    };
}

