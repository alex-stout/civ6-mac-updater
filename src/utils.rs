use crate::args;

use std::{fs::{ File}, env, path::{Path, PathBuf}, io::{Write}};

use args::CivUpdaterArgs;
use serde_json::{json, Value};

pub fn get_full_path(path: &str) -> PathBuf {
    #[allow(deprecated)]
    let home_path = env::home_dir().expect("Cannot get home dir.").to_path_buf();

    Path::new(&home_path).join(path)
}

pub fn update_file(path: PathBuf, contents: &serde_json::Value) {
    let formatted_contents = serde_json::to_string_pretty(&contents).expect("Unable to format JSON");
    let mut file = File::options().write(true).truncate(true).open(&path).unwrap();
    write!(&mut file, "{}", formatted_contents).expect("Unable to update config file.");
}

pub fn update_contents(json: &Value, args: &CivUpdaterArgs) -> Value {
    let mut updated_json = json.clone();
    updated_json["App.WinFileVersion"] = json!(args.version_number);
    updated_json["App.WinProductVersion"] = json!(args.version_number);

    updated_json["App.WinFileVersionStr"] = json!(args.version_str);
    updated_json["App.WinProductVersionStr"] = json!(args.version_str);

    updated_json
}