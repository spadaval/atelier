---
created_at: "2026-06-15T18:37:15.728072721+00:00"
id: "atelier-5yhp"
evidence_type: "validation"
captured_at: "2026-06-15T18:37:15.357533988+00:00"
command: "bash -lc 'rg \"\\[\\[bin\\]\\]|name = \\\"atelier\\\"|path = \\\"src/main.rs\\\"\" crates/atelier-cli/Cargo.toml; rg \"derive\\(Parser\\)|derive\\(Subcommand\\)|fn init_tracing|telemetry::record_command_event|println!|canonical_export\\(atelier_app::Request\" crates/atelier-cli/src/main.rs crates/atelier-cli/src/commands/agent_factory.rs -n; rg \"CanonicalExportRequest|CanonicalExportView\" crates/atelier-app/src/export.rs -n'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-sclf"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 15516
    summary: "name = \"atelier\"\n[[bin]]\nname = \"atelier\"\npath = \"src/main.rs\"\ncrates/atelier-cli/src/commands/agent_factory.rs:162:        println!();\ncrates/atelier-cli/src/commands/agent_factory.rs:163:        println!(\ncrates/atelier-cli/src/commands/agent_factory.rs:168:        println!();\ncrates/atelier-cli/src/commands/agent_factory.rs:169:        println!(\ncrates/atelier-cli/src/commands/agent_factory.rs:463:    println!(\"{identity}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:464:    println!(\"{}\", \"=\".repeat(identity.len()));\ncrates/atelier-cli/src/commands/agent_factory.rs:465:    println!(\"Status:   {}\", object.status);\ncrates/atelier-cli/src/commands/agent_factory.rs:466:    println!(\ncrates/atelier-cli/src/commands/agent_factory.rs:470:    println!(\"Type:     {}\", object.issue_type);\ncrates/atelier-cli/src/commands/agent_factory.rs:471:    println!(\"Priority: {}\", object.priority);\ncrates/atelier-cli/src/commands/agent_factory.rs:472:    println!(\ncrates/atelier-cli/src/commands/agent_factory.rs:476:    println!(\ncrates/atelier-cli/src/commands/agent_factory.rs:481:        println!(\"Closed:   {}\", format_human_datetime_str(closed_at));\ncrates/atelier-cli/src/commands/agent_factory.rs:484:        println!(\"Owner:    {owner}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:487:        println!(\"Assignee: {assignee}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:490:        println!(\"Labels:   {}\", object.labels.join(\", \"));\ncrates/atelier-cli/src/commands/agent_factory.rs:493:        println!(\"File:     {}\", path.display());\ncrates/atelier-cli/src/commands/agent_factory.rs:496:        println!();\ncrates/atelier-cli/src/commands/agent_factory.rs:497:        println!(\"Tracker Degraded\");\ncrates/atelier-cli/src/commands/agent_factory.rs:498:        println!(\"----------------\");\ncrates/atelier-cli/src/commands/agent_factory.rs:499:        println!(\"{degraded}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:500:        println!(\"Fallback: showing the last valid local projection for orientation only.\");\ncrates/atelier-cli/src/commands/agent_factory.rs:501:        println!(\"Next: atelier lint {}\", object.id);\ncrates/atelier-cli/src/commands/agent_factory.rs:538:    println!(\"\\nTransition Readiness\");\ncrates/atelier-cli/src/commands/agent_factory.rs:539:    println!(\"--------------------\");\ncrates/atelier-cli/src/commands/agent_factory.rs:553:                println!(\"  {}: {} - {}\", option.name, state, summary);\ncrates/atelier-cli/src/commands/agent_factory.rs:554:                println!(\"    {}\", option.command);\ncrates/atelier-cli/src/commands/agent_factory.rs:558:            println!(\"  options: blocked - {error}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:561:    println!(\ncrates/atelier-cli/src/commands/agent_factory.rs:674:    println!(\"\\nHierarchy\");\ncrates/atelier-cli/src/commands/agent_factory.rs:675:    println!(\"---------\");\ncrates/atelier-cli/src/commands/agent_factory.rs:679:            println!(\ncrates/atelier-cli/src/commands/agent_factory.rs:687:        None => println!(\"Parent: (none)\"),\ncrates/atelier-cli/src/commands/agent_factory.rs:694:        println!(\"\\n{title}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:695:        println!(\"{}\", \"-\".repeat(title.len()));\ncrates/atelier-cli/src/commands/agent_factory.rs:696:        println!(\"{body}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:706:    println!(\"\\n{title}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:707:    println!(\"{}\", \"-\".repeat(title.len()));\ncrates/atelier-cli/src/commands/agent_factory.rs:710:        println!(\"(none)\");\ncrates/atelier-cli/src/commands/agent_factory.rs:713:            println!(\"  {row}\");\ncrates/atelier-cli/src/commands/agent_factory.rs:747:    println!(\"\\nSubissues\");\ncrates/atelier-cli/src/commands/agent_factory.rs:748:    println!(\"---------\");\ncrates/atelier-cli/src/commands/agent_factory.rs:750:        println!(\"(none)\");\ncrates/atelier-cli/src/commands/agent_factory.rs:754:    println!(\"{}\", subissue_summary(&subissues));\ncrates/atelier-cli/src/commands/agent_factory.rs:763:        "
    truncated: true
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sclf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "CLI owns parser tracing telemetry rendering and binary target while delegating export through atelier-app request/outcome/view API"
updated_at: "2026-06-15T18:37:19.017329582+00:00"
---

