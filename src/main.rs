use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read, Write, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let ignore_set = load_ignore_list(".concatignore")?;
    let start_dir = Path::new(".");
    concat_dir(start_dir, &ignore_set)?;
    Ok(())
}

fn load_ignore_list<P: AsRef<Path>>(ignore_file: P) -> io::Result<HashSet<String>> {
    let mut ignore_set = HashSet::new();
    if let Ok(file) = File::open(ignore_file) {
        for line in BufReader::new(file).lines() {
            let line = line?;
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                ignore_set.insert(trimmed.to_string());
            }
        }
    }
    Ok(ignore_set)
}

fn is_ignored(path: &Path, ignore_set: &HashSet<String>) -> bool {
    for ignore in ignore_set {
        let ignore_path = Path::new(ignore);
        if path.ends_with(ignore_path) || path.file_name().map_or(false, |n| n == OsStr::new(ignore)) {
            return true;
        }
    }
    false
}

fn concat_dir(dir: &Path, ignore_set: &HashSet<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if is_ignored(&path, ignore_set) {
            continue;
        }
        if path.is_dir() {
            concat_dir(&path, ignore_set)?;
        } else if path.is_file() {
            if let Some(filename) = path.to_str() {
                println!("{{{{ BEGIN {} }}}}", filename);
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                print!("{}", contents);
                println!("\n{{{{ END {} }}}}\n", filename);
            }
        }
    }
    Ok(())
}
