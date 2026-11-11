//! The server stops cleanly on SIGTERM, as systemd stops it, and on
//! SIGINT: it exits with success and revokes the T3 Code session its token
//! belongs to. The `t3` command is a stand-in that logs each run; T3 Code's
//! MCP server is a port nothing listens on, which the Desk reports as
//! unreachable after it has issued its token.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::TempDir;

const T3: &str = r#"#!/bin/sh
dir=$(dirname "$0")
echo "$*" >> "$dir/calls"
if [ "$1 $2 $3" = "auth session issue" ]; then
  printf '{"sessionId":"session-1","token":"secret-1","method":"bearer-access-token","scopes":["orchestration:read"],"subject":"mcp-client","client":{"label":"atelier-desk","deviceType":"bot"},"expiresAt":"2099-01-01T00:00:00.000Z"}\n'
fi
"#;

fn calls(dir: &Path) -> String {
    fs::read_to_string(dir.join("calls")).unwrap_or_default()
}

/// Starts the server, waits for its token, sends `signal`, and gives its
/// exit status and the `t3` command's log.
fn stop_with(signal: &str) -> (std::process::ExitStatus, String) {
    let dir = TempDir::new().unwrap();
    let command = dir.path().join("t3");
    fs::write(&command, T3).unwrap();
    Command::new("chmod")
        .arg("+x")
        .arg(&command)
        .status()
        .unwrap();
    fs::create_dir(dir.path().join("repos")).unwrap();
    let config = dir.path().join("config.toml");
    let text = "caller = \"sumeet\"\nroots = [\"repos\"]\nlisten = [\"127.0.0.1:0\"]\n\
                [t3]\ncommand = \"t3\"\nurl = \"http://127.0.0.1:9/mcp\"\n";
    fs::write(&config, text).unwrap();
    let mut server = Command::new(env!("CARGO_BIN_EXE_atelier-desk"))
        .arg("--config")
        .arg(&config)
        .spawn()
        .unwrap();
    let start = Instant::now();
    while !calls(dir.path()).contains("auth session issue") {
        assert!(start.elapsed() < Duration::from_secs(20), "no token issued");
        std::thread::sleep(Duration::from_millis(50));
    }
    let sent = Command::new("kill")
        .args([&format!("-{signal}"), &server.id().to_string()])
        .status()
        .unwrap();
    assert!(sent.success());
    let start = Instant::now();
    let status = loop {
        if let Some(status) = server.try_wait().unwrap() {
            break status;
        }
        if start.elapsed() > Duration::from_secs(20) {
            server.kill().unwrap();
            panic!("the server did not stop on SIG{signal}");
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    (status, calls(dir.path()))
}

#[test]
fn sigterm_and_sigint_stop_the_server_and_revoke_its_token() {
    for signal in ["TERM", "INT"] {
        let (status, calls) = stop_with(signal);
        assert!(status.success(), "SIG{signal}: exit {status:?}");
        assert_eq!(
            calls.lines().last(),
            Some("auth session revoke session-1"),
            "SIG{signal}: the t3 command ran {calls:?}"
        );
    }
}
