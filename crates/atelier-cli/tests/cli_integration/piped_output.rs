use super::*;
use std::io::{BufRead, BufReader, Read};
use std::process::Stdio;

/// A reader that stops after the first line, as `head -1` does, must not make
/// a multi-write view panic (atelier-5rs0). The output is several pipe
/// buffers long, so the writer is still printing when the reader closes.
#[test]
fn closing_stdout_after_the_first_line_ends_the_command_quietly() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = format!(
        "## Goal\n\n{}\n## Done when\n\nD.",
        "word word word word word word word word word word\n".repeat(1000)
    );
    let (success, id, stderr) = run_atelier(
        dir.path(),
        &["-q", "issue", "create", "Long piped view", "--body", &body],
    );
    assert!(success, "create failed: {stderr}");
    let id = id.trim();

    let mut child = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir.path())
        .args(["issue", "show", id, id, id, "--full"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let mut first = String::new();
    stdout.read_line(&mut first).unwrap();
    drop(stdout);
    let mut stderr = String::new();
    child
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut stderr)
        .unwrap();
    let status = child.wait().unwrap();

    assert!(first.starts_with(id), "{first}");
    assert!(!stderr.contains("panicked"), "{stderr}");
    assert!(stderr.is_empty(), "{stderr}");
    assert_ne!(status.code(), Some(101), "{status:?}");
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        assert!(
            status.success() || status.signal() == Some(libc::SIGPIPE),
            "{status:?}"
        );
    }
}
