#![allow(dead_code)]

use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use tempfile::TempDir;

/// Result of running an Atelier CLI command.
#[derive(Debug)]
pub struct CmdResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl CmdResult {
    pub fn stdout_contains(&self, expected: &str) -> bool {
        self.stdout.contains(expected)
    }

    pub fn stderr_contains(&self, expected: &str) -> bool {
        self.stderr.contains(expected)
    }
}

/// Isolated test environment for smoke-testing the Atelier CLI.
pub struct SmokeHarness {
    pub temp_dir: TempDir,
    pub atelier_bin: PathBuf,
    issue_ids: Mutex<Vec<String>>,
}

impl SmokeHarness {
    /// Create a fully initialised test environment.
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let bin = PathBuf::from(env!("CARGO_BIN_EXE_atelier"));

        let harness = SmokeHarness {
            temp_dir,
            atelier_bin: bin,
            issue_ids: Mutex::new(Vec::new()),
        };

        // Run atelier init
        let result = harness.run(&["init"]);
        assert!(result.success, "atelier init failed: {}", result.stderr);
        let result = harness.run(&["workflow", "init"]);
        assert!(
            result.success,
            "atelier workflow init failed: {}",
            result.stderr
        );

        harness
    }

    /// Create a harness without running `atelier init`.
    pub fn new_bare() -> Self {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let bin = PathBuf::from(env!("CARGO_BIN_EXE_atelier"));
        SmokeHarness {
            temp_dir,
            atelier_bin: bin,
            issue_ids: Mutex::new(Vec::new()),
        }
    }

    /// Run an Atelier CLI command and return the full result.
    pub fn run(&self, args: &[&str]) -> CmdResult {
        let translated_args = self.translate_issue_refs_owned(args);
        let output = Command::new(&self.atelier_bin)
            .current_dir(self.temp_dir.path())
            .args(&translated_args)
            .output()
            .expect("failed to execute atelier");

        let result = CmdResult {
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        };

        if result.success {
            self.register_issue_ids_from_stdout(&result.stdout);
            self.register_issue_ids_from_state();
        }

        result
    }

    /// Run an Atelier CLI command and assert it succeeds.
    pub fn run_ok(&self, args: &[&str]) -> CmdResult {
        let result = self.run(args);
        assert!(
            result.success,
            "expected atelier {:?} to succeed but got exit code {}.\nstdout: {}\nstderr: {}",
            args, result.exit_code, result.stdout, result.stderr,
        );
        result
    }

    /// Attach minimal validation proof to an issue fixture.
    pub fn attach_issue_pass_evidence(&self, issue_ref: &str) -> String {
        let issue_id = self.translate_issue_ref(issue_ref);
        let summary = format!("issue close proof for {issue_id}");
        let evidence = self.run_ok(&[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--target",
            &format!("issue/{issue_id}"),
            &summary,
        ]);
        let evidence_id = first_record_id(&evidence.stdout)
            .expect("evidence record did not print an evidence id");
        evidence_id
    }

    /// Close an issue fixture through the current proof-backed closeout path.
    pub fn close_issue_with_evidence(&self, issue_ref: &str) {
        let issue_id = self.translate_issue_ref(issue_ref);
        self.attach_issue_pass_evidence(&issue_id);
        self.run_ok(&["issue", "update", &issue_id, "--issue-type", "spike"]);
        self.run_ok(&["issue", "transition", &issue_id, "start"]);
        self.run_ok(&["issue", "transition", &issue_id, "request_review"]);
        self.run_ok(&["issue", "close", &issue_id, "--reason", "fixture complete"]);
    }

    /// Run an Atelier CLI command and assert it fails.
    pub fn run_err(&self, args: &[&str]) -> CmdResult {
        let result = self.run(args);
        assert!(
            !result.success,
            "expected atelier {:?} to fail but it succeeded.\nstdout: {}\nstderr: {}",
            args, result.stdout, result.stderr,
        );
        result
    }

    /// Path to the `.atelier/` directory.
    pub fn atelier_dir(&self) -> PathBuf {
        self.temp_dir.path().join(".atelier")
    }

    /// Path to the SQLite database.
    pub fn db_path(&self) -> PathBuf {
        self.atelier_dir().join("state.db")
    }

    pub fn issue_id(&self, ordinal: usize) -> String {
        self.issue_ref(ordinal)
    }

    fn translate_issue_refs(&self, args: &[&str]) -> Vec<String> {
        self.translate_issue_refs_owned(
            &args
                .iter()
                .map(|arg| (*arg).to_string())
                .collect::<Vec<_>>(),
        )
    }

    fn translate_issue_refs_owned<T: AsRef<str>>(&self, args: &[T]) -> Vec<String> {
        args.iter()
            .enumerate()
            .map(|(index, arg)| {
                if issue_ref_position(args, index) {
                    self.translate_issue_ref(arg.as_ref())
                } else {
                    arg.as_ref().to_string()
                }
            })
            .collect()
    }

    fn translate_issue_ref(&self, value: &str) -> String {
        let numeric = value.strip_prefix('#').unwrap_or(value);
        match numeric.parse::<usize>() {
            Ok(ordinal) => self.issue_ref(ordinal),
            Err(_) => value.to_string(),
        }
    }

    fn issue_ref(&self, ordinal: usize) -> String {
        self.issue_ids
            .lock()
            .unwrap()
            .get(ordinal - 1)
            .cloned()
            .unwrap_or_else(|| ordinal.to_string())
    }

    fn register_issue_ids_from_stdout(&self, stdout: &str) {
        let bytes = stdout.as_bytes();
        let mut index = 0;
        while let Some(offset) = stdout[index..].find("atelier-") {
            let start = index + offset;
            let mut end = start;
            while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'-') {
                end += 1;
            }
            self.register_issue_id(stdout[start..end].to_string());
            index = end;
        }
    }

    fn register_issue_ids_from_state(&self) {
        let issues_dir = self.temp_dir.path().join(".atelier/issues");
        let Ok(entries) = std::fs::read_dir(issues_dir) else {
            return;
        };
        let mut ids = entries
            .filter_map(Result::ok)
            .filter_map(|entry| entry.path().file_stem()?.to_str().map(str::to_owned))
            .filter(|id| is_record_id(id))
            .collect::<Vec<_>>();
        ids.sort();
        for id in ids {
            self.register_issue_id(id);
        }
    }

    fn register_issue_id(&self, id: String) {
        if !is_record_id(&id) {
            return;
        }
        if !self
            .temp_dir
            .path()
            .join(".atelier/issues")
            .join(format!("{id}.md"))
            .exists()
        {
            return;
        }
        let mut ids = self.issue_ids.lock().unwrap();
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
}

