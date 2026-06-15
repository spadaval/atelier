use anyhow::Result;
use chrono::{DateTime, Days, SecondsFormat, Utc};
use serde_json::json;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

const SCHEMA: &str = "atelier.command_event";
const SCHEMA_VERSION: u8 = 1;
const DEFAULT_RETENTION_DAYS: u64 = 30;

pub fn diagnostics_enabled() -> bool {
    !env_disables("ATELIER_TELEMETRY") && !env_disables("ATELIER_DIAGNOSTICS")
}

pub fn record_command_event(
    command: &str,
    started_at: DateTime<Utc>,
    duration: Duration,
    exit_code: Option<i32>,
    success: bool,
) {
    if !diagnostics_enabled() {
        return;
    }

    if let Err(error) = write_command_event(command, started_at, duration, exit_code, success) {
        tracing::warn!("failed to write command diagnostics: {error}");
    }
}

pub fn slow_command_summary(days: u64, threshold_ms: u64) -> Result<Value> {
    let Some(root) = diagnostics_root() else {
        return Ok(empty_slow_summary(days, threshold_ms));
    };
    let commands_dir = root.join("commands");
    if !commands_dir.is_dir() {
        return Ok(empty_slow_summary(days, threshold_ms));
    }

    let cutoff = Utc::now()
        .date_naive()
        .checked_sub_days(Days::new(days))
        .unwrap_or_else(|| Utc::now().date_naive());
    let mut groups: BTreeMap<(Option<String>, String, String), SlowGroup> = BTreeMap::new();

    let mut paths = fs::read_dir(commands_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("ndjson"))
        .collect::<Vec<_>>();
    paths.sort();

    for path in paths {
        let Some(stem) = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(str::to_owned)
        else {
            continue;
        };
        let Ok(file_date) = chrono::NaiveDate::parse_from_str(&stem, "%Y-%m-%d") else {
            continue;
        };
        if file_date < cutoff {
            continue;
        }

        let content = fs::read_to_string(&path)?;
        for line in content.lines().filter(|line| !line.trim().is_empty()) {
            let Ok(event) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            if event["schema"].as_str() != Some(SCHEMA) {
                continue;
            }
            let Some(duration_ms) = event["duration_ms"].as_u64() else {
                continue;
            };
            if duration_ms < threshold_ms {
                continue;
            }
            let Some(command) = event["command"].as_str().filter(|value| !value.is_empty()) else {
                continue;
            };
            let Some(started_at) = event["started_at"].as_str() else {
                continue;
            };
            let bucket = started_at.get(..10).unwrap_or(&stem).to_string();
            let workspace_id = event["workspace_id"].as_str().map(str::to_owned);
            let result = event["result"].as_str().unwrap_or("unknown");
            let finished_at = event["finished_at"].as_str().unwrap_or(started_at);

            groups
                .entry((workspace_id, command.to_string(), bucket))
                .or_default()
                .push(duration_ms, result == "failure", started_at, finished_at);
        }
    }

    let rows = groups
        .into_iter()
        .map(|((workspace_id, command, bucket), group)| {
            group.to_json(workspace_id, command, bucket)
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "schema": "atelier.slow_commands",
        "schema_version": 1,
        "window_days": days,
        "threshold_ms": threshold_ms,
        "rows": rows,
    }))
}

fn write_command_event(
    command: &str,
    started_at: DateTime<Utc>,
    duration: Duration,
    exit_code: Option<i32>,
    success: bool,
) -> Result<()> {
    let Some(root) = diagnostics_root() else {
        return Ok(());
    };
    let commands_dir = root.join("commands");
    fs::create_dir_all(&commands_dir)?;

    let finished_at = Utc::now();
    prune_old_logs(&commands_dir, started_at);

    let workspace_root = workspace_root();
    let verbose = env_truthy("ATELIER_DIAGNOSTICS_VERBOSE");
    let workspace_id = workspace_root
        .as_deref()
        .and_then(|root| stable_workspace_id(root).ok());
    let state_path = workspace_root.as_deref().map(|root| {
        let layout = atelier_app::storage_layout::StorageLayout::new(root);
        if verbose {
            layout.canonical_dir().display().to_string()
        } else {
            layout
                .canonical_dir()
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(atelier_app::storage_layout::ATELIER_DIR)
                .to_string()
        }
    });
    let workspace_root_value = if verbose {
        workspace_root
            .as_deref()
            .map(|root| root.display().to_string())
    } else {
        None
    };

    let event = json!({
        "schema": SCHEMA,
        "schema_version": SCHEMA_VERSION,
        "event_id": event_id(started_at),
        "command": command,
        "argv_redacted": [],
        "argv_capture": "none",
        "started_at": started_at.to_rfc3339_opts(SecondsFormat::Millis, true),
        "finished_at": finished_at.to_rfc3339_opts(SecondsFormat::Millis, true),
        "duration_ms": duration.as_millis() as u64,
        "exit_code": exit_code,
        "result": if success { "success" } else { "failure" },
        "workspace_id": workspace_id,
        "workspace_root": workspace_root_value,
        "state_path": state_path,
        "agent_id": env::var("ATELIER_AGENT_ID").ok().filter(|value| !value.trim().is_empty()),
        "phase_timings": {},
        "redaction": {
            "mode": "default",
            "dropped_args": true,
            "path_policy": if verbose { "verbose" } else { "hash" },
            "notes": ["argv_omitted_by_default"]
        }
    });

    let path = commands_dir.join(format!("{}.ndjson", started_at.format("%Y-%m-%d")));
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    serde_json::to_writer(&mut file, &event)?;
    file.write_all(b"\n")?;
    Ok(())
}

