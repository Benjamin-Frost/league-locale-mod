use std::fs::File;
use std::io;
use std::path::Path;
use std::process;

fn main() {
    let config_path = get_config_path();
    let desired_locale = get_desired_locale();
    let desired_region = get_desired_region();

    let f = File::open(&config_path).unwrap();
    let mut value: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();

    let globals = value
        .get_mut("install")
        .unwrap()
        .get_mut("globals")
        .unwrap();
    *globals.get_mut("locale").unwrap() = desired_locale.into();
    *globals.get_mut("region").unwrap() = desired_region.into();

    let writer = File::create(&config_path).unwrap();
    serde_yaml::to_writer(writer, &value).unwrap();

    println!("Done");
}

fn get_config_path() -> String {
    let mut input = String::new();
    println!("Enter your config path:");
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    input = input.trim().to_string();
    let exists = Path::new(&input).exists();
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
    input.trim().to_string()
}

fn get_desired_region() -> String {
    let mut input = String::new();
    println!("Enter your desired region:");
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    input.trim().to_string()
}
