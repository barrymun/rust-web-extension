use std::fs::File;
use std::io::{self, Read};
use toml::Value;
use serde::Deserialize;
use serde_json::to_writer_pretty;
use serde_json::from_value;

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
}

fn main() -> Result<(), std::io::Error> {
    // Read the Cargo.toml file
    // let mut toml_contents = String::new();
    // File::open("Cargo.toml")?.read_to_string(&mut toml_contents)?;

    // // Parse the Cargo.toml file as TOML
    // let cargo_toml: Value = toml::from_str(&toml_contents)?;

    // // Deserialize the package section
    // let package: CargoToml = from_value(cargo_toml["package"].clone())?;

    // // Create a JSON representation of the package
    // let json_contents = serde_json::to_string_pretty(&package)?;

    // // Write the JSON to manifest.json
    // let mut json_file = File::create("manifest.json")?;
    // to_writer_pretty(&mut json_file, &json_contents)?;

    println!("manifest.json file created successfully!");

    Ok(())
}
