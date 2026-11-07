//! Done when lines as a checklist (atelier-jnrh). A bulleted line with a
//! `[ ]` box is ticked by writing `[x]`. Asking for review means every box is
//! ticked, and a change that only ticks boxes does not change what done
//! means.

/// For a bulleted line with a checkbox, whether it is ticked and the text
/// after the box; `None` for any other line.
pub fn checkbox(line: &str) -> Option<(bool, &str)> {
    let (_, ticked, rest) = split_box(line)?;
    Some((ticked, rest.trim()))
}

/// `done_when` with every box read as unticked and surrounding blank space
/// trimmed: two versions that differ only by ticks read the same.
pub fn without_ticks(done_when: &str) -> String {
    done_when
        .trim()
        .lines()
        .map(|line| match split_box(line) {
            Some((before, _, after)) => format!("{before}[ ]{after}"),
            None => line.to_string(),
        })
        .map(|line| line.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The text before a line's box, whether the box is ticked, and the text
/// after it, for a line whose bullet (`-`, `*`, `+`, `1.` or `1)`) starts
/// with a box.
fn split_box(line: &str) -> Option<(&str, bool, &str)> {
    let indent = line.len() - line.trim_start().len();
    let rest = &line[indent..];
    let bullet = ["- ", "* ", "+ "]
        .iter()
        .find(|bullet| rest.starts_with(**bullet))
        .map(|bullet| bullet.len())
        .or_else(|| {
            let digits = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
            (digits > 0 && (rest[digits..].starts_with(". ") || rest[digits..].starts_with(") ")))
                .then_some(digits + 2)
        })?;
    let after_bullet = &rest[bullet..];
    let start = indent + bullet + (after_bullet.len() - after_bullet.trim_start().len());
    let ticked = match line.get(start..start + 3)? {
        "[ ]" => false,
        "[x]" | "[X]" => true,
        _ => return None,
    };
    Some((&line[..start], ticked, &line[start + 3..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boxes_after_any_bullet_are_read_and_other_lines_are_not() {
        assert_eq!(
            checkbox("- [ ] A test passes."),
            Some((false, "A test passes."))
        );
        assert_eq!(checkbox("  * [x] Done"), Some((true, "Done")));
        assert_eq!(checkbox("1. [X] First"), Some((true, "First")));
        assert_eq!(checkbox("2)  [ ] Second"), Some((false, "Second")));
        for line in [
            "[ ] no bullet",
            "- no box",
            "Prose.",
            "-[ ] tight",
            "- [y] odd",
            "",
        ] {
            assert_eq!(checkbox(line), None, "{line}");
        }
    }

    #[test]
    fn ticks_do_not_change_the_text_but_words_do() {
        let before = "- [ ] One\n- [ ] Two\n";
        assert_eq!(without_ticks(before), without_ticks("- [x] One\n- [X] Two"));
        assert_eq!(without_ticks("1. [x] One"), "1. [ ] One");
        assert_ne!(
            without_ticks(before),
            without_ticks("- [x] One\n- [ ] Two, on macOS")
        );
        assert_ne!(
            without_ticks(before),
            without_ticks("- [ ] One\n- [ ] Two\n- [ ] Three")
        );
    }
}