fn empty_slow_summary(days: u64, threshold_ms: u64) -> Value {
    json!({
        "schema": "atelier.slow_commands",
        "schema_version": 1,
        "window_days": days,
        "threshold_ms": threshold_ms,
        "rows": [],
    })
}

#[derive(Default)]
struct SlowGroup {
    durations: Vec<u64>,
    failure_count: u64,
    first_started_at: Option<String>,
    last_finished_at: Option<String>,
}

impl SlowGroup {
    fn push(&mut self, duration_ms: u64, failed: bool, started_at: &str, finished_at: &str) {
        self.durations.push(duration_ms);
        if failed {
            self.failure_count += 1;
        }
        if self
            .first_started_at
            .as_deref()
            .map(|current| started_at < current)
            .unwrap_or(true)
        {
            self.first_started_at = Some(started_at.to_string());
        }
        if self
            .last_finished_at
            .as_deref()
            .map(|current| finished_at > current)
            .unwrap_or(true)
        {
            self.last_finished_at = Some(finished_at.to_string());
        }
    }

    fn to_json(mut self, workspace_id: Option<String>, command: String, bucket: String) -> Value {
        self.durations.sort_unstable();
        let count = self.durations.len() as u64;
        let sum = self.durations.iter().sum::<u64>();
        json!({
            "workspace_id": workspace_id,
            "command": command,
            "bucket": bucket,
            "count": count,
            "failure_count": self.failure_count,
            "min_duration_ms": self.durations.first().copied().unwrap_or(0),
            "max_duration_ms": self.durations.last().copied().unwrap_or(0),
            "mean_duration_ms": if count == 0 { 0.0 } else { sum as f64 / count as f64 },
            "p50_duration_ms": percentile_nearest_rank(&self.durations, 50),
            "p95_duration_ms": percentile_nearest_rank(&self.durations, 95),
            "first_started_at": self.first_started_at,
            "last_finished_at": self.last_finished_at,
        })
    }
}

fn percentile_nearest_rank(sorted: &[u64], percentile: u64) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let rank = ((percentile * sorted.len() as u64).div_ceil(100)).max(1);
    sorted[(rank - 1) as usize]
}

fn diagnostics_root() -> Option<PathBuf> {
    if let Ok(value) = env::var("ATELIER_DIAGNOSTICS_DIR") {
        let path = PathBuf::from(value);
        if path.is_absolute() {
            return Some(path);
        }
        tracing::warn!("ATELIER_DIAGNOSTICS_DIR must be absolute; command diagnostics disabled");
        return None;
    }

    if let Ok(value) = env::var("ATELIER_HOME") {
        return Some(PathBuf::from(value).join("diagnostics"));
    }

    if let Ok(value) = env::var("XDG_STATE_HOME") {
        return Some(PathBuf::from(value).join("atelier").join("diagnostics"));
    }

    env::var("HOME").ok().map(PathBuf::from).map(|home| {
        home.join(".local")
            .join("state")
            .join("atelier")
            .join("diagnostics")
    })
}

fn workspace_root() -> Option<PathBuf> {
    let mut current = env::current_dir().ok()?;
    loop {
        if current
            .join(atelier_app::storage_layout::ATELIER_DIR)
            .is_dir()
        {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

fn stable_workspace_id(root: &Path) -> Result<String> {
    let canonical = root.canonicalize()?;
    let mut hasher = Sha256::new();
    hasher.update(b"atelier.command_diagnostics.workspace.v1\0");
    hasher.update(canonical.to_string_lossy().as_bytes());
    let digest = format!("{:x}", hasher.finalize());
    Ok(digest[..16].to_string())
}

fn event_id(started_at: DateTime<Utc>) -> String {
    let timestamp = started_at.format("%Y%m%dT%H%M%S%6fZ");
    format!("{timestamp}-{}", std::process::id())
}

fn prune_old_logs(commands_dir: &Path, now: DateTime<Utc>) {
    let days = retention_days();
    let cutoff = now
        .date_naive()
        .checked_sub_days(Days::new(days))
        .unwrap_or_else(|| now.date_naive());
    let Ok(entries) = fs::read_dir(commands_dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("ndjson") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        let Ok(date) = chrono::NaiveDate::parse_from_str(stem, "%Y-%m-%d") else {
            continue;
        };
        if date < cutoff {
            let _ = fs::remove_file(path);
        }
    }
}

fn retention_days() -> u64 {
    match env::var("ATELIER_DIAGNOSTICS_RETENTION_DAYS") {
        Ok(value) => match value.parse::<u64>() {
            Ok(days) => days,
            Err(_) => {
                tracing::warn!("ATELIER_DIAGNOSTICS_RETENTION_DAYS must be a non-negative integer");
                DEFAULT_RETENTION_DAYS
            }
        },
        Err(_) => DEFAULT_RETENTION_DAYS,
    }
}

fn env_disables(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "0" | "false" | "off" | "disabled"
            )
        })
        .unwrap_or(false)
}

fn env_truthy(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "on"
            )
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn false_values_disable_diagnostics() {
        env::set_var("ATELIER_TELEMETRY", "off");
        assert!(!diagnostics_enabled());
        env::remove_var("ATELIER_TELEMETRY");
    }
}
