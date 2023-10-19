use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use walkdir::WalkDir;
use ignore::gitignore::Gitignore;

pub fn count_lines_in_directory(directory: &str, no_git: bool, no_target: bool) -> usize {
    let gitignore = Gitignore::empty();
    let mut total_lines = 0;

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        let is_git_related = gitignore.matched(path, false).is_ignore();

        if (!no_git || !is_git_related) && (!no_target || !path.starts_with(format!("{}/target/", directory))) {
            let lines = count_lines_in_file(path.to_str().unwrap());
            total_lines += lines;
        }
    }

    total_lines
}

fn count_lines_in_file(file_path: &str) -> usize {
    let file = fs::File::open(file_path).expect("Failed to open the file");
    let reader = BufReader::new(file);

    reader.lines().count()
}
