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

/// A `git cat-file --batch` child that has gone must be reported, not end
/// atelier with SIGPIPE (atelier-bsyn). One fake `cat-file` exits at once, so
/// atelier's first request races its exit. The other answers one request,
/// then closes its input and exits, so the next request is always written
/// after the child is gone; that case failed with SIGPIPE before the fix.
#[cfg(unix)]
#[test]
fn a_cat_file_child_that_exits_is_reported_instead_of_ending_on_sigpipe() {
    use std::os::unix::{fs::PermissionsExt, process::ExitStatusExt};

    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, id, stderr) = run_atelier(
        dir.path(),
        &[
            "-q",
            "issue",
            "create",
            "Other branch work",
            "--body",
            "## Goal\n\nG.\n\n## Done when\n\nD.",
        ],
    );
    assert!(success, "create failed: {stderr}");
    commit_all(dir.path(), "records");
    let branch = format!("task/{}", id.trim());
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["branch", &branch])
        .status()
        .unwrap();
    assert!(status.success());

    let real = std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|dir| dir.join("git"))
        .find(|path| path.is_file())
        .expect("git on PATH");
    let real = real.display();
    let exits_at_once = "  exit 128 ;;";
    let answers_once = format!(
        "  IFS= read -r line\n  exec 0<&-\n  printf '%s\\n' \"$line\" | '{real}' \"$@\"\n  exit 128 ;;"
    );
    for cat_file in [exits_at_once, &answers_once] {
        let wrapper = tempdir().unwrap();
        let script = wrapper.path().join("git");
        fs::write(
            &script,
            format!(
                "#!/bin/sh\ncase \" $* \" in *' cat-file --batch '*)\n{cat_file}\nesac\nexec '{real}' \"$@\"\n"
            ),
        )
        .unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        let path = std::env::join_paths(
            std::iter::once(wrapper.path().to_path_buf())
                .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
        )
        .unwrap();

        let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
            .current_dir(dir.path())
            .env("PATH", path)
            .arg("status")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert_ne!(
            output.status.signal(),
            Some(libc::SIGPIPE),
            "{cat_file}\n{stdout}{stderr}"
        );
        assert!(output.status.success(), "{cat_file}\n{stdout}{stderr}");
        assert!(
            stdout.contains(&format!(
                "{branch}: unavailable — git cat-file --batch exited"
            )),
            "{cat_file}\n{stdout}{stderr}"
        );
    }
}
