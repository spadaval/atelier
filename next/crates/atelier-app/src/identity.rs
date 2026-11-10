use atelier_core::Thread;

/// The variables that name the calling thread, first match wins: T3 Code's
/// thread, then each harness's session.
const VARIABLES: [&str; 4] = [
    "T3CODE_THREAD_ID",
    "CLAUDE_CODE_SESSION_ID",
    "CODEX_SESSION_ID",
    "GROK_SESSION_ID",
];

/// The calling thread, from the first of [`VARIABLES`] that `var` gives a
/// non-empty value for, else `human`.
pub fn identity(var: impl Fn(&str) -> Option<String>) -> Thread {
    VARIABLES
        .iter()
        .find_map(|name| var(name).filter(|value| !value.trim().is_empty()))
        .map(Thread)
        .unwrap_or_else(|| Thread("human".into()))
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
}
