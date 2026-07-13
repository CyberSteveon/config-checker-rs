# Config Checker (Rust)

## Overview

Config Checker is a lightweight Rust CLI tool that was derived from Securely Scoped During development, portions of the scanning engine were separated into a command-line prototype (Config Checker RS). This allowed rapid iteration on directory traversal and rule validation before integrating the functionality into the desktop application.

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

This tool is intetened to test iteration on directory traversal and rule validation before integrating the functionality into the desktop application Securely Scoped

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
