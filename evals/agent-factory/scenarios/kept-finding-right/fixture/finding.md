## Finding 1: slugify keeps a leading hyphen, so the second Done when example fails
Recommendation: fix now
`slug.py` line 7 trims hyphens only at the end (`rstrip`), so
`slugify("  --Rust & Go--  ")` returns `-rust-go`, not `rust-go`. Trim both
ends (`strip`).
