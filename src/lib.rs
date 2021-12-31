use std::env;
use std::error::Error;
use std::fs;

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
            let line_content = line.split(&params.query).collect::<Vec<&str>>();

            for (i, item) in line_content.iter().enumerate() {
                if item.is_empty() {
                    continue;
                }

                if i > 0 && i < line_content.len() {
                    red!("{}", &params.query);
                }

                print!("{}", item);
            }

            print!("\n");
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
