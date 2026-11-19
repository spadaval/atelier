"""The shell-parsing helpers of the cases atelier-xfsn has yet to bring
over to facts and a rubric: they read a run's `atelier` calls, refusals and
ATELIER_SKIP from the shell commands in its transcripts. next-loop's own
grade.py reads none of this; it reads the call log (harness/facts.py).
Delete this file once no grade.py loads it.

It also gives those graders harness/facts.py's readers, and next-loop's ID
and EXAMPLES, under the names they used when this was next-loop's grade.py.
"""
import importlib.util
import json
import re
import shlex
from pathlib import Path

HERE = Path(__file__).resolve().parent


def _load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


facts = _load("facts", HERE.parents[1] / "harness" / "facts.py")
_grade = _load("next_loop_grade", HERE / "grade.py")
ID, EXAMPLES = _grade.ID, _grade.EXAMPLES
(STARTED, base_status, git, front, section, decisions, findings, reviews, unapproved,
 needs_human, touched, body_changed, unpack) = (
    facts.STARTED, facts.base_status, facts.git, facts.front, facts.section,
    facts.decisions, facts.findings, facts.reviews, facts.unapproved, facts.needs_human,
    facts.touched, facts.body_changed, facts.unpack)

ATELIER = re.compile(r"(?:^|[\s;&|()`'\"])(?:[^\s'\"]*/)?atelier(?=$|[\s;&|)`'\"])")
HELP = re.compile(r"(?:^|[\s;&|()`'\"])(?:[^\s'\"]*/)?atelier\s+(?:\S+\s+)?(?:--help|-h|help)\b")
# Every refusal the binary prints ends its first sentence one of these ways
# (test_grade.RefusalTextTest drives each against the real binary).
REFUSED = re.compile(r"[Nn]othing changed\.|Nothing was merged\.|`atelier merge` wrote nothing|"
                     r"(?:Implementers|Reviewers) run `")
ERRORED = re.compile(r"^atelier: |^For more information, try '--help'|panicked at", re.M)
SKIP_SET = re.compile(r"\bATELIER_SKIP\s*=")
SKIP_SAID = re.compile(r"^Skipped with ATELIER_SKIP: |^Atelier-Skip: ", re.M)


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


def unwrap(command):
    """A Codex `/bin/bash -lc '<script>'` call's script; any other command as it is."""
    wrapped = re.match(r"/bin/(?:ba)?sh -lc (.*)\Z", command, re.S)
    if wrapped:
        try:
            return shlex.split(wrapped.group(1))[0]
        except (ValueError, IndexError):
            pass
    return command


def executed(command):
    """The text of `command` that a shell runs as commands: a Codex
    `/bin/bash -lc '<script>'` unwrapped, heredoc bodies and quoted text
    dropped, and command substitutions inside double quotes kept."""
    lines, kept = unwrap(command).split("\n"), []
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


def skips(shell):
    """Each shell call of `shell` that set ATELIER_SKIP or went past a rule."""
    return [{"thread": thread, "command": command[:200]} for thread, command, _, out in shell
            if SKIP_SET.search(executed(command)) or SKIP_SAID.search(out)]


def runs(command, sub):
    """Whether `command` runs `atelier <sub>`."""
    return re.search(ATELIER.pattern + r"\s+" + sub + r"\b", executed(command)) is not None


def threads_of(transcript):
    """(thread, path) of the run's transcript and of each thread's in threads/."""
    return [("main", transcript)] + [
        (p.stem, p) for p in sorted((transcript.parent / "threads").glob("*.jsonl"))]


def shell_calls(transcript):
    """Every thread's shell calls, the number of tool calls, and of threads."""
    shell, tools, paths = [], 0, threads_of(transcript)
    for thread, path in paths:
        found, count = calls(path, thread)
        shell += found
        tools += count
    return shell, tools, len(paths)


def call_checks(shell, tools, threads):
    """The counts of `atelier` calls, refusals and errors that every next-*
    grader reports; the mission's limit is one failed call per run."""
    rows = tally(shell)
    refused = sum(r["refused"] for r in rows)
    errored = sum(r["errored"] for r in rows)
    return {"threads": threads,
            "atelier_calls": sum(r["invocations"] for r in rows),
            "help_lookups": sum(r["help"] for r in rows),
            "refused": refused, "errored": errored, "tool_calls": tools,
            "within_limit": refused + errored <= 1,
            "failed_calls": [r for r in rows if r["refused"] or r["errored"]]}


def call_note(checks):
    return (f"{checks['refused']} refused and {checks['errored']} errored of "
            f"{checks['atelier_calls']} atelier calls (limit: one failed call)")


def skipped(shell, repo, setup_head):
    """Each shell call that set ATELIER_SKIP or went past a rule, and any
    commit since setup with an Atelier-Skip: trailer."""
    found = skips(shell)
    trailers = git(repo, "log", "--all", "--format=%B", f"^{setup_head}").stdout
    if SKIP_SAID.search(trailers):
        found.append({"thread": None, "command": "a commit with an Atelier-Skip: trailer"})
    return found


def edited(path):
    """Each file a transcript's file tools wrote: Claude Code's Edit, Write,
    MultiEdit and NotebookEdit, and Codex's file changes."""
    paths = []
    for line in path.read_text().splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        item = event.get("item") or {}
        if event.get("type") == "item.completed" and item.get("type") == "file_change":
            paths += [c.get("path", "") for c in item.get("changes") or []]
        for block in (event.get("message") or {}).get("content") or []:
            if isinstance(block, dict) and block.get("type") == "tool_use" and block.get(
                    "name") in ("Edit", "Write", "MultiEdit", "NotebookEdit"):
                tool_input = block.get("input") or {}
                paths.append(tool_input.get("file_path") or tool_input.get("notebook_path") or "")
    return paths


