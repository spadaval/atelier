//! Canonical Markdown record storage boundary.
//!
//! Record discovery, parsing, rendering, ID allocation, relationship rendering,
//! and atomic tracked-file mutation move here during extraction.

pub use atelier_core::RecordId;

/// Canonical record kind vocabulary shared by parser and application code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordKind {
    Evidence,
    Issue,
    Mission,
    Plan,
}

impl RecordKind {
    pub fn directory(self) -> &'static str {
        match self {
            Self::Evidence => "evidence",
            Self::Issue => "issues",
            Self::Mission => "missions",
            Self::Plan => "plans",
        }
    }
}

/// Deterministic front matter/body rendering used by record-store extraction.
pub fn render_document(front_matter: &serde_yaml::Value, body: &str) -> anyhow::Result<String> {
    let mut rendered = String::from("---\n");
    rendered.push_str(&serde_yaml::to_string(front_matter)?);
    rendered.push_str("---\n\n");
    rendered.push_str(body.trim_end());
    rendered.push('\n');
    Ok(rendered)
}

/// Split a canonical Markdown document into YAML front matter and body text.
pub fn split_document(input: &str) -> Option<(&str, &str)> {
    let rest = input.strip_prefix("---\n")?;
    let (front_matter, body) = rest.split_once("\n---\n")?;
    Some((front_matter, body.trim_start_matches('\n')))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_kind_maps_to_canonical_directories() {
        assert_eq!(RecordKind::Issue.directory(), "issues");
        assert_eq!(RecordKind::Evidence.directory(), "evidence");
    }

    #[test]
    fn document_render_round_trips_front_matter_and_body() {
        let front_matter = serde_yaml::to_value([("id", "atelier-test")]).unwrap();
        let rendered = render_document(&front_matter, "## Description\n\nBody").unwrap();
        let (yaml, body) = split_document(&rendered).unwrap();
        assert!(yaml.contains("atelier-test"));
        assert_eq!(body, "## Description\n\nBody\n");
    }
}
