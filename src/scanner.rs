use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::models::{FindingVuln};
use crate::patterns::get_patterns;

// --- Allowlist: File Extensions ---

const ALLOWED_EXTENSIONS: &[&str] = &[
    // Systems & Compiled
    "rs", "c", "cpp", "cs", "go", "java", "swift", "kt", "dart", "scala",

    // Scripting
    "py", "rb", "php", "lua",
    "sh", "bash", "zsh", "fish",       // shell variants
    "bat", "cmd", "vbs",               // Windows scripting
    "ps1", "psm1", "psd1",             // PowerShell

    // Web
    "js", "ts", "jsx", "tsx",
    "html", "htm", "css", "scss",

    // Data & Config
    "json", "yaml", "yml", "toml",
    "xml", "ini", "cfg", "conf",
    "properties", "env", "config",

    // IaC & Cloud
    "tf", "tfvars", "hcl", "bicep", "template",

    // Certs & Keys
    "pem", "key", "pub",
    "crt", "cer", "csr",
    "p12", "pfx", "jks",

    // Database & API
    "sql", "prisma", "graphql", "gql",

    // Package & Dependency
    "gradle", "lock", "mod",

    // Misc / Enterprise
    "abap",
    "admx", "adml", "pol",

    // Docs & Logs
    "md", "txt", "log",
];


// --- Allowlist: Exact Filenames ---

const ALLOWED_FILENAMES: &[&str] = &[
    ".npmrc",        // npm auth tokens
    ".yarnrc",       // yarn config
    ".htaccess",     // Apache rules
    ".editorconfig", // editor config
    ".dockerignore", // docker ignore

    ".profile",      // shell profile
    "Procfile",      // Heroku
    "Gemfile",       // Ruby
    "Pipfile",       // Python
    "go.sum",        // Go dependencies
];

// --- Allowlist helpers ---
fn is_allowed_extension(ext: Option<&std::ffi::OsStr>) -> bool {
    ext.and_then(|s| s.to_str())
        .map(|s| ALLOWED_EXTENSIONS.contains(&s))
        .unwrap_or(false)
}

fn is_allowed_filename(filename: Option<&std::ffi::OsStr>) -> bool {
    let name = match filename.and_then(|s| s.to_str()) {
        Some(s) => s.to_lowercase(),
        None => return false,
    };

    let prefixes = ["dockerfile","jenkinsfile","makefile","vagrantfile"];
    let starts_with = [".env",".git",".bash",".zsh"];
    let ends_with = [".env"];

    prefixes.iter().any(|p| name.starts_with(p))
        || starts_with.iter().any(|p| name.starts_with(p))
        || ends_with.iter().any(|p| name.ends_with(p))
        || ALLOWED_FILENAMES.iter().any(|p| name == *p)
}

// --- Scan a single file ---
fn scan_file(path: &str) -> Vec<FindingVuln> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return vec![],
    };

    let reader = BufReader::new(file);
    let patterns = get_patterns();
    let mut findings = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l.trim().to_lowercase(),
            Err(_) => continue,
        };

        for pattern in &patterns {
            let matcher = pattern.matcher.to_lowercase();
            if line.contains(&matcher) {
                findings.push(FindingVuln {
                    title: pattern.title.to_string(),
                    id: pattern.id.to_string(),
                    description: pattern.description.to_string(),
                    severity: pattern.severity.clone(),
                    file: path.to_string(),
                    line: line_number + 1,
                });
            }
        }
    }

    findings
}

// --- Scan a directory recursively ---
pub fn scan_directory(path: String) -> Vec<FindingVuln> {
    use std::path::Path;

    if !Path::new(&path).exists() {
        println!("ERROR: Path does not exist -> {}", path);
        return vec![];
    }

    WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            is_allowed_extension(e.path().extension())
                || is_allowed_filename(e.path().file_name())
        })
        .flat_map(|e| {
            let path_str = e.path().to_string_lossy().to_string();
            scan_file(&path_str)
        })
        .collect()
}


