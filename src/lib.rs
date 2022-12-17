#![feature(slice_pattern)]
#![feature(slice_take)]

use std::path::{Path, PathBuf};
use std::{env, error::Error};
use std::fs;

pub struct Config {
    pub dir_path: String,
    pub query: String,
    pub replacement_text: String,
    pub ignore_case: bool,
    pub operation_replace: bool,
}

impl Config {
    pub fn build(mut args: &[String]) -> Result<Config, &'static str> {
        let num_args = args.len();
        if num_args < 3 {
            return Err("not enough arguments");
        }

        let dir_path = args[1].clone();
        let query = args[2].clone();
        let replacement_text = args[3].clone();
        
        let flags_start_index = 4;
        let (mut ignore_case, mut operation_replace) = (false, true);
        if num_args > flags_start_index {
            let flags = args.take(flags_start_index..).unwrap();

            for flag in flags {
                if flag == "ignore-case" {
                    ignore_case = true;
                }
                if flag == "dry" {
                    operation_replace = false;
                }
            }
        }

        if ignore_case == false && env::var("IGNORE_CASE").is_ok() {
            ignore_case = true;
        }

        if operation_replace && env::var("DRY").is_ok() {
            operation_replace = false;
        }

        Ok(Config { dir_path, query, replacement_text, ignore_case, operation_replace })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dir = Path::new(&config.dir_path);

    let found_paths = recursively_list_dir(dir)?;
    files_search_replace(&config, found_paths)?;

    Ok(())
}

pub fn recursively_list_dir(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();

    if false == dir.is_dir() {
        println!("Input directory path is not a directory. Correct the path?");
        return Ok(paths)
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let mut dir_paths = recursively_list_dir(&path)?;
            paths.append(&mut dir_paths);
        } else {
            paths.push(path);
        }
    }

    Ok(paths)
}

pub fn files_search_replace(config: &Config, found_paths: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut update_count = 0;

    for p in found_paths {
        let contents_result = file_read(&p);
        if contents_result.is_err() {
            continue;
        }
        let contents = contents_result?;

        if false == config.operation_replace {
            let (found_results, results) = text_search(&config, &contents)?;

            if false == found_results {
                continue;
            }

            println!("\nMatches in \"{}\":", p.display());

            for result in results {
                println!("- \"{}\"", result);
            }

            continue;
        }

        let (did_replace, new_content): (bool, String) = text_replace(&config, &contents)?;

        if false == did_replace {
            continue;
        }

        if config.operation_replace {
            file_save(&p, &new_content)?;
            update_count += 1;
        }
    }

    if config.operation_replace {
        println!("\nUpdated {} files", update_count);
    }

    Ok(())
}

pub fn file_read(file_path: &PathBuf) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn file_save(file_path: &PathBuf, new_content: &str) -> Result<(), std::io::Error> {
    fs::write(file_path, new_content)
}

pub fn text_replace(config: &Config, contents: &str) -> Result<(bool, String), Box<dyn Error>> {
    let (did_replace, new_content): (bool, String) = if config.ignore_case {
        replace_case_insensitive(&config, &contents)
    } else {
        replace_case_sensitive(&config, &contents)
    };

    Ok((did_replace, new_content))
}

pub fn replace_case_sensitive(config: &Config, contents: &str) -> (bool, String) {
    if false == contents.contains(&config.query) {
        return (false, String::new())
    }
    
    let new_contents = contents.replace(&config.query, &config.replacement_text);

    return (true, new_contents)
}

pub fn replace_case_insensitive(config: &Config, contents: &str) -> (bool, String) {
    if false == contents.to_lowercase().contains(&config.query.to_lowercase()) {
        return (false, String::new());
    }

    let match_index = contents.to_lowercase().find(&config.query.to_lowercase());

    if match_index.is_none() {
        return (false, String::new())
    }

    let (content_before_query, content_from_query) = contents.split_at(match_index.unwrap());
    let (_content_query, content_after_query) = content_from_query.split_at(config.query.len());

    let new_content = content_before_query.to_owned() + &config.replacement_text + content_after_query;

    return (true, new_content)
}

pub fn text_search<'a>(config: &Config, contents: &'a str) -> Result<(bool, Vec<&'a str>), Box<dyn Error>> {
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    let has_results = results.len() > 0;

    Ok((has_results, results))
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_matching_query = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            lines_matching_query.push(line);
        }
    }
    
    lines_matching_query
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut lines_matching_query = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            lines_matching_query.push(line);
        }
    }

    lines_matching_query
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUsT";
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

    #[test]
    fn case_sensitive_replace() {
        let config = Config {
            dir_path: "./test".to_string(),
            query: "Duct t".to_string(),
            replacement_text: "Gr".to_string(),
            ignore_case: false,
            operation_replace: true
        };

        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        let (_success, new_content) = replace_case_sensitive(&config, &contents);

        assert_eq!(
            "\
Rust:
safe, fast, productive.
Grape.",
            new_content
        );
    }

    #[test]
    fn case_insensitive_replace() {
        let config = Config {
            dir_path: "./test".to_string(),
            query: "duct t".to_string(),
            replacement_text: "Gr".to_string(),
            ignore_case: true,
            operation_replace: true
        };

        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        let (_success, new_content) = replace_case_insensitive(&config, &contents);

        assert_eq!(
            "\
Rust:
safe, fast, productive.
Grape.",
            new_content
        );
    }
}
