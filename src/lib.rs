use std::path::{Path, PathBuf};
use std::{ env, error::Error };
use std::fs;

pub struct Config {
    pub dir_path: String,
    pub query: String,
    pub replacement_text: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let num_args = args.len();
        if num_args < 3 {
            return Err("not enough arguments");
        }

        let dir_path = args[1].clone();
        let query = args[2].clone();
        let replacement_text = args[3].clone();
        
        let ignore_case: bool = if num_args == 5 {
            args[4].contains("ignore-case")
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config { dir_path, query, replacement_text, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dir = Path::new(&config.dir_path);

    let found_paths = recursively_list_dir(dir)?;
    search_files(&config, found_paths)?;

    Ok(())
}

pub fn search_files(config: &Config, found_paths: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut search_results: Vec<PathBuf> = Vec::new();
    
    for p in found_paths {      
        if true == search_file(&config, &p).unwrap_or(false) {
            search_results.push(p);
        }
    }

    // println!("# search results {:?}", search_results.len());

    // for sr in search_results {
    //     println!("file containing query: {:?}", sr.display());
    // }

    Ok(())
}

pub fn search_file(config: &Config, file_path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let contents_result = fs::read_to_string(file_path);
    if contents_result.is_err() {
        // file is probably not UTF-8
        return Ok(false)
    };

    let contents = contents_result?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    let has_results = results.len() > 0;

    if has_results {
        let (did_replace, new_content): (bool, String) = if config.ignore_case {
            replace_case_insensitive(&config, &contents)
        } else {
            replace(&config, &contents)
        };

        if did_replace {
            fs::write(file_path, new_content)?;
        }

        println!("\nMatch in file {:?}:", file_path.display());

        for line in results {
            println!("{line}");
        }

        println!("---");
    }

    Ok(has_results)
}

pub fn recursively_list_dir(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    
    if dir.is_dir() {
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
    } else {
        println!("Provided directory path is not a directory");
    }

    Ok(paths)
}

pub fn replace(config: &Config, contents: &str) -> (bool, String) {
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_matching_query = Vec::new();

    // let mut updated_lines: Vec<String> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            // let updated_line = line.replace(&config.query, &config.replacement_text);
            // updated_lines.push(updated_line.clone());
            lines_matching_query.push(line);
        // } else {
        //     updated_lines.push(line.to_owned());
        }
    }
    
    lines_matching_query
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            ignore_case: false
        };

        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        let (_success, new_content) = replace(&config, &contents);

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
            ignore_case: true
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
