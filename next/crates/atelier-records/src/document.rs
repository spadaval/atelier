//! A record file: YAML front matter between `---` lines, then a Markdown
//! body that is kept byte for byte.

/// Splits a record into its front matter and its body. Copied from
/// `split_document` in `crates/atelier-records/src/lib.rs`, changed to keep
/// the body as the raw slice where that one strips its leading newlines.
pub(crate) fn split(input: &str) -> Option<(&str, &str)> {
    let rest = input.strip_prefix("---\n")?;
    rest.split_once("\n---\n")
}

/// The inverse of [`split`]: `front_matter` is YAML ending in a newline.
pub(crate) fn join(front_matter: &str, body: &str) -> String {
    format!("---\n{front_matter}---\n{body}")
}

/// The sections of a Markdown `text` under headings of one `level`, as
/// (heading, content) pairs in order. A heading inside a fenced code block
/// is text. Content runs to the next heading of the same or a higher level.
pub(crate) fn sections(text: &str, level: usize) -> Vec<(&str, &str)> {
    let mut sections: Vec<(&str, usize)> = Vec::new();
    let mut ends = Vec::new();
    let mut fence: Option<&str> = None;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let start = offset;
        offset += line.len();
        let trimmed = line.trim_end_matches('\n');
        if let Some(marker) = fence {
            if trimmed.trim_start().starts_with(marker) {
                fence = None;
            }
            continue;
        }
        if let Some(marker) = ["```", "~~~"]
            .into_iter()
            .find(|marker| trimmed.trim_start().starts_with(marker))
        {
            fence = Some(marker);
            continue;
        }
        let hashes = trimmed.len() - trimmed.trim_start_matches('#').len();
        let is_heading = (1..=level).contains(&hashes)
            && (trimmed.len() == hashes || trimmed[hashes..].starts_with(' '));
        if is_heading {
            if !sections.is_empty() && ends.len() < sections.len() {
                ends.push(start);
            }
            if hashes == level {
                sections.push((trimmed[hashes..].trim(), offset));
            }
        }
    }
    if ends.len() < sections.len() {
        ends.push(text.len());
    }
    sections
        .into_iter()
        .zip(ends)
        .map(|((heading, start), end)| (heading, &text[start..end]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_the_body_byte_for_byte() {
        assert_eq!(split("---\na: 1\n---\nbody\n"), Some(("a: 1", "body\n")));
        assert_eq!(split("---\na: 1\n---\n\nbody"), Some(("a: 1", "\nbody")));
        assert_eq!(split("---\na: 1\n---\n"), Some(("a: 1", "")));
        assert_ne!(
            split("---\na: 1\n---\nbody"),
            split("---\na: 1\n---\n\nbody\n")
        );
        assert_eq!(split("a: 1\n---\nbody"), None);
        assert_eq!(split("---\na: 1\nbody"), None);
        assert_eq!(join("a: 1\n", "\nbody"), "---\na: 1\n---\n\nbody");
    }

    #[test]
    fn splits_sections_at_their_level_and_ends_them_at_a_higher_one() {
        let text =
            "intro\n## One\na\n### Sub\nb\n## Two\n```\n## not a heading\n```\n# Top\nc\n## Three";
        assert_eq!(
            sections(text, 2),
            [
                ("One", "a\n### Sub\nb\n"),
                ("Two", "```\n## not a heading\n```\n"),
                ("Three", ""),
            ]
        );
        assert_eq!(sections(text, 3), [("Sub", "b\n")]);
        assert_eq!(sections("##NoSpace\n#### Deeper\n", 2), []);
    }
}
