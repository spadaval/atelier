# `atelier import-beads`

Primary role: Admin.

Primary question: "How do I import predecessor Beads data from an explicit
backup file?"

## Assessment

- Name: Correct for migration, but predecessor-specific.
- Documentation: Correctly hidden. Normal setup should use `init --import-beads`
  for the standard repo-local import path.
- Design: Acceptable as a one-off migration escape hatch.
- Output hierarchy: Input path, imported record counts, canonical output path,
  validation commands.

## Hidden Surface Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| hidden `atelier import-beads <input>` | Admin/migration operator | Import external predecessor backup; recover migration from a non-standard Beads path. | Input path, imported counts, generated IDs, skipped/conflicting records, canonical output path. | Run `atelier lint`, inspect imported records, then use normal issue/mission commands. | Good as hidden migration-only surface. Standard repo-local import belongs to `init --import-beads`. |
