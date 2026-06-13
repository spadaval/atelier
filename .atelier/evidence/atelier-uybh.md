---
created_at: "2026-06-13T23:11:19.566361780+00:00"
id: "atelier-uybh"
data: "{\"agent_identity\":null,\"captured_at\":\"2026-06-13T23:11:19.454850179+00:00\",\"command\":\"cargo machete\",\"exit_code\":1,\"exit_status\":\"1\",\"follow_up_ids\":[],\"independence_level\":\"unspecified\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":60,\"summary\":\"Analyzing dependencies of crates in this directory...\\nDone!\\n\",\"truncated\":false},\"stdout\":{\"bytes\":538,\"summary\":\"cargo-machete found the following unused dependencies in this directory:\\natelier-tracker -- ./Cargo.toml:\\n\\tsignal-hook\\n\\nIf you believe cargo-machete has detected an unused dependency incorrectly,\\nyou can add the dependency to the list of dependencies to ignore in the\\n`[package.metadata.cargo-machete]` section of the appropriate Cargo.toml.\\nFor example:\\n\\n[package.metadata.cargo-machete]\\nignored = [\\\"prost\\\"]\\n\\nYou can also try running it with the `--with-metadata` flag for better accuracy,\\nthough this may modify your Cargo.lock files.\\n\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"proof_scope\":\"scoped to the attached target or summary\",\"residual_risks\":[],\"result\":\"fail\",\"spawn_error\":null,\"success\":false,\"target\":{\"id\":\"atelier-pa33\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-pa33"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "fail"
title: "cargo machete is installed and runs as the repo-supported unused-dependency scan; baseline found unused dependency signal-hook in Cargo.toml, which requires a follow-up owner outside this policy-decision slice."
updated_at: "2026-06-13T23:11:22.599830831+00:00"
---

cargo machete is installed and runs as the repo-supported unused-dependency scan; baseline found unused dependency signal-hook in Cargo.toml, which requires a follow-up owner outside this policy-decision slice.

Command: cargo machete
Exit status: 1

Stdout summary:
cargo-machete found the following unused dependencies in this directory:
atelier-tracker -- ./Cargo.toml:
	signal-hook

If you believe cargo-machete has detected an unused dependency incorrectly,
you can add the dependency to the list of dependencies to ignore in the
`[package.metadata.cargo-machete]` section of the appropriate Cargo.toml.
For example:

[package.metadata.cargo-machete]
ignored = ["prost"]

You can also try running it with the `--with-metadata` flag for better accuracy,
though this may modify your Cargo.lock files.

Stderr summary:
Analyzing dependencies of crates in this directory...
Done!
