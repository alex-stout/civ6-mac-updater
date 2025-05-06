# Civ VI Mac Updater CLI
CLI for multiplayer Civ 6 games. Updates the version number on Macs to match Windows game versions. This fixes the version mismatch with host error.

## Error Example
![error example](./docs/images/example_error.png)

## Installation

Installing the CLI via the Rust toolchain:
```console
$ cargo install civ6-mac-updater
```
*Note: If you do not have the Rust toolchain installed follow the instructions here: https://www.rust-lang.org/tools/install*

## Usage
The easiest way to use the cli is without any args. This will automatically reach out to the GitHub repository and grab the latest values: 
```console
$ civ6-mac-updater
Fetching values from remote repository... Done.
Found configuration at: ~/Library/Application Support/Steam/steamapps/common/Sid Meier's Civilization VI/Civ6.app/Contents/AspyrAssets/global/String/App.json
Updated configuration:
	Version number: 1.0.12.28 -> 1.0.12.37
	Version string: (846892) -> (871434)
Creating backup... Done.
Updating values... Done.
```

### Custom Values
To specify custom version values, you can call the cli with the following args:
- `--version-number`: string in the format similar to 1.0.12.28
- `--version-str`: string in the format similar to (846892)

```console
$ civ6-mac-updater --version-number test --version-str test
Found configuration at: ~/Library/Application Support/Steam/steamapps/common/Sid Meier's Civilization VI/Civ6.app/Contents/AspyrAssets/global/String/App.json
Updated configuration:
	Version number: 1.0.12.37 -> test
	Version string: (871434) -> test
Creating backup... Done.
Updating values... Done.
```
