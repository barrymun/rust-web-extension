use std::fs;
use std::io::{Read, Write};
use toml::Value as TomlValue;

fn main() {
    let toml_file_path = "../extension/Cargo.toml";
    
    // Read the contents of Cargo.toml from one directory below
    let mut toml_content = String::new();
    fs::File::open(&toml_file_path)
        .expect("Unable to open Cargo.toml")
        .read_to_string(&mut toml_content)
        .expect("Unable to read Cargo.toml");

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
    });

    // Convert the JSON object to a nicely formatted JSON string
    let pretty_json = serde_json::to_string_pretty(&manifest).expect("Unable to format JSON");

    // Define the path for the manifest.json file in the destination directory
    let destination_path = "../extension/pkg/manifest.json";

    // Create a new file named "manifest.json" in the destination directory and open it for writing
    let mut file = fs::File::create(&destination_path).expect("Unable to create file");

    // Write the nicely formatted JSON content to the file
    file.write_all(pretty_json.as_bytes())
        .expect("Unable to write to file");

    println!("manifest.json file created in {} successfully.", destination_path);
}
