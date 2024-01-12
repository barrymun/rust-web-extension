use serde_json::Value;
use std::io::{Read, Write};
use std::{env, fmt, fs};
use wasm_bindgen::throw_str;

enum EngineType {
    Chromium,
    Gecko,
}
impl fmt::Display for EngineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineType::Chromium => write!(f, "chromium"),
            EngineType::Gecko => write!(f, "gecko"),
        }
    }
}
use EngineType::*;

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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <chromium|gecko>", args[0]);
        return;
    }

    let engine = match args[1].as_str() {
        "chromium" => Chromium,
        "gecko" => Gecko,
        _ => {
            println!("Invalid argument. Usage: {} <chromium|gecko>", args[0]);
            return;
        }
    };
    let engine_str = engine.to_string();
    println!("Building for {}", engine_str);

    let cwd = get_current_working_dir();
    let common_manifest_path = cwd.clone() + "/extension/engines/common/manifest.json";
    let engine_manifest_path = cwd.clone() + "/extension/engines/" + &engine_str + "/manifest.json";
    let output_manifest_path = cwd.clone() + "/dist/manifest.json";
    let executable_path = cwd.clone() + "/dist/run.js";

    let mut common_manifest_file =
        fs::File::open(common_manifest_path).expect("Failed to open manifest.json");
    let mut common_manifest_contents = String::new();
    common_manifest_file
        .read_to_string(&mut common_manifest_contents)
        .expect("Failed to read manifest.json");
    let common_manifest_json: Value = serde_json::from_str(&common_manifest_contents)
        .expect("Failed to parse JSON from manifest.json");

    let mut engine_manifest_file =
        fs::File::open(engine_manifest_path).expect("Failed to open manifest.json");
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
        fs::File::create(&output_manifest_path).expect("Failed to create combined.json");
    output_manifest_file
        .write_all(pretty_json_str.as_bytes())
        .expect("Failed to write combined.json");
    println!(
        "manifest.json file created in {} successfully.",
        output_manifest_path
    );

    let executable_code = "const runtime = chrome.runtime || browser.runtime;(async () => await wasm_bindgen(runtime.getURL('content_bg.wasm')))();";
    let mut executable_file = fs::File::create(&executable_path).expect("Unable to create file");
    executable_file
        .write_all(executable_code.as_bytes())
        .expect("Unable to write to file");
    println!("run.js file created in {} successfully.", executable_path);
}
