use checksums::hash_file;
use checksums::Algorithm;
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    #[arg(short, long, value_enum, default_value = "MD5")]
    algo: Algorithm,
}

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();

    let Args { path, algo } = Args::parse();

    println!("Checking duplicates in folder: {}", path.display());

    println!("Hash algorithm : {:?}", algo);

    let (mut files, counter) = visit_dirs(&path, algo)?;

    deduplicate_files(&mut files);

    println!("{:#?}", files);

    let duration = start.elapsed();

    println!("Processed {} files in {:.2?}", counter, duration);

    Ok(())
}

fn visit_dirs(
    dir: &Path,
    hash_algorithm: Algorithm,
) -> Result<(HashMap<String, Vec<PathBuf>>, u32), std::io::Error> {
    let mut counter = 0;
    let mut map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    println!("Processing dir {}", dir.display());

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            //map.append(&mut visit_dirs(&path)?);
            let mut sub_dir = visit_dirs(&path, hash_algorithm)?;
            merge_maps(&mut map, &mut sub_dir.0);
            counter += sub_dir.1;
        } else if is_image(&path) {
            let hash = hash_file(&path, hash_algorithm);

            let files_for_hash = map.entry(hash).or_insert(Vec::new());

            files_for_hash.push(path);
            counter += 1;
        } else {
            println!("Ignoring file {}", path.display());
        }
    }
    Ok((map, counter))
}

fn is_image(file: &PathBuf) -> bool {
    file.is_file()
        && match file.extension() {
            None => false,
            Some(os_str) => match os_str.to_str() {
                None => false,
                Some(str) => match str.to_lowercase().as_str() {
                    "png" => true,
                    "jpeg" => true,
                    "jpg" => true,
                    "txt" => true,
                    _ => false,
                },
            },
        }
}

fn merge_maps(
    source: &mut HashMap<String, Vec<PathBuf>>,
    other: &mut HashMap<String, Vec<PathBuf>>,
) {
    for (key, value) in other.iter_mut() {
        let files = source.entry(key.to_string()).or_insert(Vec::new());
        files.append(value);
    }
}

fn deduplicate_files(files: &mut HashMap<String, Vec<PathBuf>>) {
    files.retain(|_, v| v.len() > 1);
}
