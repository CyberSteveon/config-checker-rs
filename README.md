# Config Checker (Rust)

## Overview

Config Checker is a lightweight Rust CLI tool that scans files and directories for potential security risks such as hardcoded secrets and unsafe configurations.

It’s designed to be fast, simple, and useful during development or system reviews.

## Features

* Recursive directory scanning
* File filtering (extensions + sensitive filenames)
* Detects:

  * Hardcoded passwords
  * API keys
  * Generic secrets
  * Debug mode enabled
* Outputs file + line number for findings
* Built in Rust for speed and reliability

## Why I Built This

Manually reviewing configuration files for secrets or insecure settings is slow and inconsistent.

I built this tool to quickly scan a system and surface potential risks early, making the setup and review process more efficient.

## Usage

Run the scanner:

```bash
cargo run -- <path>
```

### PowerShell Example

```powershell
cargo run -- .\TestFolder
```

## Example Output

```text
Scanning: .\TestFolder

[High] TestFolder\test.env:1 → Potential API Key
[High] TestFolder\test.env:2 → Hardcoded Password
[Medium] TestFolder\test.env:3 → Debug Mode Enabled
```

## How It Works

* Walks directories recursively
* Filters relevant files
* Scans each file line-by-line
* Matches against predefined patterns
* Reports findings with severity levels

## Limitations

* Basic string matching (no regex yet)
* Possible false positives
* No ignore/config system yet

## Future Improvements

* Regex-based detection
* Configurable rules
* JSON output
* CLI flags
* Ignore file support

## Disclaimer

This tool is intended for development and learning purposes. It should not be relied on as a complete security solution.
