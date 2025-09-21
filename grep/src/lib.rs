use anyhow::Error;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,
    case_insensitive: bool,
    file_names: bool,
    entire_line: bool,
    inverted: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            line_numbers: flags.contains(&"-n"),
            case_insensitive: flags.contains(&"-i"),
            file_names: flags.contains(&"-l"),
            entire_line: flags.contains(&"-x"),
            inverted: flags.contains(&"-v"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res: Vec<String> = vec![];
    for file in files {
        let mut f = File::open(file)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        for (i, line) in contents.lines().enumerate() {
            let clean_line = flags
                .case_insensitive
                .then(|| line.to_lowercase())
                .unwrap_or_else(|| line.to_string());
            let clean_pattern = flags
                .case_insensitive
                .then(|| pattern.to_lowercase())
                .unwrap_or_else(|| pattern.to_string());
            println!("{line}");
            let mut contains = false;
            if flags.entire_line {
                if flags
                    .inverted
                    .then(|| clean_line != clean_pattern)
                    .unwrap_or_else(|| clean_line == clean_pattern)
                {
                    contains = true;
                }
            } else if flags
                .inverted
                .then(|| !clean_line.contains(&clean_pattern))
                .unwrap_or_else(|| clean_line.contains(&clean_pattern))
            {
                contains = true;
            }

            if contains {
                if flags.file_names {
                    if !res.contains(&file.to_string()) {
                        res.push(file.to_string());
                    }
                } else if flags.line_numbers {
                    if files.len() > 1 {
                        res.push(format!("{}:{}:{}", file, i + 1, line));
                    } else {
                        res.push(format!("{}:{}", i + 1, line));
                    }
                } else {
                    if files.len() > 1 {
                        res.push(format!("{}:{}", file, line));
                    } else {
                        res.push(line.to_string());
                    }
                }
            }
        }
    }

    return Ok(res);
}
