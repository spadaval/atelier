//! The record IDs commands take: matching a full or short ID, and the one
//! not-found message every command gives.

use anyhow::{anyhow, Error, Result};

/// How many near matches a not-found message names.
const NEAR_MATCH_LIMIT: usize = 3;

/// A kind of record a command can name by ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordKind {
    Issue,
    Evidence,
    Review,
}

impl RecordKind {
    pub const ALL: [RecordKind; 3] = [RecordKind::Issue, RecordKind::Evidence, RecordKind::Review];

    /// The kind as a message names it.
    pub fn noun(self) -> &'static str {
        match self {
            RecordKind::Issue => "issue",
            RecordKind::Evidence => "evidence record",
            RecordKind::Review => "review",
        }
    }

    pub fn show_command(self) -> &'static str {
        match self {
            RecordKind::Issue => "atelier issue show",
            RecordKind::Evidence => "atelier evidence show",
            RecordKind::Review => "atelier review show",
        }
    }

    pub fn list_command(self) -> &'static str {
        match self {
            RecordKind::Issue => "atelier issue list",
            RecordKind::Evidence => "atelier evidence list",
            RecordKind::Review => "atelier review list",
        }
    }
}

/// The ID among `ids` that `reference` names: the full ID, or a unique short
/// ID (the part after the project slug), ignoring case. Fails when a short ID
/// names more than one record.
pub fn match_id<'a>(
    kind: RecordKind,
    reference: &str,
    ids: impl IntoIterator<Item = &'a str>,
) -> Result<Option<String>> {
    let reference = reference.trim().to_ascii_lowercase();
    if reference.is_empty() {
        return Ok(None);
    }
    let suffix = format!("-{reference}");
    let mut short_matches = Vec::new();
    for id in ids {
        if id == reference {
            return Ok(Some(id.to_string()));
        }
        if !reference.contains('-') && id.ends_with(&suffix) {
            short_matches.push(id.to_string());
        }
    }
    match short_matches.as_slice() {
        [] => Ok(None),
        [id] => Ok(Some(id.clone())),
        _ => Err(anyhow!(
            "{} {reference} is ambiguous: {}. Use the full ID.",
            kind.noun(),
            short_matches.join(", ")
        )),
    }
}

/// `<kind> <reference> not found`, then up to three IDs among `ids` the
/// reference may have meant, or the command that lists the kind.
pub fn not_found<'a>(
    kind: RecordKind,
    reference: &str,
    ids: impl IntoIterator<Item = &'a str>,
) -> Error {
    let near = near_matches(reference, ids);
    let recovery = match near.as_slice() {
        [] => format!("Next: {}", kind.list_command()),
        _ if near.len() > NEAR_MATCH_LIMIT => format!(
            "Did you mean {} (and {} more)?",
            near[..NEAR_MATCH_LIMIT].join(", "),
            near.len() - NEAR_MATCH_LIMIT
        ),
        [only] => format!("Did you mean {only}?"),
        [rest @ .., last] => format!("Did you mean {} or {last}?", rest.join(", ")),
    };
    anyhow!("{} {} not found. {recovery}", kind.noun(), reference.trim())
}

/// `<id> is <kind>, not <expected>`, with the command that shows it.
pub fn wrong_kind(expected: RecordKind, actual: RecordKind, id: &str) -> Error {
    anyhow!(
        "{id} is {}, not {}. Use `{} {id}`.",
        with_article(actual.noun()),
        with_article(expected.noun()),
        actual.show_command()
    )
}

/// `noun` after "a", or "an" when it starts with a vowel letter. Record kinds
/// and workflow issue types are lowercase words, so the letter is the guide.
pub fn with_article(noun: &str) -> String {
    let article = match noun.chars().next() {
        Some(first) if "aeiou".contains(first.to_ascii_lowercase()) => "an",
        _ => "a",
    };
    format!("{article} {noun}")
}

