use crate::model::Thread;

/// The variables that name the calling thread, first match wins: T3 Code's
/// thread, then each harness's session.
const VARIABLES: [&str; 4] = [
    "T3CODE_THREAD_ID",
    "CLAUDE_CODE_SESSION_ID",
    "CODEX_SESSION_ID",
    "GROK_SESSION_ID",
];

/// The calling thread, from the first of [`VARIABLES`] that `var` gives a
/// non-empty value for, else `human`. A Codex subagent, whose
/// `CODEX_THREAD_ID` differs from the `CODEX_SESSION_ID` it shares with
/// its parent, is a thread of its own: its parent's identity, `/`, then
/// its `CODEX_THREAD_ID`. A Codex root thread has the two equal.
pub fn identity(var: impl Fn(&str) -> Option<String>) -> Thread {
    let set = |name: &str| var(name).filter(|value| !value.trim().is_empty());
    let parent = VARIABLES
        .iter()
        .find_map(|name| set(name))
        .unwrap_or_else(|| "human".into());
    match (set("CODEX_THREAD_ID"), set("CODEX_SESSION_ID")) {
        (Some(thread), Some(session)) if thread != session => Thread(format!("{parent}/{thread}")),
        _ => Thread(parent),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(set: &[(&str, &str)]) -> String {
        let set: Vec<(String, String)> = set
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        identity(|name| {
            set.iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.clone())
        })
        .0
    }

    #[test]
    fn takes_the_first_variable_set_in_order_else_human() {
        let all = [
            ("T3CODE_THREAD_ID", "t3"),
            ("CLAUDE_CODE_SESSION_ID", "claude"),
            ("CODEX_SESSION_ID", "codex"),
            ("GROK_SESSION_ID", "grok"),
        ];
        for skip in 0..all.len() {
            assert_eq!(with(&all[skip..]), all[skip].1);
        }
        assert_eq!(with(&[]), "human");
        // An empty value is not set.
        assert_eq!(
            with(&[("T3CODE_THREAD_ID", ""), ("GROK_SESSION_ID", "grok")]),
            "grok"
        );
        assert_eq!(with(&[("CODEX_SESSION_ID", " ")]), "human");
    }

    /// What `codex exec` inside a T3 thread had set on 2026-11-11, in
    /// `~/.cache/atelier-identity-probe/root.txt` and `child.txt`: the root
    /// and its `spawn_agent` child share the session and the T3 thread.
    const T3: &str = "5bcb08e9-f2d3-4f91-8690-1f2c2e9401a5";
    const SESSION: &str = "01a1c881-3c90-7600-9c6d-231de181a3c1";
    const CHILD: &str = "01a1c881-65c2-7162-8484-fc63ec812d04";

    #[test]
    fn a_codex_root_thread_keeps_its_identity_and_its_subagent_has_its_own() {
        let root = [
            ("T3CODE_THREAD_ID", T3),
            ("CODEX_SESSION_ID", SESSION),
            ("CODEX_THREAD_ID", SESSION),
        ];
        assert_eq!(with(&root), T3);
        let child = [
            ("T3CODE_THREAD_ID", T3),
            ("CODEX_SESSION_ID", SESSION),
            ("CODEX_THREAD_ID", CHILD),
        ];
        assert_eq!(with(&child), format!("{T3}/{CHILD}"));
    }

    #[test]
    fn a_codex_subagent_outside_t3_code_is_its_session_then_its_thread() {
        let root = [("CODEX_SESSION_ID", SESSION), ("CODEX_THREAD_ID", SESSION)];
        assert_eq!(with(&root), SESSION);
        let child = [("CODEX_SESSION_ID", SESSION), ("CODEX_THREAD_ID", CHILD)];
        assert_eq!(with(&child), format!("{SESSION}/{CHILD}"));
    }

    #[test]
    fn a_codex_thread_without_both_variables_set_changes_nothing() {
        // No session: not a subagent, whatever names the caller.
        assert_eq!(
            with(&[("T3CODE_THREAD_ID", T3), ("CODEX_THREAD_ID", CHILD)]),
            T3
        );
        assert_eq!(with(&[("CODEX_THREAD_ID", CHILD)]), "human");
        // An empty value counts as unset, on either side.
        let empty_session = [
            ("T3CODE_THREAD_ID", T3),
            ("CODEX_SESSION_ID", " "),
            ("CODEX_THREAD_ID", CHILD),
        ];
        assert_eq!(with(&empty_session), T3);
        let empty_thread = [
            ("T3CODE_THREAD_ID", T3),
            ("CODEX_SESSION_ID", SESSION),
            ("CODEX_THREAD_ID", ""),
        ];
        assert_eq!(with(&empty_thread), T3);
        // A Claude Code subagent carries its parent's session: its parent.
        assert_eq!(with(&[("CLAUDE_CODE_SESSION_ID", "claude")]), "claude");
    }
}
