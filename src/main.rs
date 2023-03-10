mod args;
mod utils;

use owo_colors::OwoColorize;

use args::CivUpdaterArgs;
use clap::Parser;

use crate::utils::{
    create_backup, done, get_config_file, get_full_path, get_values, json_diff, update_contents,
    update_file,
};

// default path to the config file
static DEFAULT_FILE_PATH: &str = "Library/Application Support/Steam/steamapps/common/Sid Meier\'s Civilization VI/Civ6.app/Contents/AspyrAssets/global/String/App.json";

fn main() {
    let args = CivUpdaterArgs::parse();
    let path = get_full_path(DEFAULT_FILE_PATH);

    let raw_contents = get_config_file(&path);

    let contents: serde_json::Value =
        serde_json::from_str(&raw_contents).expect("JSON was not well-formatted");

    let mut values: Vec<String> = Vec::new();

    if args.version_str.is_empty() || args.version_str.is_empty() {
        values.extend(get_values());
    } else {
        values.extend([args.version_number, args.version_str].to_vec());
    }

    let updated_json = update_contents(&contents, values);

    println!("Found configuration at: {}", path.display());

    if !json_diff(&contents, &updated_json) {
        println!("Values are up-to-date.");
        done();
        return;
    }

    println!("Updated configuration:");
    println!(
        "\tVersion number: {} -> {}",
        contents["App.WinFileVersion"].as_str().unwrap().yellow(),
        updated_json["App.WinFileVersion"].as_str().unwrap().green()
    );

    println!(
        "\tVersion string: {} -> {}",
        contents["App.WinFileVersionStr"].as_str().unwrap().yellow(),
        updated_json["App.WinFileVersionStr"]
            .as_str()
            .unwrap()
            .green()
    );

    print!("Creating backup... ");
    create_backup(&path, &contents);
    done();

    print!("Updating values... ");
    update_file(&path, &updated_json);
    done();
}
