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

/// The lines of a Markdown `text` that are not in a fenced code block, each
/// with the offset it starts at and without its newline. The fence lines
/// themselves are left out too, so an example in a fence is never read as a
/// heading or a field. As in CommonMark, a fence closes only on a line of
/// its own character, at least as long, with nothing after it.
pub(crate) fn unfenced_lines(text: &str) -> Vec<(usize, &str)> {
    let mut lines = Vec::new();
    let mut open: Option<(char, usize)> = None;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let start = offset;
        offset += line.len();
        let line = line.trim_end_matches('\n');
        match (open, fence(line)) {
            (Some((c, len)), Some((close, close_len, rest)))
                if close == c && close_len >= len && rest.trim().is_empty() =>
            {
                open = None
            }
            (Some(_), _) => {}
            (None, Some((c, len, _))) => open = Some((c, len)),
            (None, None) => lines.push((start, line)),
        }
    }
    lines
}

/// A fence line's character, its length, and what follows it: three or
/// more backticks or tildes, indented by at most three spaces. After
/// backticks no backtick may follow, so ```` ```code``` ```` is inline code.
fn fence(line: &str) -> Option<(char, usize, &str)> {
    let marks = line.trim_start_matches(' ');
    let c = marks.chars().next().filter(|c| matches!(c, '`' | '~'))?;
    let rest = marks.trim_start_matches(c);
    let len = marks.len() - rest.len();
    let is_fence = line.len() - marks.len() <= 3 && len >= 3 && !(c == '`' && rest.contains('`'));
    is_fence.then_some((c, len, rest))
}

/// The sections of a Markdown `text` under headings of one `level`, as
/// (heading, content) pairs in order. A heading inside a fenced code block
/// is text. Content runs to the next heading of the same or a higher level.
pub(crate) fn sections(text: &str, level: usize) -> Vec<(&str, &str)> {
    let mut sections: Vec<(&str, usize)> = Vec::new();
    let mut ends = Vec::new();
    for (start, line) in unfenced_lines(text) {
        let hashes = line.len() - line.trim_start_matches('#').len();
        let is_heading = (1..=level).contains(&hashes)
            && (line.len() == hashes || line[hashes..].starts_with(' '));
        if is_heading {
            if ends.len() < sections.len() {
                ends.push(start);
            }
            if hashes == level {
                let content_start = (start + line.len() + 1).min(text.len());
                sections.push((line[hashes..].trim(), content_start));
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

    #[test]
    fn leaves_out_fenced_lines_and_their_fences() {
        let lines =
            |text| -> Vec<&str> { unfenced_lines(text).into_iter().map(|(_, l)| l).collect() };
        assert_eq!(
            lines("a\n```text\nTaken: x\n~~~\n```\nb\n~~~\n```\n~~~\nc"),
            ["a", "b", "c"]
        );
        // Only a line of the same character, at least as long, closes.
        assert_eq!(lines("````\n```\nTaken: x\n```\n````\nb"), ["b"]);
        assert_eq!(lines("~~~~\n~~~\n~~~~ not closing\nx\n~~~~~\nb"), ["b"]);
        // Backticks with a backtick after them are inline code, not a fence.
        assert_eq!(lines("```inline code```\nb"), ["```inline code```", "b"]);
        // Four spaces of indent is no fence.
        assert_eq!(lines("    ```\na"), ["    ```", "a"]);
    }
}
