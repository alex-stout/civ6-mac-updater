# Civ VI Mac Updater CLI
CLI for multiplayer Civ 6 games. Updates the version number on Macs to match Windows game versions. This fixes the host version mismatch error.

## Installation

Using the Rust toolchain:
```console
$ cargo install civ6-mac-updater
```

This installs the CLI.

To run:
```console
$ civ6-mac-updater
```

## Usage

```console
$ civ-6-mac-updater
Fetching values from remote repository... Retrieved values
Updating configuration:
	Version number: 1.0.12.37 -> 1.0.12.37
	Version string: (871434) -> (871434)
Done.
```