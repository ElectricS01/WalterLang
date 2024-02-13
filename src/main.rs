use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return println!("Need to specify file");
    }

    let file_path = &args[1];

    println!("Compiling file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        let mut line: Vec<&str> = line.split(' ').collect();
        for word in &line {
            if let "//" = * word {
                break;
            }
            if let "Um" = *word {
               let mut readLine = line.to_vec();
               readLine.remove(0);
               for word2 in readLine {
                   println!("{}", word2);
               }
            }
            println!("With text:\n{word}");
        }
    }
}
