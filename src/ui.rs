use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub fn get_directory_path() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the directory path")
        .interact_text()
        .expect("Failed to read directory path")
}

pub fn ask_exclude_git() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Exclude Git-related files?")
        .interact()
        .expect("Failed to read input")
}

pub fn ask_exclude_target() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Exclude Rust's 'target' directory?")
        .interact()
        .expect("Failed to read input")
}

pub fn ask_exclude_node_modules() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Exclude Node Modules directory?")
        .interact()
        .expect("Failed to read input")
}

pub fn ask_exclude_pycache() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Exclude __pycache__ directory?")
        .interact()
        .expect("Failed to read input")
}
