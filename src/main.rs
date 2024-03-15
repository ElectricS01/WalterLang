// main.rs
// Created 12/2/2024
// Modified 12/3/2024
// Created by ElectricS01

use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path;

    if args.len() < 2 {
        if Path::new("example.wltr").exists() {
            file_path = "example.wltr";
        } else {
            return println!("Need to specify file");
        }
    } else {
        file_path = &args[1];
    }

    println!("Compiling file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Failed to read the file");

    let multi_comment_regex = Regex::new(r"///.*?///").unwrap();
    let comment_regex = Regex::new(r"[^\/]\/\/[^\/]+.*$").unwrap();

    let mut vars: HashMap<String, String> = HashMap::new();

    println!("{}", contents);

    for line in contents.lines() {
        let line  = " ".to_owned() + line;
        let line = comment_regex.replace_all(&line, "");

        let line = multi_comment_regex.replace_all(&line, "");

        let line = line.trim();

        println!("line: {}", line);
        let line: Vec<&str> = line.split(' ').collect();
        for word in &line {
            if word.trim().is_empty() {
                break;
            } else if "Um" == *word {
                um(line.to_vec(), &mut vars);
                break;
            } else if "Set" == *word {
                set(line.to_vec(), &mut vars);
                break;
            }
            println!("With text:\n{word}");
        }
    }
}

fn set (line: Vec<&str>, vars: &mut HashMap<String, String>) {
    let mut read_line = line;
    read_line.remove(0);

    let name = read_line[0];
    read_line.remove(0);

    let mut print_line: Vec<&str> = [].to_vec();
    for word in &read_line {
        if *word != "Ok" {
            print_line.push(word);
        } else {
            break
        }
    }
    vars.insert(
        name.to_string(),
        print_line.join(" ")
    );
    println!("Set {} to \"{}\"", name, print_line.join(" "));
    return;
}

fn um (line: Vec<&str>, vars: &mut HashMap<String, String>) {
    let mut read_line = line;
    read_line.remove(0);
    if read_line[0] == "print" {
        read_line.remove(0);
        let mut print_line: Vec<&str> = [].to_vec();
        for word in &read_line {
            if *word != "Ok" {
                if vars.contains_key(*word) {
                    print_line.push(vars.get(&word.to_string()).expect("Could not find a variable"));
                } else {
                    print_line.push(word);
                }
            } else {
                break
            }
        }
        print(print_line.to_vec()); 
        return;
    }
    for word in read_line {
        println!("{}", word);
    }
}

fn print ( line: Vec<&str>) {
    println!("{}", line.join(" "));
}

