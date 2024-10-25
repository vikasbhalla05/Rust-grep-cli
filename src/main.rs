// https://rust-cli.github.io/book/tutorial/impl-draft.html
// grep - search for a string inside a files
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let mut count=0;
    for line in content.lines() {
        count+=1;
        if line.contains(&args.pattern) {
            println!("{} {}", line,count);
        }
    }
}
