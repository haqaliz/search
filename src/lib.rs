use std::env;
use std::error::Error;
use std::fs;
use std::collections::HashMap;

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
    for file in params.files {
        green_ln!("{}:", file);

        let contents = fs::read_to_string(file)?;
        let results = search(&params.query, &contents);

        for line in results {
            let line_content = line.1.split(&params.query).collect::<Vec<&str>>();
            let mut highlighted_before: bool = false;

            blue!("{}: ", line.0);

            for (i, item) in line_content.iter().enumerate() {
                if item.is_empty() {
                    red!("{}", &params.query);
                    highlighted_before = true;
                    continue;
                }

                if highlighted_before == false && i > 0 && i < line_content.len() {
                    red!("{}", &params.query);
                }

                print!("{}", item);
            }

            print!("\n");
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> HashMap<String, String> {
    let mut lines = HashMap::new();

    for (i, line) in contents.lines().enumerate() {
        if !line.contains(query) {
            continue;
        }

        lines.insert(
            (i + 1).to_string(),
            line.to_string(),
        );
    }

    lines
}