fn first_record_id(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut index = 0;
    while let Some(offset) = value[index..].find("atelier-") {
        let start = index + offset;
        let mut end = start;
        while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'-') {
            end += 1;
        }
        let candidate = &value[start..end];
        if is_record_id(candidate) {
            return Some(candidate.to_string());
        }
        index = end;
    }
    None
}

fn is_record_id(value: &str) -> bool {
    let Some((slug, suffix)) = value.rsplit_once('-') else {
        return false;
    };
    !slug.is_empty()
        && suffix.len() == 4
        && slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
}

fn command_offset<T: AsRef<str>>(args: &[T]) -> usize {
    args.iter()
        .position(|arg| !arg.as_ref().starts_with('-'))
        .unwrap_or(args.len())
}

fn issue_ref_position<T: AsRef<str>>(args: &[T], index: usize) -> bool {
    let offset = command_offset(args);
    if index <= offset {
        return false;
    }

    let rest = args
        .get(offset..)
        .unwrap_or_default()
        .iter()
        .map(|arg| arg.as_ref())
        .collect::<Vec<_>>();
    match rest.as_slice() {
        ["show" | "update" | "close" | "reopen" | "delete" | "start" | "related", ..] => {
            index == offset + 1
        }
        ["label" | "unlabel" | "comment", ..] => index == offset + 1,
        ["block" | "unblock" | "relate" | "unrelate", ..] => {
            index == offset + 1 || index == offset + 2
        }
        ["subissue", ..] => index == offset + 1,
        ["session", "work", ..] => index == offset + 2,
        ["archive", "add" | "remove", ..] => index == offset + 2,
        ["milestone", "add" | "remove", ..] => index > offset + 2,
        ["issue", "show" | "update" | "close" | "reopen" | "delete" | "related" | "impact", ..] => {
            index == offset + 2
        }
        ["issue", "label" | "unlabel" | "comment", ..] => index == offset + 2,
        ["issue", "block" | "unblock" | "relate" | "unrelate", ..] => {
            index == offset + 2 || index == offset + 3
        }
        ["issue", "subissue", ..] => index == offset + 2,
        ["dep", "list", ..] => index == offset + 2,
        ["dep", "add" | "remove", ..] => index == offset + 2 || index == offset + 3,
        _ => false,
    }
}

/// Assert that `result.stdout` contains `expected`.
pub fn assert_stdout_contains(result: &CmdResult, expected: &str) {
    assert!(
        result.stdout_contains(expected),
        "expected stdout to contain {:?} but got:\n{}",
        expected,
        result.stdout,
    );
}

/// Assert that `result.stderr` contains `expected`.
pub fn assert_stderr_contains(result: &CmdResult, expected: &str) {
    assert!(
        result.stderr_contains(expected),
        "expected stderr to contain {:?} but got:\n{}",
        expected,
        result.stderr,
    );
}
