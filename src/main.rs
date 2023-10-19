mod count_lines;
mod ui;

fn main() {
    println!("Lines Tally!\nKnow the number of lines of code you've written in your project");

    let directory = ui::get_directory_path();
    let no_git = !ui::ask_exclude_git();
    let no_target = !ui::ask_exclude_target();

    println!("Calculating... This may take a few seconds.\n\n");

    let total_lines = count_lines::count_lines_in_directory(&directory, no_git, no_target);

    println!("Total lines of code in '{}': {}", directory, total_lines);
}
