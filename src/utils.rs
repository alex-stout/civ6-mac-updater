use owo_colors::OwoColorize;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use serde_json::{json, Value};

pub fn get_full_path(path: &str) -> PathBuf {
    #[allow(deprecated)]
    let home_path = env::home_dir().expect("Cannot get home dir.");

    Path::new(&home_path).join(path)
}

pub fn done() {
    println!("{}", "Done.".green());
}

pub fn create_backup(path: &Path, contents: &serde_json::Value) {
    let backup_path = format!("{}{}", path.display(), ".backup");
    let formatted_contents =
        serde_json::to_string_pretty(&contents).expect("Unable to format JSON");
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(backup_path)
        .expect("Could not create backup file.");
    writeln!(&mut file, "{}", formatted_contents).expect("Unable to update config file.");
}

pub fn update_file(path: &PathBuf, contents: &serde_json::Value) {
    let formatted_contents =
        serde_json::to_string_pretty(&contents).expect("Unable to format JSON");
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
    writeln!(&mut file, "{}", formatted_contents).expect("Unable to update config file.");
}

/**
 * Compares the JSON values
 * @returns same = false, different = true
 */
pub fn json_diff(original_json: &Value, new_json: &Value) -> bool {
    let keys = vec![
        "App.WinFileVersion",
        "App.WinProductVersion",
        "App.WinFileVersionStr",
        "App.WinProductVersionStr",
    ];

    for key in keys.iter() {
        if original_json[key] != new_json[key] {
            return true;
        }
    }

    false
}

pub fn update_contents(json: &Value, args: Vec<String>) -> Value {
    let mut updated_json = json.clone();
    updated_json["App.WinFileVersion"] = json!(args[0]);
    updated_json["App.WinProductVersion"] = json!(args[0]);

    updated_json["App.WinFileVersionStr"] = json!(args[1]);
    updated_json["App.WinProductVersionStr"] = json!(args[1]);

    updated_json
}

pub fn get_values() -> Vec<String> {
    print!("Fetching values from remote repository... ");

    let resp = reqwest::blocking::get(
        "https://raw.githubusercontent.com/alex-stout/civ6-mac-updater/main/values.json",
    )
    .unwrap()
    .text()
    .unwrap();
    let values: Value = serde_json::from_str(resp.as_str()).expect("JSON was not well-formatted");

    let version = values["version"]
        .as_str()
        .expect("Incorrect or missing version from values.json");
    let version_str = values["version_str"]
        .as_str()
        .expect("Incorrect or missing version_str from values.json");

    done();

    [String::from(version), String::from(version_str)].to_vec()
}
