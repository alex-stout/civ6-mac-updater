mod args;
mod utils;

use std::fs;
use owo_colors::OwoColorize;

use clap::Parser;
use args::CivUpdaterArgs;

use crate::utils::{get_full_path, update_contents, update_file};

// default path to the config file 
static DEFAULT_FILE_PATH: &str = "Library/Application Support/Steam/steamapps/common/Sid Meier\'s Civilization VI/Civ6.app/Contents/AspyrAssets/global/String/App.json";

fn main() {
    let args = CivUpdaterArgs::parse();
    let path = get_full_path(DEFAULT_FILE_PATH);

    let raw_contents = fs::read_to_string(&path).expect("Could not open file.");
    
    let contents: serde_json::Value =
        serde_json::from_str(&raw_contents).expect("JSON was not well-formatted");

    let updated_json = update_contents(&contents, &args);

    println!("{}", "Updating configuration:");
    println!("\tVersion number: {} -> {}", contents["App.WinFileVersion"].as_str().unwrap().yellow(), updated_json["App.WinFileVersion"].as_str().unwrap().green());
    println!("\tVersion string: {} -> {}", contents["App.WinFileVersionStr"].as_str().unwrap().yellow(), updated_json["App.WinFileVersionStr"].as_str().unwrap().green());

    update_file(path, &updated_json);

    println!("{}", "Done.".green());
}