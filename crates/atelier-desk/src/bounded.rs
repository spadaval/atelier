//! Commands the reads run for the views a host shows (`git log`,
//! `gh pr view`), each given a deadline, so a hung command fails one read
//! and holds no request open.

use anyhow::{bail, Context, Result};
use std::io::Read;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

/// Runs `command` with nothing on its standard input, and its output once
/// it exits, or an error once `limit` has passed, when it is killed. A
/// command that exits unsuccessfully is an error that names its standard
/// error.
pub fn run(command: &mut Command, limit: Duration) -> Result<Vec<u8>> {
    let what = format!("{:?}", command.get_program());
    let mut child = (command.stdin(Stdio::null()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("cannot run {what}"))?;
    // Each pipe is drained on a thread of its own, so a command that writes
    // more than a pipe holds is not stopped waiting on a full pipe.
    let drain = |pipe: Option<Box<dyn Read + Send>>| {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut bytes);
            }
            bytes
        })
    };
    let stdout = drain(
        child
            .stdout
            .take()
            .map(|p| Box::new(p) as Box<dyn Read + Send>),
    );
    let stderr = drain(
        child
            .stderr
            .take()
            .map(|p| Box::new(p) as Box<dyn Read + Send>),
    );
    let deadline = Instant::now() + limit;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            // A pipe a grandchild still holds open would keep its thread
            // reading, so neither is waited for.
            bail!("{what} did not finish within {} seconds", limit.as_secs());
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let output = Output {
        status,
        stdout: stdout.join().unwrap_or_default(),
        stderr: stderr.join().unwrap_or_default(),
    };
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("{what} failed: {}", error.trim());
    }
    Ok(output.stdout)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::process::Command;
    use std::time::{Duration, Instant};

    #[test]
    fn a_command_gives_its_output() {
        let out = run(
            Command::new("sh").args(["-c", "echo hi"]),
            Duration::from_secs(5),
        );
        assert_eq!(out.unwrap(), b"hi\n");
    }

    #[test]
    fn a_failed_command_names_its_error() {
        let failed = run(
            Command::new("sh").args(["-c", "echo no >&2; exit 3"]),
            Duration::from_secs(5),
        );
        assert!(format!("{:#}", failed.unwrap_err()).contains("no"));
    }

    #[test]
    fn a_command_past_its_limit_is_killed() {
        let started = Instant::now();
        let hung = run(Command::new("sleep").arg("30"), Duration::from_millis(200));
        assert!(format!("{:#}", hung.unwrap_err()).contains("did not finish"));
        assert!(started.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn output_larger_than_a_pipe_is_read_whole() {
        let out = run(
            Command::new("head").args(["-c", "1000000", "/dev/zero"]),
            Duration::from_secs(10),
        );
        assert_eq!(out.unwrap().len(), 1_000_000);
    }
}
