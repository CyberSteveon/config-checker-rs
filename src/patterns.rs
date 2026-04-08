use crate::models::Severity;

pub struct Pattern {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub severity: Severity,
    pub matcher: &'static str,
}

pub fn get_patterns() -> Vec<Pattern> {
    vec![
        Pattern {
            id: "DEBUG_ENABLED",
            title: "Debug Mode Enabled",
            description: "Debug mode should not be enabled in production",
            severity: Severity::Medium,
            matcher: "debug = true",
        },
        Pattern {
            id: "API_KEY",
            title: "Potential API Key",
            description: "Possible API key detected",
            severity: Severity::High,
            matcher: "api_key",
        },
        Pattern {
            id: "PASSWORD",
            title: "Hardcoded Password",
            description: "Possible hardcoded password detected",
            severity: Severity::High,
            matcher: "password",
        },
        Pattern {
            id: "SECRET",
            title: "Generic Secret",
            description: "Potential secret detected",
            severity: Severity::High,
            matcher: "secret",
        },
    ]
}