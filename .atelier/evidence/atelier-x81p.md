---
created_at: "2026-06-15T18:49:11.226753889+00:00"
id: "atelier-x81p"
evidence_type: "validation"
captured_at: "2026-06-15T18:49:10.886554480+00:00"
command: "bash -lc 'rg \"clap::|derive\\(Parser\\)|derive\\(Subcommand\\)|println!|eprintln!\" crates/atelier-app/src -n; test $? -eq 1; rg \"atelier_app::(init|lint|health|man|export)::|render_doctor|println!\" crates/atelier-cli/src/commands/init.rs crates/atelier-cli/src/commands/agent_factory.rs crates/atelier-cli/src/commands/man.rs -n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-14z2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-14z2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "atelier-app has no raw Clap dependency or direct rendering after migrated app modules; CLI command modules render returned view models"
updated_at: "2026-06-15T18:49:14.652786342+00:00"
---

## Summary

atelier-app has no raw Clap dependency or direct rendering after migrated app modules; CLI command modules render returned view models

## Command

```console
bash -lc 'rg "clap::|derive\(Parser\)|derive\(Subcommand\)|println!|eprintln!" crates/atelier-app/src -n; test $? -eq 1; rg "atelier_app::(init|lint|health|man|export)::|render_doctor|println!" crates/atelier-cli/src/commands/init.rs crates/atelier-cli/src/commands/agent_factory.rs crates/atelier-cli/src/commands/man.rs -n'
```

Exit status: 0

## Stdout

Bytes: 24853
Truncated: yes

```text
crates/atelier-cli/src/commands/man.rs:90:    let stale_count = atelier_app::export::canonical_stale_entries(db, state_dir)?.len();
crates/atelier-cli/src/commands/man.rs:103:    println!("Atelier Man");
crates/atelier-cli/src/commands/man.rs:104:    println!("===========");
crates/atelier-cli/src/commands/man.rs:105:    println!("Role guides filter the existing command surface for the job at hand.");
crates/atelier-cli/src/commands/man.rs:106:    println!();
crates/atelier-cli/src/commands/man.rs:107:    println!("Roles");
crates/atelier-cli/src/commands/man.rs:108:    println!("-----");
crates/atelier-cli/src/commands/man.rs:109:    println!("  worker    Implement assigned or ready issue work.");
crates/atelier-cli/src/commands/man.rs:110:    println!("  reviewer  Check proof, review outputs, and validate transitions.");
crates/atelier-cli/src/commands/man.rs:111:    println!("  manager   Create, organize, and coordinate missions, plans, and work.");
crates/atelier-cli/src/commands/man.rs:112:    println!("  admin     Set up, repair, migrate, and maintain Atelier state.");
crates/atelier-cli/src/commands/man.rs:113:    println!();
crates/atelier-cli/src/commands/man.rs:114:    println!("Commands");
crates/atelier-cli/src/commands/man.rs:115:    println!("--------");
crates/atelier-cli/src/commands/man.rs:116:    println!("  atelier man worker");
crates/atelier-cli/src/commands/man.rs:117:    println!("  atelier man reviewer");
crates/atelier-cli/src/commands/man.rs:118:    println!("  atelier man manager");
crates/atelier-cli/src/commands/man.rs:119:    println!("  atelier man admin");
crates/atelier-cli/src/commands/man.rs:123:    println!("Atelier Man: {}", role.title());
crates/atelier-cli/src/commands/man.rs:124:    println!("{}", "=".repeat("Atelier Man: ".len() + role.title().len()));
crates/atelier-cli/src/commands/man.rs:132:    println!("\nCurrent State");
crates/atelier-cli/src/commands/man.rs:133:    println!("-------------");
crates/atelier-cli/src/commands/man.rs:136:            println!("  Repository: {}", snapshot.repo);
crates/atelier-cli/src/commands/man.rs:137:            println!("  Tracker:    {}", snapshot.tracker);
crates/atelier-cli/src/commands/man.rs:139:                println!("  Stale records: {}", snapshot.stale_count);
crates/atelier-cli/src/commands/man.rs:142:                Some(mission) => println!("  Active mission: {mission}"),
crates/atelier-cli/src/commands/man.rs:143:                None => println!("  Active mission: none"),
crates/atelier-cli/src/commands/man.rs:146:                println!("  Current work:   none");
crates/atelier-cli/src/commands/man.rs:148:                println!("  Current work:   {} issue(s)", snapshot.current_work.len());
crates/atelier-cli/src/commands/man.rs:150:                    println!("    {work}");
crates/atelier-cli/src/commands/man.rs:153:                    println!(
crates/atelier-cli/src/commands/man.rs:159:            println!("  Ready work:     {}", snapshot.ready_count);
crates/atelier-cli/src/commands/man.rs:162:            println!("  Tracker: unavailable");
crates/atelier-cli/src/commands/man.rs:164:                println!("  State error: {error}");
crates/atelier-cli/src/commands/man.rs:171:    println!("\nMost Relevant Commands");
crates/atelier-cli/src/commands/man.rs:172:    println!("----------------------");
crates/atelier-cli/src/commands/man.rs:179:                println!("  1. atelier status - Review the checkout's current-work set.");
crates/atelier-cli/src/commands/man.rs:180:                println!("  2. atelier evidence record --target issue/<id> --kind test --result pass -- <command> - Attach proof.");
crates/atelier-cli/src/commands/man.rs:181:                println!("  3. atelier issue transition <id> --options - Inspect allowed next workflow steps.");
crates/atelier-cli/src/commands/man.rs:183:                println!("  1. atelier issue list --ready - Find executable work.");
crates/atelier-cli/src/commands/man.rs:184:                println!("  2. atelier issue show <id> - Read the issue contract before editing."
```

## Stderr

Bytes: 0
Truncated: no

```text
```
