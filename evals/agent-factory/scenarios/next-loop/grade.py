"""Grade next-loop: one ready item taken to done with an independent reviewer.

State, read from main: the item's record says done and arrived in an Atelier
merge; a review file approves it and none of its reviewers is one of the
item's implementers; the suite passes on main's tree, both Done when examples
hold, and the tests fail against the original slug.py.

Calls, read from the transcript and every delegated thread's transcript
beside it (threads/): each `atelier` invocation a shell call runs, found in
what the shell executes (Codex's `bash -lc` unwrapped, quoted text and
heredoc bodies left out, command substitutions kept), so
`delegate "Run atelier review x"` runs none and `atelier merge; atelier
status` runs two. A call's refusals and errors are counted from Atelier's
own refusal and error text in its output, at most one per invocation, since
a shell, a pipeline or a command substitution can hide the exit status, as
in `delegate "Run `atelier review x`"`. Read the transcripts too.
"""
import json
import re
import shlex
import shutil
import subprocess
import sys
from pathlib import Path

ID = "slug-k3fa"
ATELIER = re.compile(r"(?:^|[\s;&|()`'\"])(?:[^\s'\"]*/)?atelier(?=$|[\s;&|)`'\"])")
HELP = re.compile(r"(?:^|[\s;&|()`'\"])(?:[^\s'\"]*/)?atelier\s+(?:\S+\s+)?(?:--help|-h|help)\b")
# Every refusal the binary prints ends its first sentence one of these ways
# (test_grade.RefusalTextTest drives each against the real binary).
REFUSED = re.compile(r"[Nn]othing changed\.|Nothing was merged\.|`atelier merge` wrote nothing|"
                     r"(?:Implementers|Reviewers) run `")
ERRORED = re.compile(r"^atelier: |^For more information, try '--help'|panicked at", re.M)
EXAMPLES = 'assert slugify("Hello,  World!") == "hello-world"; ' \
           'assert slugify("  --Rust & Go--  ") == "rust-go"'


def git(repo, *args):
    return subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True)


def front(text):
    """Front-matter fields; a block list becomes a list."""
    fields, key = {}, None
    for line in text.split("\n---", 1)[0].splitlines()[1:]:
        if line.startswith("- ") and key:
            fields[key] = (fields[key] or []) + [line[2:].strip()]
        elif ":" in line and not line.startswith(" "):
            key, value = (part.strip() for part in line.split(":", 1))
            fields[key] = value or None
    return fields


def calls(path, thread):
    """(thread, command, exit code, output) for each shell call, and the
    number of tool calls, from a Claude Code or Codex transcript."""
    found, tools, pending = [], 0, {}
    for line in path.read_text().splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        item = event.get("item") or {}
        if event.get("type") == "item.completed" and item.get("type") not in (
                None, "agent_message", "reasoning", "todo_list", "error"):
            tools += 1
            if item.get("type") == "command_execution":
                found.append((thread, item.get("command", ""), item.get("exit_code"),
                              item.get("aggregated_output", "")))
        for block in (event.get("message") or {}).get("content") or []:
            if not isinstance(block, dict):
                continue
            if block.get("type") == "tool_use":
                tools += 1
                if block.get("name") == "Bash":
                    pending[block.get("id")] = (block.get("input") or {}).get("command", "")
            if block.get("type") == "tool_result" and block.get("tool_use_id") in pending:
                content = block.get("content")
                if isinstance(content, list):
                    content = "\n".join(c.get("text", "") for c in content if isinstance(c, dict))
                content = content or ""
                code = re.match(r"Exit code (\d+)", content) if block.get("is_error") else None
                found.append((thread, pending.pop(block["tool_use_id"]),
                              int(code.group(1)) if code else 0, content))
    return found, tools


def executed(command):
    """The text of `command` that a shell runs as commands: a Codex
    `/bin/bash -lc '<script>'` unwrapped, heredoc bodies and quoted text
    dropped, and command substitutions inside double quotes kept."""
    wrapped = re.match(r"/bin/(?:ba)?sh -lc (.*)\Z", command, re.S)
    if wrapped:
        try:
            command = shlex.split(wrapped.group(1))[0]
        except (ValueError, IndexError):
            pass
    lines, kept = command.split("\n"), []
    while lines:
        line = lines.pop(0)
        kept.append(line)
        for tag in re.findall(r"<<-?\s*['\"]?(\w+)", line):
            while lines and lines.pop(0).strip() != tag:
                pass
    text, out, quote, i = "\n".join(kept), [], None, 0
    while i < len(text):
        c = text[i]
        if quote == "'":
            quote = None if c == "'" else quote
        elif c == "\\":
            i += 1
        elif quote == '"' and c == '"':
            quote = None
        elif quote == '"' and (c == "`" or text.startswith("$(", i)):
            end = text.find("`" if c == "`" else ")", i + 1)
            end = len(text) if end < 0 else end
            out.append(" " + text[i + (1 if c == "`" else 2):end] + " ")
            i = end
        elif quote is None and c in "'\"":
            quote = c
        elif quote is None:
            out.append(c)
        i += 1
    return "".join(out)


