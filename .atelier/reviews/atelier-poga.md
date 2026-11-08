---
schema: atelier.review
schema_version: 2
id: atelier-poga
title: finding on atelier-8v6s @bf8031df
target: atelier-8v6s
identity: planning-reviewer-sonnet
commit: bf8031df022117c069d0e92a86f9d96019eb2298
created_at: 2026-11-08T18:09:43.905102386Z
event:
  event: finding
  recommendation: fix_now
---

Question and Case lines contain literal backslash-escaped backticks (\`atelier land\`, \`main\`) that render as stray backslashes; use plain backticks.
