# Validation and Review

[Agent Factory](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/SKILL.md) owns the process
for author proof, independent review, validation, finding disposition, and
handoff. Atelier supplies durable receipts and mechanical invariant checks.

Author proof must run after the last relevant change and exercise the specific
claim. Use focused unit or CLI integration tests for the changed boundary,
then the required repository checks. Do not claim a command or rejection path
works from a test that never exercises it.

Independent review is examination by someone who did not author the change.
Validation is evidence from executing checks or scenarios. They are separate
obligations; a test run does not substitute for independent approval.

Evidence records use `pass`, `fail`, `blocked`, and `not-applicable`, plus the
full checked commit. Record failure causes and proof limits in the receipt.
See [evidence](../../product/validation.md) and
[reviews and merge](../../product/reviews-and-merge.md) for the product contracts.

For prose-only changes, check local links, documented command shapes, and
`git diff --check`. For Rust changes, follow
[standards](standards.md) and [hazard scans](rust-quality-hazard-scans.md).
Summarize the checks and unresolved claims in the standard handoff.
