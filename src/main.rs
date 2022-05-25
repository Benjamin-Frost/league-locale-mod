use std::io::Write;

fn main() {
    // Get User Inputs
    let config_path = get_user_input("Path to config file");
    assert_path_exists(&config_path);
    let locale = get_user_input("Locale");
    let region = get_user_input("Region");

    // Open Config File
    let config_file_reader = std::fs::File::open(&config_path).expect("Could not open config file");
    let mut config_values: serde_yaml::Value =
        serde_yaml::from_reader(config_file_reader).expect("Could not parse config file");

    // Read relevant values, write user input
    let globals = config_values
        .get_mut("install")
        .unwrap()
        .get_mut("globals")
        .unwrap();
    *globals.get_mut("locale").unwrap() = locale.into();
    *globals.get_mut("region").unwrap() = region.into();

    // Write to file
    let config_file_writer =
        std::fs::File::create(&config_path).expect("Could not open config file");
    serde_yaml::to_writer(config_file_writer, &config_values)
        .expect("Could not write to config file");

    // Done
    println!("Done");
}

fn get_user_input(request: &str) -> String {
    let mut input = String::new();
    print!("{}: ", request);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn assert_path_exists(path: &str) {
    let exists = std::path::Path::new(path).exists();
    if !exists {
        println!("Path does not exist");
        std::process::exit(1);
    }
}
