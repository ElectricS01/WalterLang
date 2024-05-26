// main.rs
// Created 12/2/2024
// Modified 25/5/2024
// Created by ElectricS01

use home::home_dir;
use regex::Regex;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    let file_path;
    let debug;
    let shell;

    let index = args.iter().position(|n| n == &"-d".to_string());
    if index.is_some() {
        args.remove(index.expect("Could not find index of debug tag"));
        debug = true;
    } else {
        debug = false;
    }

    let index = args.iter().position(|n| n == &"-s".to_string());
    if index.is_some() {
        args.remove(index.expect("Could not find index of shell tag"));
        shell = true;
    } else {
        shell = false;
    }

    let mut vars: HashMap<String, String> = HashMap::new();

    if shell {
        println!(
            "WalterShell - WalterLang {}",
            option_env!("CARGO_PKG_VERSION").unwrap_or("Unknown Version")
        );
        let mut rl = DefaultEditor::new()?;
        let history_path = home_dir().unwrap().join(".walter_history");

        if rl.load_history(&history_path).is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline("WalterShell ");
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if rl.add_history_entry(line).is_err() {
                        println!("Could not save history");
                    }

                    if line == "exit" {
                        break;
                    } else {
                        execute(debug, line.to_string(), &mut vars);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        if rl.save_history(&history_path).is_err() {
            println!("Could not save history");
        }
        Ok(())
    } else {
        if args.len() < 2 {
            if Path::new("example.wltr").exists() {
                file_path = "example.wltr";
            } else {
                println!("Need to specify file");
                return Ok(());
            }
        } else {
            file_path = &args[1];
        }

        println!("Compiling file {}\n", file_path);

        let contents = fs::read_to_string(file_path).expect("Failed to read the file");
        execute(debug, contents, &mut vars);
        Ok(())
    }
}

fn execute(debug: bool, contents: String, vars: &mut HashMap<String, String>) {
    let multi_comment_regex = Regex::new(r"///.*?///").unwrap();
    let comment_regex = Regex::new(r"[^/]//[^/]+.*$").unwrap();

    if debug == true {
        println!("{}", contents);
    }

    let mut trimmed_contents: String = String::new();

    for line in contents.lines() {
        let line = " ".to_owned() + line;
        let line = comment_regex.replace_all(&line, "");

        let line = multi_comment_regex.replace_all(&line, "");

        let line = line.trim();
        if line != "" {
            trimmed_contents += "\n";
            trimmed_contents += line
        }
    }

    let mut function: String = String::new();
    let mut sub = false;
    let mut read_buffer: String = String::new();

    for line in trimmed_contents.lines() {
        if debug == true {
            println!("line: {}", line);
        }

        let mut line: Vec<&str> = line.split(' ').collect();

        for _i in 0..line.len() {
            if vars.contains_key(line[0]) {
                execute(
                    debug,
                    vars.get(&line[0].to_string())
                        .expect("Could not find a variable")
                        .to_string(),
                    vars,
                );
            }
            if line[0].trim().is_empty() {
                break;
            } else if ("Um" == &*line[0] || "Set" == &*line[0]) && function == String::new() {
                read_buffer = String::new();
                function = line[0].to_string();
            } else if ("Um" == &*line[0] || "Set" == &*line[0]) && function != String::new() {
                sub = true;
            }
            if "Ok" == &*line[0] && function != "" && !sub {
                if function == "Um" {
                    um(read_buffer.trim().split(' ').collect(), vars);
                } else {
                    set(read_buffer.trim().split(' ').collect(), vars);
                }
                read_buffer = String::new();
                function = String::new();
            } else if function != "" {
                if "Ok" == &*line[0] && function != "" {
                    sub = false;
                }
                if read_buffer.len() != 0 {
                    if &read_buffer[read_buffer.len() - 1..] != '\n'.to_string() {
                        read_buffer += " ";
                    }
                }
                read_buffer += line[0];
            }
            line.remove(0);
        }
        if function != "" {
            read_buffer += " \n"
        }
    }

    let read_buffer = read_buffer.trim();

    if function == "Um" {
        um(read_buffer.split(' ').collect(), vars);
    } else if function == "Set" {
        set(read_buffer.split(' ').collect(), vars);
    }
}

fn set(line: Vec<&str>, vars: &mut HashMap<String, String>) {
    let mut read_line = line.clone();
    read_line.remove(0);

    let name = read_line[0];
    read_line.remove(0);

    let mut print_line: Vec<&str> = [].to_vec();
    if read_line[0] == "to" || read_line[0] == "To" || read_line[0] == "tO" {
        read_line.remove(0);
    }
    read_line[0] = read_line[0].trim();

    for i in 0..read_line.len() {
        print_line.push(read_line[i]);
    }
    vars.insert(name.to_string(), print_line.join(" "));
    println!("Set {} to \"{}\"", name, print_line.join(" "));
    return;
}

fn um(line: Vec<&str>, vars: &mut HashMap<String, String>) {
    let mut read_line = line;
    read_line.remove(0);
    if read_line[0] == "print" {
        read_line.remove(0);
        let mut print_line: Vec<&str> = [].to_vec();
        for word in &read_line {
            if *word != "Ok" {
                let word = word.trim();
                if vars.contains_key(word) {
                    print_line.push(
                        vars.get(&word.to_string())
                            .expect("Could not find a variable"),
                    );
                } else {
                    print_line.push(word);
                }
            } else {
                break;
            }
        }
        print(print_line.to_vec());
        return;
    }
    for word in read_line {
        println!("{}", word);
    }
}

fn print(line: Vec<&str>) {
    println!("{}", line.join(" "));
}
