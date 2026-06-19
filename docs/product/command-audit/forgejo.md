# `atelier forgejo`

Primary role: Admin.

Primary question: "Is the Forgejo review provider configured so Atelier role
authors can operate review artifacts?"

## Assessment

- Name: Correct for provider-specific administration. It should remain outside
  normal review workflows.
- Documentation: Visible in root help under records, but role guidance should
  classify it as admin setup/preflight for provider mode.
- Design: Correct if it only configures and verifies provider integration. It
  should not leak into room-mode review workflows.
- Output hierarchy: Provider config source, required token env, role authors,
  permission/sudo checks, writes performed when provisioning, then `review
  status` or admin repair next steps.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `forgejo roles check` | Admin, reviewer preflight | Verify role authors exist; check repository permissions; diagnose provider review failures. | Host/repo, token env, configured role authors, existence, permission, sudo capability, failures. | Fix config/token/users or run `forgejo roles provision`. | Good admin check. Root help should not imply reviewers normally need provider administration. |
| `forgejo roles provision [--write-config]` | Admin | Create missing role users; grant repo access; persist role author mapping when explicitly requested. | Which users were created, permissions granted, config path changed, and any skipped writes. | Run `forgejo roles check`, then `review status --issue <id>`. | `--write-config` is the durable mutation; output should name `.atelier/config.toml` when changed. |

## Guidance Finding

Provider setup is a friction-point command. It should be discoverable from review
errors and `doctor`, but it should not appear in ordinary worker/reviewer loops
unless the failure is specifically Forgejo configuration.
