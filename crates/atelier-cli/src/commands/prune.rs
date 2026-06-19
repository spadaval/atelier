use anyhow::Result;

use crate::telemetry::{self, DiagnosticsPruneSummary};

pub fn run(apply: bool, retention_days: Option<u64>) -> Result<()> {
    let diagnostics = telemetry::prune_diagnostics_logs(retention_days, apply)?;

    println!("Prune");
    println!("=====");
    println!("Mode: {}", if apply { "apply" } else { "dry-run" });
    println!();

    print_diagnostics(&diagnostics, apply);
    println!();
    print_deferred_classes();

    if !apply {
        println!();
        println!("Next: atelier prune --apply");
    }

    Ok(())
}

fn print_diagnostics(summary: &DiagnosticsPruneSummary, apply: bool) {
    println!("Diagnostics Logs");
    println!("----------------");
    println!("Retention: {} day(s)", summary.retention_days);
    println!("Cutoff:    before {}", summary.cutoff);
    match &summary.commands_dir {
        Some(path) => println!("Path:      {}", path.display()),
        None => println!("Path:      disabled or unavailable"),
    }

    if summary.candidates.is_empty() {
        println!("Candidates: none");
        return;
    }

    println!("Candidates: {}", summary.candidates.len());
    for candidate in &summary.candidates {
        let status = if apply && summary.removed.contains(&candidate.path) {
            "removed"
        } else if apply {
            "failed"
        } else {
            "eligible"
        };
        println!(
            "  {status} diagnostics-log {} (date {}, {} bytes)",
            candidate.path.display(),
            candidate.date,
            candidate.size_bytes
        );
    }

    if !summary.failures.is_empty() {
        println!("Failures:");
        for (path, error) in &summary.failures {
            println!("  {} - {}", path.display(), error);
        }
    }
}

fn print_deferred_classes() {
    println!("Deferred Cleanup Classes");
    println!("------------------------");
    println!("  report-only evidence records - canonical retention contract pending");
    println!("  report-only issue and mission records - canonical retention contract pending");
    println!("  report-only activity sidecars - canonical retention contract pending");
    println!("  report-only branches and worktrees - Git safety contract pending");
}
