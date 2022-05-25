use std::io;
use std::path::Path;
use std::process;

fn main() {
    let config_path = get_config_path();
    let desired_locale = get_desired_locale();
    let desired_region = get_desired_region();

    println!(
        "Path: {}\nLocale: {}\nRegion: {}",
        config_path, desired_locale, desired_region
    );
}

fn get_config_path() -> String {
    let mut input = String::new();
    println!("Enter your config path:");
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let exists = Path::new(&input.trim()).exists();
    if !exists {
        println!("Path does not exist");
        process::exit(1);
    }
    input
}

fn get_desired_locale() -> String {
    let mut input = String::new();
    println!("Enter your desired locale:");
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    input
}

fn get_desired_region() -> String {
    let mut input = String::new();
    println!("Enter your desired region:");
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    input
}
