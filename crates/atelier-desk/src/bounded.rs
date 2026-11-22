//! Commands the reads run for the views a host shows (`git log`,
//! `gh pr view`), and a mission's history read through the core, each given
//! a deadline, so a hung command fails one read and holds no request open.

use anyhow::{anyhow, bail, Context, Result};
use std::io::Read;
use std::process::{Command, Output, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Runs `command` with nothing on its standard input, and its output once
/// it exits and its output is read, or an error once `limit` has passed,
/// when it is killed. The limit covers reading its output too, so a process
/// it leaves behind holding a pipe open fails the read rather than holding
/// it. A command that exits unsuccessfully is an error that names its
/// standard error.
pub fn run(command: &mut Command, limit: Duration) -> Result<Vec<u8>> {
    let what = format!("{:?}", command.get_program());
    let deadline = Instant::now() + limit;
    let late = || format!("{what} did not finish within {} seconds", limit.as_secs());
    let mut child = (command.stdin(Stdio::null()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("cannot run {what}"))?;
    // Each pipe is drained on a thread of its own, so a command that writes
    // more than a pipe holds is not stopped waiting on a full pipe. A thread
    // still reading at the deadline is left to end with the pipe.
    fn drain(pipe: Option<impl Read + Send + 'static>) -> mpsc::Receiver<Vec<u8>> {
        let (send, receive) = mpsc::channel();
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut bytes);
            }
            let _ = send.send(bytes);
        });
        receive
    }
    let stdout = drain(child.stdout.take());
    let stderr = drain(child.stderr.take());
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            bail!(late());
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let read = |pipe: &mpsc::Receiver<Vec<u8>>| {
        let left = deadline.saturating_duration_since(Instant::now());
        pipe.recv_timeout(left).map_err(|_| anyhow!(late()))
    };
    let output = Output {
        status,
        stdout: read(&stdout)?,
        stderr: read(&stderr)?,
    };
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("{what} failed: {}", error.trim());
    }
    Ok(output.stdout)
}

/// What `work` gives, or an error once `limit` has passed. It runs on a
/// thread of its own, which is left to end: a blocked read cannot be
/// stopped, so work that can run long also takes the deadline itself.
pub fn within<T: Send + 'static>(
    limit: Duration,
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T> {
    let (send, receive) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = send.send(work());
    });
    receive.recv_timeout(limit).map_err(|error| match error {
        mpsc::RecvTimeoutError::Timeout => {
            anyhow!("the read did not finish within {} seconds", limit.as_secs())
        }
        mpsc::RecvTimeoutError::Disconnected => anyhow!("the read failed"),
    })
}

#[cfg(test)]
mod tests {
    use super::{run, within};
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
    fn a_pipe_held_open_after_the_command_exits_is_not_waited_for() {
        let started = Instant::now();
        let held = run(
            Command::new("sh").args(["-c", "sleep 30 & exit 0"]),
            Duration::from_millis(300),
        );
        assert!(format!("{:#}", held.unwrap_err()).contains("did not finish"));
        assert!(started.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn work_past_its_limit_is_not_waited_for() {
        assert_eq!(within(Duration::from_secs(5), || 7).unwrap(), 7);
        let started = Instant::now();
        let late = within(Duration::from_millis(100), || {
            std::thread::sleep(Duration::from_secs(10));
        });
        assert!(late.unwrap_err().to_string().contains("did not finish"));
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
