use std::env;
use std::error::Error;
use std::fs;
use regex::Regex;

#[macro_use]
extern crate colour;

pub struct Params {
    pub query: String,
    pub files: Vec<String>,
}

impl Params {
    pub fn new(mut args: env::Args) -> Result<Params, &'static str> {
        args.next();

        let mut files: Vec<String> = Vec::new();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        loop {
            if let Some(next) = args.next() {
                files.push(next);
                continue;
            }

            break;
        }

        if files.len() == 0 {
            return Err("Didn't get a file name");
        }

        Ok(Params {
            query,
            files: files.to_vec(),
        })
    }
}

pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let mut files = params.files;

    files.sort();

    for file in files {
        magenta_ln!("{}", file);

        let contents = fs::read_to_string(file)?;
        let re = Regex::new(&params.query).unwrap();
        let results = search(&re, &contents);

        for line in results {
            let line_content = &re.split(&line.1).collect::<Vec<&str>>();
            let line_captures = &re.find_iter(&line.1)
                .map(|i| i.as_str())
                .collect::<Vec<&str>>();
            let mut highlighted_before: bool = false;

            green!("{}: ", line.0);

            for (i, item) in line_content.iter().enumerate() {
                let ii = if i == 0 { 0 } else { i - 1 };
                if item.is_empty() {
                    red!("{}", &line_captures[ii]);
                    highlighted_before = true;
                    continue;
                }

                if highlighted_before == false
                    && i > 0
                    && i < line_content.len() {
                    red!("{}", &line_captures[ii]);
                }

                print!("{}", item);
            }

            print!("\n");
        }
    }

    Ok(())
}

pub fn search<'a>(re: &Regex, contents: &'a str) -> Vec<(usize, String)> {
    let mut lines = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if !re.is_match(line) {
            continue;
        }

        lines.push((
            i + 1,
            line.to_string(),
        ));

    }

    lines.sort_by_key(|i| i.0);

    lines
}