CLI owns parser tracing telemetry rendering and binary target while delegating export through atelier-app request/outcome/view API

Command: bash -lc 'rg "\[\[bin\]\]|name = \"atelier\"|path = \"src/main.rs\"" crates/atelier-cli/Cargo.toml; rg "derive\(Parser\)|derive\(Subcommand\)|fn init_tracing|telemetry::record_command_event|println!|canonical_export\(atelier_app::Request" crates/atelier-cli/src/main.rs crates/atelier-cli/src/commands/agent_factory.rs -n; rg "CanonicalExportRequest|CanonicalExportView" crates/atelier-app/src/export.rs -n'
Exit status: 0

Stdout summary (truncated):
name = "atelier"
[[bin]]
name = "atelier"
path = "src/main.rs"
crates/atelier-cli/src/commands/agent_factory.rs:162:        println!();
crates/atelier-cli/src/commands/agent_factory.rs:163:        println!(
crates/atelier-cli/src/commands/agent_factory.rs:168:        println!();
crates/atelier-cli/src/commands/agent_factory.rs:169:        println!(
crates/atelier-cli/src/commands/agent_factory.rs:463:    println!("{identity}");
crates/atelier-cli/src/commands/agent_factory.rs:464:    println!("{}", "=".repeat(identity.len()));
crates/atelier-cli/src/commands/agent_factory.rs:465:    println!("Status:   {}", object.status);
crates/atelier-cli/src/commands/agent_factory.rs:466:    println!(
crates/atelier-cli/src/commands/agent_factory.rs:470:    println!("Type:     {}", object.issue_type);
crates/atelier-cli/src/commands/agent_factory.rs:471:    println!("Priority: {}", object.priority);
crates/atelier-cli/src/commands/agent_factory.rs:472:    println!(
crates/atelier-cli/src/commands/agent_factory.rs:476:    println!(
crates/atelier-cli/src/commands/agent_factory.rs:481:        println!("Closed:   {}", format_human_datetime_str(closed_at));
crates/atelier-cli/src/commands/agent_factory.rs:484:        println!("Owner:    {owner}");
crates/atelier-cli/src/commands/agent_factory.rs:487:        println!("Assignee: {assignee}");
crates/atelier-cli/src/commands/agent_factory.rs:490:        println!("Labels:   {}", object.labels.join(", "));
crates/atelier-cli/src/commands/agent_factory.rs:493:        println!("File:     {}", path.display());
crates/atelier-cli/src/commands/agent_factory.rs:496:        println!();
crates/atelier-cli/src/commands/agent_factory.rs:497:        println!("Tracker Degraded");
crates/atelier-cli/src/commands/agent_factory.rs:498:        println!("----------------");
crates/atelier-cli/src/commands/agent_factory.rs:499:        println!("{degraded}");
crates/atelier-cli/src/commands/agent_factory.rs:500:        println!("Fallback: showing the last valid local projection for orientation only.");
crates/atelier-cli/src/commands/agent_factory.rs:501:        println!("Next: atelier lint {}", object.id);
crates/atelier-cli/src/commands/agent_factory.rs:538:    println!("\nTransition Readiness");
crates/atelier-cli/src/commands/agent_factory.rs:539:    println!("--------------------");
crates/atelier-cli/src/commands/agent_factory.rs:553:                println!("  {}: {} - {}", option.name, state, summary);
crates/atelier-cli/src/commands/agent_factory.rs:554:                println!("    {}", option.command);
crates/atelier-cli/src/commands/agent_factory.rs:558:            println!("  options: blocked - {error}");
crates/atelier-cli/src/commands/agent_factory.rs:561:    println!(
crates/atelier-cli/src/commands/agent_factory.rs:674:    println!("\nHierarchy");
crates/atelier-cli/src/commands/agent_factory.rs:675:    println!("---------");
crates/atelier-cli/src/commands/agent_factory.rs:679:            println!(
crates/atelier-cli/src/commands/agent_factory.rs:687:        None => println!("Parent: (none)"),
crates/atelier-cli/src/commands/agent_factory.rs:694:        println!("\n{title}");
crates/atelier-cli/src/commands/agent_factory.rs:695:        println!("{}", "-".repeat(title.len()));
crates/atelier-cli/src/commands/agent_factory.rs:696:        println!("{body}");
crates/atelier-cli/src/commands/agent_factory.rs:706:    println!("\n{title}");
crates/atelier-cli/src/commands/agent_factory.rs:707:    println!("{}", "-".repeat(title.len()));
crates/atelier-cli/src/commands/agent_factory.rs:710:        println!("(none)");
crates/atelier-cli/src/commands/agent_factory.rs:713:            println!("  {row}");
crates/atelier-cli/src/commands/agent_factory.rs:747:    println!("\nSubissues");
crates/atelier-cli/src/commands/agent_factory.rs:748:    println!("---------");
crates/atelier-cli/src/commands/agent_factory.rs:750:        println!("(none)");
crates/atelier-cli/src/commands/agent_factory.rs:754:    println!("{}", subissue_summary(&subissues));
crates/atelier-cli/src/commands/agent_factory.rs:763:

Stderr summary:
(none)

