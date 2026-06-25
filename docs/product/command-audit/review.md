# `atelier review`

Primary role: Reviewer.

Primary question: "How do I manage the configured review artifact for issue or
epic work?"

## Assessment

- Name: Correct. Review is a first-class workflow concern, but it should not
  decide lifecycle state by itself.
- Documentation: Visible in root help. Role guides should send operators here
  only when status, transition, or configured review output names a review
  action.
- Design: Mixed. The family has a clear job, but `open` currently asks for
  low-level review plumbing that should usually be derived from the issue,
  branch policy, and configured provider.
- Output hierarchy: Issue/owner, review artifact identifier or URL, provider
  mode, role source, action result, then `issue transition`.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `review open --title <title> --source-branch <branch> [--issue <id>]` | Reviewer | Open or confirm the active review artifact. | Needs refinement; too many required manual provider fields for routine use. |
| `review link <pull-request> [--issue <id>]` | Reviewer | Attach an existing review artifact. | Good as recovery/import. |
| `review status [--issue <id>]` | Reviewer | Inspect concise review state. | Good. |
| `review show [--issue <id>]` | Reviewer | Inspect review detail. | Good. |
| `review merge [--issue <id>]` | Manager/orchestrator | Merge or confirm the linked artifact without changing workflow state. | Advanced; should follow workflow guidance. |
| `review comments [--issue <id>] [--unresolved]` | Reviewer | Inspect live comments. | Good. |
| `review comment <body> [--issue <id>]` | Reviewer | Add a review comment or native finding. | Good. |
| `review approve [--issue <id>]` | Reviewer | Approve the artifact. | Good. |
| `review request-changes [--issue <id>]` | Reviewer | Request changes on the artifact. | Good. |
| `review resolve <finding> [--issue <id>]` | Reviewer | Resolve a native room finding. | Good. |

## Cutting Note

Refine `review open` before adding more provider-specific flags. Routine review
creation should use issue and branch context; fully manual branch/title/body
forms belong in recovery guidance if they remain necessary.

## Human Output Debt

Review output should make the review authority and state obvious before showing
provider details. Interactive color may distinguish approved, changes
requested, unresolved findings, mergeable, and blocked states, but the text must
remain complete without color. Comments and findings should be bounded by
default and route to focused commands for full discussion.
