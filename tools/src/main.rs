use std::{fs, env};
use std::io::{Read, Write};
use toml::Value as TomlValue;
use wasm_bindgen::throw_str;

fn get_current_working_dir() -> String {
    match env::current_dir() {
        Ok(current_dir) => {
            return current_dir.to_str().unwrap().to_string();
        }
        Err(err) => {
            eprintln!("Error getting current working directory: {}", err);
            throw_str(std::format!("Error getting current working directory: {}", err).as_str());
        }
    }
}

fn main() {
    let cwd = get_current_working_dir();
    let toml_file_path = cwd.clone() + "/extension/Cargo.toml";
    let destination_path = cwd.clone() + "/dist/manifest.json";
    
    // Read the contents of Cargo.toml from one directory below
    let mut toml_content = String::new();
    fs::File::open(&toml_file_path)
        .expect("Unable to open Cargo.toml")
        .read_to_string(&mut toml_content)
        .expect("Unable to read Cargo.toml");
    println!("Cargo.toml read successfully.");

    // Parse the TOML content into a toml::Value
    let cargo_data: TomlValue =
        toml::from_str(&toml_content).expect("Unable to parse Cargo.toml");

    // Extract the values for "name", "version", and "description" from Cargo.toml
    let name = cargo_data["package"]["name"]
        .as_str()
        .expect("Missing 'name' in Cargo.toml");
    let version = cargo_data["package"]["version"]
        .as_str()
        .expect("Missing 'version' in Cargo.toml");
    let description = cargo_data["package"]["description"]
        .as_str()
        .expect("Missing 'description' in Cargo.toml");

    // Create a JSON object representing your manifest
    let manifest = serde_json::json!({
        "manifest_version": 3,
        "name": name,
        "version": version,
        "description": description,
        "content_scripts": [
            {
                "matches": ["<all_urls>"],
                "js": ["content.js"]
            }
        ],
    });

    // Convert the JSON object to a nicely formatted JSON string
    let pretty_json = serde_json::to_string_pretty(&manifest).expect("Unable to format JSON");

    // Create a new file named "manifest.json" in the destination directory and open it for writing
    let mut file = fs::File::create(&destination_path).expect("Unable to create file");

    // Write the nicely formatted JSON content to the file
    file.write_all(pretty_json.as_bytes())
        .expect("Unable to write to file");

    println!("manifest.json file created in {} successfully.", destination_path);
}
