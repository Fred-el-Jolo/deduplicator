use clap::Parser;
use std::fs::{self, ReadDir, FileType};
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{:#?}", args);

    println!("Hello {}!", args.path.display());

    let dirs = fs::read_dir(args.path).expect("Error navigating to path");

    for dir in dirs {
        let dir_content = dir.unwrap();
        println!("{:#?}", dir_content);
        println!("{}", dir_content.file_name().into_string().unwrap());
        println!("{}", match dir_content.file_type().unwrap().is_file() {
            true => "File !!!",
            false => "Folder !!!",
        });
    }
}
