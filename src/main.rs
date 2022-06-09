#![allow(non_snake_case)]
#![allow(unused_variables)]

use error_chain::error_chain;
use std::fs;
use regex::Regex;
use std::str;
use fancy_regex::Regex as OtherRegex;
use std::path::Path;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Gathering parameters... Please wait.");
    if Path::new("./output.txt").exists() {
        fs::remove_file("./output.txt")?;
    }
    fs::create_dir_all("./analysis")?;
    let domain = std::env::args().nth(1).expect("Missing URL");


    let u = r"https://web.archive.org/cdx/search/cdx?url=".to_owned() + domain.as_str() + "/*&output=txt&fl=original&collapse=urlkey&page=/";
    let response = reqwest::get(u).await?;
    let response = response.text().await?;
    let re = Regex::new(r"^.?^.*=").unwrap();
    let re2 = Regex::new(r".jpg|.png.|.js|.PNG|.JPG|.pdf").unwrap();
    for line in response.lines() {
        let lines = line.to_string();
        let replace = OtherRegex::new(r"\=(.*)").unwrap();
        let website = replace.replace_all(&lines, "=ParamHunter\">Bugbounty").to_string();
        if re.is_match(&website) {
            if !re2.is_match(&website) {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open("./output.txt")
                    .unwrap();
                write!(file, "{}\n", website)?;
            }
        }
    }
    if Path::new("./output.txt").exists() == false {
        println!("No URLs Found!")
    }
        let file = File::open("./output.txt").expect("file error");
        let reader = BufReader::new(file);
        let lines: BTreeSet<_> = reader
            .lines()
            .map(|l| l.expect("Couldn't read a line"))
            .collect();
        let mut file = File::create("./output.txt").expect("file error");
        for line in lines {
            file.write_all(line.as_bytes())
                .expect("Couldn't write to file");
            file.write_all(b"\n").expect("Couldn't write to file");
            println!("{}", line);
        }
    Ok(())
}
