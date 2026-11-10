---
schema: atelier.review
schema_version: 2
id: atelier-vjqf
title: finding on atelier-pz1y @94aa3f83
target: atelier-pz1y
identity: sol-pz1y-r3
commit: 94aa3f83d442674ded786355c883e89266357e16
created_at: 2026-11-10T22:43:16.663154449Z
event:
  event: finding
  recommendation: fix_now
---

Low: next/crates/atelier-cli/src/check.rs:142: R6. A pre-push refusal for an unreadable checkout config bypasses the required refusal diagnostics. Reproduced with the real pre-push hook and a local bare remote: append base = 1 to the otherwise valid checkout config.toml, then git push origin main. Store::config returns through ? and the output is only atelier: <path>/.atelier/config.toml: line 4: unknown field base, followed by Git failing the push. There is no line saying this is not a rule and cannot be skipped, and no way out, contrary to Done when 6. Handle a config read/parse failure as an unread-record pre-push refusal, naming that file and telling the user to fix the checkout config before retrying. Add a real-Git scenario for this early refusal path.