/// Whether any of `ids` is one [`not_found`] would name for `reference`.
pub fn has_near_match<'a>(reference: &str, ids: impl IntoIterator<Item = &'a str>) -> bool {
    !near_matches(reference, ids).is_empty()
}

/// IDs whose short ID starts with the reference's, or is a prefix of it, or
/// differs from it in one character.
fn near_matches<'a>(reference: &str, ids: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    let reference = reference.trim().to_ascii_lowercase();
    let wanted = short_id(&reference);
    if wanted.is_empty() {
        return Vec::new();
    }
    ids.into_iter()
        .filter(|id| {
            let short = short_id(id);
            short.starts_with(wanted)
                || (!short.is_empty() && wanted.starts_with(short))
                || one_substitution(short, wanted)
        })
        .map(str::to_string)
        .collect()
}

fn short_id(id: &str) -> &str {
    id.rsplit_once('-').map_or(id, |(_, short)| short)
}

fn one_substitution(left: &str, right: &str) -> bool {
    left.len() == right.len()
        && left
            .bytes()
            .zip(right.bytes())
            .filter(|(left, right)| left != right)
            .count()
            == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const IDS: &[&str] = &[
        "atelier-1w8c",
        "atelier-7a2b",
        "atelier-7c3d",
        "atelier-7e4f",
        "atelier-7g5h",
    ];

    fn ids() -> impl Iterator<Item = &'static str> {
        IDS.iter().copied()
    }

    #[test]
    fn short_and_full_ids_match_ignoring_case() {
        for reference in ["1w8c", "1W8C", "atelier-1w8c", "ATELIER-1W8C", " 1w8c "] {
            assert_eq!(
                match_id(RecordKind::Issue, reference, ids()).unwrap(),
                Some("atelier-1w8c".to_string()),
                "{reference}"
            );
        }
        assert_eq!(match_id(RecordKind::Issue, "1w8", ids()).unwrap(), None);
        assert_eq!(match_id(RecordKind::Issue, "", ids()).unwrap(), None);
    }

    #[test]
    fn a_short_id_two_slugs_share_is_ambiguous() {
        let error = match_id(RecordKind::Review, "1w8c", ["atelier-1w8c", "other-1w8c"])
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "review 1w8c is ambiguous: atelier-1w8c, other-1w8c. Use the full ID."
        );
    }

    #[test]
    fn not_found_names_near_matches_or_the_list_command() {
        assert_eq!(
            not_found(RecordKind::Issue, "1w8", ids()).to_string(),
            "issue 1w8 not found. Did you mean atelier-1w8c?"
        );
        assert_eq!(
            not_found(RecordKind::Issue, "1w8d", ids()).to_string(),
            "issue 1w8d not found. Did you mean atelier-1w8c?"
        );
        assert_eq!(
            not_found(RecordKind::Issue, "7", ids()).to_string(),
            "issue 7 not found. Did you mean atelier-7a2b, atelier-7c3d, atelier-7e4f (and 1 more)?"
        );
        assert_eq!(
            not_found(RecordKind::Issue, "7", ids().take(3)).to_string(),
            "issue 7 not found. Did you mean atelier-7a2b or atelier-7c3d?"
        );
        assert_eq!(
            not_found(RecordKind::Evidence, "zzzz", ids()).to_string(),
            "evidence record zzzz not found. Next: atelier evidence list"
        );
    }

    #[test]
    fn with_article_picks_an_before_a_vowel() {
        assert_eq!(with_article("incident"), "an incident");
        assert_eq!(with_article("Epic"), "an Epic");
        assert_eq!(with_article("feature"), "a feature");
        assert_eq!(with_article("review"), "a review");
    }

    #[test]
    fn wrong_kind_names_both_kinds_and_the_show_command() {
        assert_eq!(
            wrong_kind(RecordKind::Issue, RecordKind::Evidence, "atelier-waj0").to_string(),
            "atelier-waj0 is an evidence record, not an issue. Use `atelier evidence show atelier-waj0`."
        );
    }
}
