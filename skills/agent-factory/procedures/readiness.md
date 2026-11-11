# Readiness

Check these lines before reading any numbers. Each is a precondition for an
agent to find its way, change code and prove it works in this repository.
The scorecard shows a missing one weeks later; the checklist shows it on the
day. Each line names the measure that would show its failure: a Scorecard
measure from `docs/product.md`, or a number counted from `atelier log`. A
line whose failure shows in no measure is cut.

## How to Check

- Change nothing. Clone the base into a scratch directory, with `TMPDIR`
  under `$HOME` and outside any Git repository: `git clone --no-local
  --branch <base> <repo> "$TMPDIR/ready"`. Run every command there, and
  remove the clone when done.
- Never install over the host's own tools: `cargo install --root
  "$TMPDIR/bin"`, not a bare `cargo install`.
- Run one command at a time, with `-j 4` for cargo. Stop a command at ten
  minutes and record the line as could not test, saying why.
- Record each line as met, not met or could not test, with its evidence in
  a phrase: the command's result, or the file and line. There is no score.

## Easy to Navigate

1. **Each harness loads `AGENTS.md` and starts with `atelier status`.**
   Test: `AGENTS.md` exists. Where `.claude/` exists, `CLAUDE.md` beside it
   imports it with `@AGENTS.md` or is a symlink to it, and
   `.claude/settings.json` runs `atelier status` at `SessionStart`. Claude
   Code then reads the map whatever its version and Project instructions
   setting, except `managed-only`, which leaves out every project file.
   Without that `CLAUDE.md`, whether it reads `AGENTS.md` depends on the
   host's settings and on files above the checkout. Where `.codex/` exists,
   `.codex/hooks.json` runs `atelier status` too. Shows in: tool calls
   before an agent's first edit; Confusion (commands run by the wrong
   role).
2. **`AGENTS.md` names the commands that build, test, lint and launch the
   code agents are changing now,** or says which of them do not exist. Test:
   read it beside where the last month's work landed, `git log
   --since=30.days --name-only --format= -- . ':!.atelier' | cut -d/ -f1 |
   sort | uniq -c | sort -rn | head`. Shows in: tool calls before an agent's
   first edit; Confusion (errored calls).
3. **What `AGENTS.md` points to exists.** Every path and Markdown file it
   names in backticks, and every relative link in it and in the Markdown
   files it names or links, resolves. Test: the script below prints nothing. Shows in: tool calls
   before an agent's first edit.

## Easy and Reliable to Change

4. **A fresh clone builds with the documented commands alone.** Dependencies
   install from a committed lockfile, and no step the docs leave out is
   needed. Test: in the scratch clone, run the install and build commands
   that `AGENTS.md` names or links, in order. Shows in: time in `building`;
   tool calls before an agent's first edit.
5. **Format, lint and type check run from documented commands that change
   nothing, and pass on the base,** for each language the changed code is
   in. Test: run them in the scratch clone. Shows in: Review yield (style
   findings are not product defects); rounds.

## Easy to Validate

6. **The tests run from one documented command and pass on the base,**
   for each part of the code with its own tests. Test: run it in the
   scratch clone. Where it takes longer than ten minutes,
   observe the last CI run on the base that ran the tests instead. Shows in:
   where each defect was caught; defects found after merge.
7. **Every merge runs the checks of lines 5 and 6.** `.atelier/config.toml`
   lists each as a merge check, or names a CI workflow that runs it, and
   that workflow's last run on the base that ran them passed inside the 9
   minutes `atelier merge` waits. Test: read `config.toml` and the workflow,
   and `gh run list --workflow <file> --branch <base> --limit 20`. Shows in:
   where each defect was caught; defects found after merge; time in
   `approved`.
8. **A verify skill says how to launch the product and prove a feature
   works, and the launch works on the base.** Test: find it under
   `.claude/skills/` or `.agents/skills/`, and run its launch in the scratch
   clone until the product answers. Shows in: defects found after merge;
   where each defect was caught.
9. **Every bug names its cause.** Each item labelled `bug` and not dropped
   has `caused_by` set to the item or commit that introduced it. Test: the
   command below prints nothing. Shows in: defects found after merge and
   where each defect was caught, which leave out a bug with no cause and so
   read better than the code is.

Line 3's script, run from the repository's root:

```sh
names=$(grep -oE '`[^` <>:*@]+`' AGENTS.md | tr -d '`' | sed 's|^/||' | grep -E '/|\.md$')
for p in $names; do test -e "$p" || echo "AGENTS.md names missing $p"; done
links() { grep -oE '\]\([^) ]+' "$1" | cut -c3- | grep -v : | sed 's/#.*//' | grep .; }
for f in AGENTS.md $(links AGENTS.md | grep '\.md$') $(printf '%s\n' $names | grep '\.md$'); do
  test -f "$f" || continue
  links "$f" | while read -r p; do test -e "$(dirname "$f")/$p" || echo "$f links to missing $p"; done
done
```

Line 9's command:

```sh
grep -l '^- bug$' .atelier/items/*/item.md | xargs -r grep -l '^caused_by: null' | xargs -r grep -L '^status: dropped'
```

## A Repository's Own Lines

A repository's own lines live in `.atelier/readiness.md` in that
repository, in the same form: the precondition, its test, and the measure
that shows its failure. They are checked after the lines above. A line is
added or removed there only by a reviewed change: `atelier check` accepts
the file, and the pre-push hook counts it as code, so it reaches the base
only through an Atelier merge.
