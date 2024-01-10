
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod trie;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        eprintln!("Err: Path to file must be provied.");
        return;
    }
    let mut trie = trie::Trie::new();

    let file = &args[1];
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            trie.insert(&line);
        }
    }


    loop {
        let mut buffer = String::new();
        println!("Search:");
        let _ = io::stdin().read_line(&mut buffer);
        let binding =  buffer.to_string();
        let word = &binding.trim();
        if word.eq(&"exit") {
            println!("exiting...");
            return;
        }
        let result = trie.search(word);
        println!("{} -> {}", word, result);


    }


}
