use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

struct TextFile
{
    f: File,
    contents: String,
}

impl TextFile
{
    fn new(file_name: &String) -> TextFile
    {
        let f = file_open(file_name);
        let contents = read_text(&f);
        let text_file = TextFile
        {
            f,
            contents,
        };
        text_file
    }
}

fn file_open(filename: &String) -> File
{
    File::open(filename).expect("file can't open or not found")
}

fn read_text(mut file: &File) -> String
{
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    contents
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let query = config.query;
    let filename = config.filename;
    let text_file = TextFile::new(&filename);

    let results = if config.case_sensitive
    {
        search(&query, &text_file.contents);
    }
    else
    {
        search_case_insensitive(&query, &text_file.contents);
    };

    Ok(())
}


pub struct Config
{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl Config
{
    pub fn new(args: &[String]) -> Result<Config, &str>
    {
        let len = args.len();

        if args.len() < 3
        {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(
            Config
            {
                query,
                filename,
                case_sensitive,
            }
        )
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.contains(query)
        {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn one_result()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        // (最後の行のみ) // 私を信じて
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}