# `atelier workflow`

Primary role: Admin.

Primary question: "How do I inspect raw workflow-policy diagnostics?"

## Assessment

- Name: Correct for a hidden diagnostic family.
- Documentation: Correctly hidden. Normal workflow users should use `lint`,
  `mission status`, and `issue transition --options`.
- Design: Acceptable as an admin/debug surface. It should not appear in normal
  role guides.
- Output hierarchy: Raw validator or policy detail, failed configuration path,
  replacement normal commands.

## Hidden Surface Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| hidden `workflow check` | Admin, workflow-policy implementer | Debug raw policy evaluation; inspect validator behavior during migration or tests. | Policy file, target record, transition/validator results, raw failure reason. | Fix workflow config, rerun `lint`, or use `issue transition --options` for normal readiness. | Good only if hidden. Normal operators should use `lint`, `mission status`, and `issue transition --options`. |
