use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum Severity {
    Minimal,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize)]
pub struct FindingVuln {
    pub title: String,
    pub id: String,
    pub description: String,
    pub severity: Severity,
    pub file: String,
    pub line: usize,
}