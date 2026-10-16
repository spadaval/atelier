//! The epic to parent issue record rewrite (atelier-9vq3), shared by
//! `atelier migrate epics` and the review gate, which accepts an exact
//! migration of an approved epic as a non-authoring edit.
use anyhow::{bail, Result};

/// An epic record rewritten as a parent issue.
pub struct MigratedEpic {
    pub text: String,
    /// `(from, to)` when a draft epic moves to the draft status of `to`.
    pub status_change: Option<(String, String)>,
}

/// The rewritten record text for an epic, or `None` for any other record.
/// The result is not validated; callers parse it.
pub fn migrate_epic_text(text: &str, to: &str, draft_status: &str) -> Result<Option<MigratedEpic>> {
    let Some((front_matter, body)) = atelier_records::split_document(text) else {
        return Ok(None);
    };
    let is_epic = front_matter
        .lines()
        .any(|line| matches!(line.trim(), "issue_type: \"epic\"" | "issue_type: epic"));
    if !is_epic {
        return Ok(None);
    }
    let mut status_change = None;
    let front_matter = front_matter
        .lines()
        .map(|line| match line.trim() {
            "issue_type: \"epic\"" | "issue_type: epic" => format!("issue_type: \"{to}\""),
            "status: \"draft\"" | "status: draft" => {
                status_change = Some(("draft".to_string(), draft_status.to_string()));
                format!("status: \"{draft_status}\"")
            }
            _ => line.to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n");

    let sections = split_sections(body);
    let section = |name: &str| {
        sections
            .iter()
            .find(|(heading, _)| heading == name)
            .map(|(_, content)| content.trim().to_string())
            .filter(|content| !content.is_empty())
    };
    let goal = section("Outcome")
        .or_else(|| section("Goal"))
        .unwrap_or_else(|| "Outcome was not specified.".to_string());
    let done_when = match (section("Proof"), section("Delivers")) {
        (Some(proof), Some(delivers)) => format!("{proof}\n\nDelivers:\n\n{delivers}"),
        (Some(proof), None) => proof,
        (None, Some(delivers)) => format!("Every child issue is done, delivering:\n\n{delivers}"),
        (None, None) => section("Done when").unwrap_or_else(|| {
            "Every child issue is done. (Migrated from an epic with no Proof route; refine this before starting.)".to_string()
        }),
    };
    let mut new_body = format!("## Goal\n\n{goal}\n\n## Done when\n\n{done_when}\n");
    for name in [
        "Scope",
        "Context",
        "Unknowns",
        "Design",
        "Notes",
        "Close Reason",
    ] {
        if let Some(content) = section(name) {
            new_body.push_str(&format!("\n## {name}\n\n{content}\n"));
        }
    }
    for (heading, _) in &sections {
        if !matches!(
            heading.as_str(),
            "Outcome"
                | "Goal"
                | "Proof"
                | "Delivers"
                | "Done when"
                | "Scope"
                | "Context"
                | "Unknowns"
                | "Design"
                | "Notes"
                | "Close Reason"
        ) {
            bail!("unexpected section '{heading}'; migrate it by hand");
        }
    }
    Ok(Some(MigratedEpic {
        text: format!("---\n{front_matter}\n---\n\n{new_body}"),
        status_change,
    }))
}

/// Level-two sections in order, as `(heading, content)`.
fn split_sections(body: &str) -> Vec<(String, String)> {
    let mut sections: Vec<(String, String)> = Vec::new();
    for line in body.lines() {
        let heading = line
            .strip_prefix("## ")
            .filter(|_| !line.starts_with("###"))
            .map(str::trim);
        match heading {
            Some(heading) => sections.push((heading.to_string(), String::new())),
            None => {
                if let Some((_, content)) = sections.last_mut() {
                    content.push_str(line);
                    content.push('\n');
                }
            }
        }
    }
    sections
}