def tally(shell):
    """Each shell call of `shell` (thread, command, exit code, output) that
    runs `atelier`, with its invocations, help lookups, refusals and errors."""
    rows = []
    for thread, command, code, out in shell:
        runs = executed(command)
        count = len(ATELIER.findall(runs))
        if not count:
            continue
        refused = min(len(REFUSED.findall(out)), count)
        rows.append({"thread": thread, "command": command[:200], "exit": code,
                     "output": out[-600:], "invocations": count,
                     "help": len(HELP.findall(runs)), "refused": refused,
                     "errored": min(len(ERRORED.findall(out)), count - refused)})
    return rows


def main():
    work, transcript = Path(sys.argv[1]), Path(sys.argv[2])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    item = git(repo, "show", f"main:.atelier/items/{ID}/item.md").stdout
    fields = front(item) if item else {}
    implementers = fields.get("implementers") or []
    merges = git(repo, "log", "--merges", "--format=%B%x00", f"{setup_head}..main").stdout
    reviews = []
    listing = git(repo, "ls-tree", "--name-only", f"main:.atelier/items/{ID}/reviews/").stdout
    for name in listing.split():
        review = front(git(repo, "show", f"main:.atelier/items/{ID}/reviews/{name}").stdout)
        reviews.append({"file": name, "verdict": review.get("verdict"),
                        "reviewers": review.get("reviewers") or [],
                        "model": review.get("model")})

    tree = work / ".grade" / "main"
    shutil.rmtree(tree.parent, ignore_errors=True)
    tree.mkdir(parents=True)
    archive = subprocess.run(["git", "-C", str(repo), "archive", "main"], capture_output=True)
    subprocess.run(["tar", "-x", "-C", str(tree)], input=archive.stdout, check=True)

    def py(*args):
        return subprocess.run([sys.executable, *args], cwd=tree, capture_output=True,
                              text=True).returncode == 0

    suite = py("-m", "unittest", "-q")
    examples = py("-c", f"from slug import slugify; {EXAMPLES}")
    (tree / "slug.py").write_text(git(repo, "show", f"{setup_head}:slug.py").stdout)
    catches_old = not py("-m", "unittest", "-q")

    shell, tools = [], 0
    transcripts = [("main", transcript)] + [
        (p.stem, p) for p in sorted((transcript.parent / "threads").glob("*.jsonl"))]
    for thread, path in transcripts:
        found, count = calls(path, thread)
        shell += found
        tools += count
    rows = tally(shell)
    refused = sum(r["refused"] for r in rows)
    errored = sum(r["errored"] for r in rows)
    checks = {
        "status_on_main": fields.get("status"),
        "implementers": implementers,
        "merged_by_atelier": f"Atelier-Merge: {ID}" in merges,
        "reviews_on_main": reviews,
        "approved_by_non_implementer": any(
            r["verdict"] == "approve" and r["reviewers"]
            and not set(r["reviewers"]) & set(implementers) for r in reviews),
        "suite_passes_on_main": suite,
        "examples_hold_on_main": examples,
        "tests_fail_on_old_code": catches_old,
        "threads": len(transcripts),
        "atelier_calls": sum(r["invocations"] for r in rows),
        "help_lookups": sum(r["help"] for r in rows),
        "refused": refused,
        "errored": errored,
        "tool_calls": tools,
        "within_limit": refused + errored <= 1,
        "failed_calls": [r for r in rows if r["refused"] or r["errored"]],
    }
    problems = []
    if fields.get("status") != "done":
        problems.append(f"{ID} is {fields.get('status')} on main, not done")
    if not checks["merged_by_atelier"]:
        problems.append(f"no Atelier merge of {ID} on main")
    if not checks["approved_by_non_implementer"]:
        problems.append("no review on main approves it from a thread that never implemented it")
    if not (suite and examples and catches_old):
        problems.append("the change's tests do not pass on main, miss a Done when example, "
                        "or pass on the original code")
    notes = problems or ["done on main, approved by an independent thread, tests hold"]
    notes.append(f"{refused} refused and {errored} errored of "
                 f"{checks['atelier_calls']} atelier calls (limit: one failed call)")
    print(json.dumps({"pass": not problems, "checks": checks, "notes": notes}))


if __name__ == "__main__":
    main()
