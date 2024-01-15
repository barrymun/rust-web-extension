mod utils;

use serde_json::Value;
use std::io::{self, Read, Write};
use std::path::Path;
use std::{env, fs};
use utils::types::EngineType::{self, Chromium, Gecko};
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

fn get_engine() -> EngineType {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <chromium|gecko>", args[0]);
        throw_str(std::format!("Usage: {} <chromium|gecko>", args[0]).as_str());
    }

    let engine = match args[1].as_str() {
        "chromium" => Chromium,
        "gecko" => Gecko,
        _ => {
            eprintln!("Invalid argument. Usage: {} <chromium|gecko>", args[0]);
            throw_str(
                std::format!("Invalid argument. Usage: {} <chromium|gecko>", args[0]).as_str(),
            );
        }
    };
    engine
}

/// generates the manifest.json file for the extension.
fn generate_manifest(engine_str: &str, output_dir: &str) -> io::Result<()> {
    let cwd = get_current_working_dir();
    let common_manifest_dir = cwd.clone() + "/extension/engines/common/manifest.json";
    let engine_manifest_dir = cwd.clone() + "/extension/engines/" + engine_str + "/manifest.json";
    let output_manifest_dir = output_dir.to_owned() + "/manifest.json";

    let mut common_manifest_file =
        fs::File::open(common_manifest_dir).expect("Failed to open manifest.json");
    let mut common_manifest_contents = String::new();
    common_manifest_file
        .read_to_string(&mut common_manifest_contents)
        .expect("Failed to read manifest.json");
    let common_manifest_json: Value = serde_json::from_str(&common_manifest_contents)
        .expect("Failed to parse JSON from manifest.json");

    let mut engine_manifest_file =
        fs::File::open(engine_manifest_dir).expect("Failed to open manifest.json");
    let mut engine_manifest_contents = String::new();
    engine_manifest_file
        .read_to_string(&mut engine_manifest_contents)
        .expect("Failed to read manifest.json");
    let engine_manifest_json: Value = serde_json::from_str(&engine_manifest_contents)
        .expect("Failed to parse JSON from manifest.json");

    // merge the JSON objects
    let mut combined_json = common_manifest_json.as_object().unwrap().clone();
    let engine_manifest_object = engine_manifest_json.as_object().unwrap();
    for (key, value) in engine_manifest_object.iter() {
        combined_json.insert(key.clone(), value.clone());
    }

    // convert the JSON object to a nicely formatted JSON string
    let pretty_json_str =
        serde_json::to_string_pretty(&combined_json).expect("Unable to format JSON");
    let mut output_manifest_file =
        fs::File::create(&output_manifest_dir).expect("Failed to create combined.json");
    output_manifest_file
        .write_all(pretty_json_str.as_bytes())
        .expect("Failed to write combined.json");
    println!(
        "manifest.json file created in {} successfully.",
        output_manifest_dir
    );

    Ok(())
}

/// creates the executable code for the scripts.
fn append_script_executable_code(output_dir: &str) -> io::Result<()> {
    let script_types = ["background", "content", "popup"];
    for value in &script_types {
        let executable_code = format!(
            "const runtime = chrome.runtime || browser.runtime;(async () => await wasm_bindgen(runtime.getURL('{}_bg.wasm')))();",
            value
        );
        let executable_dir = output_dir.to_owned() + "/" + value + ".js";
        let mut executable_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&executable_dir)
            .expect("Failed to open file for appending");
        writeln!(executable_file, "{}", executable_code)
            .expect("Failed to write to file for appending");
    }
    Ok(())
}

/// recursively copies the contents of the specified directory to the output directory.
fn copy_popup_assets(src_dir_path: &Path, output_dir_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src_dir_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // if it's a directory, recursively copy its contents
            copy_popup_assets(&entry_path, output_dir_path)?;
        } else if entry_path.is_file() {
            // if it's a file, copy it to the destination
            let entry_name = entry_path.file_name().unwrap();
            let dest_path = output_dir_path.join(entry_name);
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}

/// copies the icons from the extension directory to the output directory.
/// if the output directory does not exist, it will be created.
fn copy_icons() -> io::Result<()> {
    let cwd = get_current_working_dir();
    let src_dir = cwd.clone() + "/extension/icons";
    let output_dir = cwd.clone() + "/dist/icons";
    let output_dir_path = Path::new(&output_dir);
    if !output_dir_path.exists() {
        fs::create_dir_all(&output_dir)?;
    }
    return copy_popup_assets(Path::new(&src_dir), Path::new(&output_dir));
}

/// removes extraneous files from the specified directory.
/// the files to remove are specified in the `files_to_remove` vector.
fn remove_extraneous_files(output_dir: &str) -> io::Result<()> {
    let files_to_remove = vec![".gitignore", "package.json"];
    for file_name in files_to_remove {
        let file_dir = format!("{}/{}", output_dir, file_name);
        if fs::remove_file(&file_dir).is_ok() {
            println!("Removed: {:?}", file_dir);
        }
    }
    Ok(())
}

fn main() {
    let cwd = get_current_working_dir();
    let output_dir = cwd.clone() + "/dist";
    let popup_assets_dir = cwd.clone() + "/scripts/popup/assets";

    let engine = get_engine();
    let engine_str = engine.to_string();
    println!("Building for {} engine", engine_str);

    let res = generate_manifest(&engine_str, &output_dir);
    if res.is_err() {
        println!("Error generating manifest: {:?}", res);
    } else {
        println!("Generated manifest successfully.");
    }

    let res = append_script_executable_code(&output_dir);
    if res.is_err() {
        println!("Error appending executable code: {:?}", res);
    } else {
        println!("Appended executable code successfully.");
    }

    let res = copy_popup_assets(Path::new(&popup_assets_dir), Path::new(&output_dir));
    if res.is_err() {
        println!("Error copying popup assets: {:?}", res);
    } else {
        println!("Copied popup assets successfully.");
    }

    let res = copy_icons();
    if res.is_err() {
        println!("Error copying icons: {:?}", res);
    } else {
        println!("Copied icons successfully.");
    }

    let res = remove_extraneous_files(&output_dir);
    if res.is_err() {
        println!("Error removing extraneous files: {:?}", res);
    } else {
        println!("Removed extraneous files successfully.");
    }
}
