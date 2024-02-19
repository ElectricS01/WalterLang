use std::env;
use std::fs;
use std::path::Path;
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
        .expect("Should have been able to read the file");
   
    let multi_comment_regex = Regex::new(r"///.*?///").unwrap();
    let comment_regex = Regex::new(r"[^\/]\/\/[^\/]+.*$").unwrap();

    for line in contents.lines() {
        let line  = " ".to_owned() + line;
        let line = comment_regex.replace_all(&line, "");

        let line = multi_comment_regex.replace_all(&line, "");

        let line = line.trim();

        let line: Vec<&str> = line.split(' ').collect();
        for word in &line {
            if word.trim().is_empty() {
                break;
            }
            if "Um" == *word {
                um(line.to_vec());
                break;
            }
            println!("With text:\n{word}");
        }
    }
}

fn um (line: Vec<&str>) {
    let mut read_line = line;
    read_line.remove(0);
    if read_line[0] == "print" {
        read_line.remove(0);
        let mut print_line: Vec<&str> = [].to_vec();
        for word in &read_line {
            if *word != "Ok" {
                print_line.push(word);
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

