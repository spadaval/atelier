# `atelier review`

Primary role: Reviewer.

Primary question: "How do I operate the configured code review artifact linked
to an issue or epic?"

## Assessment

- Name: Correct. Review artifacts are distinct from evidence records and
  workflow transitions.
- Documentation: Needs strong boundary language: review commands manage native
  room or provider-backed review state; they do not close issues or satisfy
  proof by themselves.
- Design: Correct if provider-backed and native room behavior share the same
  operator contract. Merge authority belongs here, while Atelier workflow
  advancement remains `issue transition`.
- Output hierarchy: Issue/review artifact, provider or room mode, state,
  unresolved findings/comments, approval/change-request/merge readiness, then
  evidence or transition follow-up.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `review open --issue <id> --role <role> ...` | Worker, manager/orchestrator | Open a review artifact for an epic/issue branch; confirm an existing active artifact; prepare review handoff. | Artifact ID/URL, source/target branch, role author, and what state was linked. | Post context with `review comment`, run `review status`, or record evidence. | Help is parameter-heavy; role guides should teach when workflow effects open reviews automatically versus manual open. |
| `review link --issue <id> <pull_request>` | Manager/orchestrator | Attach an existing provider artifact; recover from manual PR creation; migrate an imported review link. | Normalized artifact ID, provider, issue link, branch match warnings. | Run `review status --issue <id>` or `issue transition <id> --options`. | Good recovery surface; should not be the default happy path when workflow effects own review creation. |
| `review status --issue <id>` | Reviewer | Check review readiness quickly; see unresolved findings; verify provider/room state before transition. | Approval state, change requests, unresolved comments/findings, merge state, stale approval signals. | Run `review comments --unresolved`, `review approve`, `review request-changes`, or `issue transition --options`. | Strong orientation command. It should distinguish review readiness from evidence sufficiency. |
| `review show --issue <id>` | Reviewer, validator | Inspect full linked review detail; audit room/provider state; support validation. | Timeline, artifact metadata, participants, comments/findings, approvals, merge data. | Record validation evidence or request changes. | Good drill-down from status/history. |
| `review merge --issue <id> --role <role>` | Reviewer, manager/orchestrator | Merge or confirm the linked review artifact; close a review room/provider artifact once review gates pass. | Merge readiness, blocking findings, stale approvals, branch expectations, result. | Run `issue transition <id> --options` or the relevant terminal transition. | Must keep saying it does not change Atelier workflow state. |
| `review comments --issue <id> [--unresolved]` | Worker, reviewer | See review feedback; check what remains to address; gather validation context. | Comment/finding IDs, severity, author role, resolved/unresolved state. | Reply with `review comment`, fix work, or run `review resolve <finding>`. | Good worker/reviewer bridge. |
| `review comment --issue <id> --role <role> [--finding] ...` | Worker, reviewer, manager/orchestrator | Leave review context; record a blocking/non-blocking native room finding; document response. | Created event/finding ID and whether it blocks merge. | Fix work, request changes, approve, or resolve finding. | `--finding` should be clearly room-mode oriented; provider behavior should be explicit. |
| `review approve --issue <id> --role <role>` | Reviewer, validator | Record review approval; signal validation acceptance for review artifact. | Approval event, stale approval caveats, merge readiness. | Run `review status`, `review merge`, or `issue transition --options`. | Approval is not proof by itself; evidence may still be required. |
| `review request-changes --issue <id> --role <role>` | Reviewer, validator | Block review until issues are addressed; classify review as not ready. | Change-request event, blocking status, requested changes body. | Worker fixes work, records evidence, then asks reviewer to re-check. | Good friction-point command; should point to unresolved comments/findings. |
| `review resolve --issue <id> <finding>` | Worker, reviewer | Mark a native room finding resolved after changes or agreement. | Finding ID, previous severity, resolver, remaining blockers. | Run `review comments --unresolved` or `review status`. | Should stay native-room specific and avoid implying provider comment resolution when unsupported. |

## Guidance Finding

The `review` family needs the strongest orientation of any active surface because
it overlaps with proof, workflow gates, branch integration, and provider state.
Role guides should route workers to comments/status, reviewers to status/show
and approve/request-changes, validators to evidence plus review status, and
managers to open/link/merge only when workflow effects did not already own it.
