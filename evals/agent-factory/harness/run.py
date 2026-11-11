#!/usr/bin/env python3
"""Run Agent Factory scenarios against Claude Code or Codex and grade the result.

Each scenario directory holds:

- scenario.json: {"description": ..., "timeout_sec": ..., "max_budget_usd": ...}
- prompt.md: the task. "{{skill}}" is replaced by an instruction to use the
  skill in the skill arm and removed in the none arm.
- setup.sh: called with a fresh work directory; must create work/repo (the
  agent's working directory) and may create work/bin (prepended to PATH).
- grade.py: called with the work directory and the transcript path; prints
  one JSON object {"pass": bool, "checks": {...}, "notes": [...]}.

A scenario with "threads": true in scenario.json runs each agent as a thread,
as T3 Code does: it gets its own T3CODE_THREAD_ID, and work/bin/delegate
(harness/delegate.py) starts another agent of the same kind and model as a
new thread in the caller's directory, waits for it and prints its final
reply. The system prompt describes `delegate` in THREADS_NOTE, and nothing
else. Each such thread's transcript goes to threads/ in the run directory.
setup.sh is given the first agent's thread ID in T3CODE_THREAD_ID, so a
fixture can leave work that thread already holds.

--agent codex runs `codex exec` instead of Claude Code (the none arm only),
with HOME and CODEX_HOME in the work directory, so the host's ~/.codex
instructions and ~/.agents/skills are not loaded; the host's Codex sign-in
is linked in, never copied, and the link is removed when the run ends.

Arms: "none" runs with no skills, "skill" stages skills/agent-factory from a
git ref of this repository (without maintainers/, which describes the expected
behaviour) into the workspace's .claude/skills. Runs use --setting-sources
project, no MCP servers, and an allowlisted environment. Each run's init event
is checked: a run that loaded MCP servers, had the skill in the wrong arm,
exited non-zero, timed out or produced no result is recorded with
"pass": null and an "invalid" reason instead of a grade.

Claude Code loads instruction files (CLAUDE.md, AGENTS.md, .claude/CLAUDE.md,
.claude/rules/ and the like) from every directory above its working directory,
whatever --setting-sources says, so a run under $HOME sees ~/.claude/CLAUDE.md.
Each run's work directory is therefore made under --work-root, which must lie
outside $HOME (default /var/tmp/af-evals); transcripts and results still go to
--out. As a backstop, every run passes --settings with claudeMdExcludes naming
each instruction file path in the directories above its working directory.
Before a matrix, a canary run in the same setup on a cheap model lists the
instruction files it was given, and the harness aborts unless the canary
repository's own AGENTS.md is the only one.

The agent still runs as the user with permissions skipped and full access to
$HOME and the network; fixtures use local remotes, but nothing confines it.
"""

import argparse
import concurrent.futures
import datetime
import json
import os
import re
import secrets
import shutil
import signal
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
SCENARIOS = Path(__file__).resolve().parents[1] / "scenarios"
DEFAULT_OUT = Path.home() / ".cache" / "af-evals" / "runs"
DEFAULT_WORK_ROOT = Path("/var/tmp/af-evals")
SKILL_LINE = "Use the agent-factory skill for this task.\n\n"
# Passed through to the agent; everything else in the parent environment
# (including a parent Claude Code or T3 session's sockets and tokens) is dropped.
ENV_KEEP = ("HOME", "USER", "LOGNAME", "LANG", "LC_ALL", "SHELL", "PATH")
NO_MCP = ["--strict-mcp-config", "--mcp-config", '{"mcpServers":{}}']
# What Claude Code 2.1.318 loads as instructions from each directory above its
# working directory (.claude/rules is a directory of rule files). AGENTS.md
# loads where no CLAUDE.md does, so hiding a CLAUDE.md can expose one.
INSTRUCTION_NAMES = ("CLAUDE.md", "CLAUDE.local.md", "AGENTS.md",
                     ".claude/CLAUDE.md", ".claude/AGENTS.md", ".claude/rules/**")
CANARY_MODEL = {"claude": "haiku", "codex": "gpt-6-luna"}
THREADS_NOTE = (
    "Agents here run as threads, each with its own thread ID in "
    "T3CODE_THREAD_ID. To start another agent as a new thread, run "
    "`delegate \"<task>\"` in a shell: it starts a fresh agent of your kind in "
    "the current directory with that task as its only prompt, waits for it to "
    "finish, and prints its final reply."
)
CODEX_AUTH = Path.home() / ".codex" / "auth.json"
CANARY_PROMPT = (
    "This is an automated isolation check. Do not use any tools. List every "
    "instruction or memory file (such as CLAUDE.md, CLAUDE.local.md, AGENTS.md "
    "or a rules file) whose contents were loaded into your context, one per "
    "line, as its absolute path followed by ' | ' and the first Markdown "
    "heading in it. If no such file was loaded, reply NONE."
)


def inside(path: Path, root: Path) -> bool:
    path, root = path.resolve(), root.resolve()
    return path == root or root in path.parents


def instruction_excludes(cwd: Path) -> list:
    """claudeMdExcludes naming every instruction file path above cwd."""
    dirs = []
    for d in [*cwd.parents, *cwd.resolve().parents]:
        if d not in dirs:
            dirs.append(d)
    return [str(d / name) for d in dirs for name in INSTRUCTION_NAMES]


def claude_command(prompt: str, model: str, effort: str, budget, cwd: Path) -> list:
    settings = {"claudeMdExcludes": instruction_excludes(cwd)}
    return [
        "claude", "-p", prompt,
        "--model", model, "--effort", effort,
        "--setting-sources", "project",
        "--settings", json.dumps(settings),
        "--output-format", "stream-json", "--verbose",
        "--dangerously-skip-permissions",
        "--no-session-persistence",
        "--max-budget-usd", str(budget),
        *NO_MCP,
    ]


def codex_command(prompt: str, model: str, system=None) -> list:
    """codex exec at Codex's default effort, in the directory it is run in."""
    extra = ["-c", f"developer_instructions={json.dumps(system)}"] if system else []
    return ["codex", "exec", "--json", "--dangerously-bypass-approvals-and-sandbox",
            "--skip-git-repo-check", "-m", model, *extra, prompt]


def agent_command(agent: str, prompt: str, model: str, effort: str, budget,
                  cwd: Path, system=None) -> list:
    if agent == "codex":
        return codex_command(prompt, model, system)
    cmd = claude_command(prompt, model, effort, budget, cwd)
    return cmd + (["--append-system-prompt", system] if system else [])


def codex_home(work: Path) -> Path:
    """A scratch CODEX_HOME holding only a link to the host's sign-in."""
    home = work / "home" / ".codex"
    home.mkdir(parents=True, exist_ok=True)
    link = home / "auth.json"
    if not link.is_symlink():
        link.symlink_to(CODEX_AUTH)
    return home


def unlink_auth(work: Path) -> None:
    """Remove the sign-in from the scratch home, whether link or file."""
    (work / "home" / ".codex" / "auth.json").unlink(missing_ok=True)


def agent_env(work: Path, agent: str = "claude") -> dict:
    env = {k: os.environ[k] for k in ENV_KEEP if k in os.environ}
    if agent == "codex":
        env["CODEX_HOME"] = str(codex_home(work))
        env["HOME"] = str(work / "home")
    env["TERM"] = "dumb"
    env["ENABLE_CLAUDEAI_MCP_SERVERS"] = "false"
    # atelier's update notice names the host's own skill checkout and its age,
    # which is not the skill a run stages.
    env["ATELIER_NO_UPDATE_CHECK"] = "1"
    if (work / "bin").is_dir():
        env["PATH"] = f"{work / 'bin'}:{env['PATH']}"
    env["TMPDIR"] = str(work / "tmp")
    (work / "tmp").mkdir(exist_ok=True)
    return env


def run_claude(cmd: list, cwd: Path, env: dict, transcript: Path, timeout):
    """Run the agent into transcript; return its exit code or "timeout"."""
    with transcript.open("w") as out:
        # Own process group, so servers the agent starts die with the run.
        proc = subprocess.Popen(cmd, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                                stdout=out, stderr=subprocess.STDOUT,
                                start_new_session=True)
        try:
            exit_code = proc.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            exit_code = "timeout"
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        proc.wait()
    return exit_code


def stage_skill(ref: str, repo_dir: Path) -> str:
    sha = subprocess.run(
        ["git", "-C", str(REPO), "rev-parse", "--short", ref],
        check=True, capture_output=True, text=True,
    ).stdout.strip()
    dest = repo_dir / ".claude" / "skills"
    dest.mkdir(parents=True, exist_ok=True)
    archive = subprocess.run(
        ["git", "-C", str(REPO), "archive", ref, "skills/agent-factory"],
        check=True, capture_output=True,
    ).stdout
    subprocess.run(
        ["tar", "-x", "--strip-components=1", "-C", str(dest),
         "--exclude=agent-factory/maintainers"],
        input=archive, check=True,
    )
    exclude = repo_dir / ".git" / "info" / "exclude"
    if exclude.parent.exists():
        with exclude.open("a") as f:
            f.write("\n.claude/\n")
    return sha


def read_transcript(transcript: Path) -> dict:
    """The init event's isolation facts and the final result, if any. A
    Codex run's result is its last reply once its turn completed; its init
    names no MCP servers or skills because its scratch home has none."""
    info = {"init": None, "result": None}
    reply = None
    for line in transcript.read_text().splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if event.get("type") == "thread.started" and info["init"] is None:
            info["init"] = {"thread_id": event.get("thread_id"), "mcp_servers": [],
                            "agent_factory": False}
        item = event.get("item") or {}
        if event.get("type") == "item.completed" and item.get("type") == "agent_message":
            reply = item.get("text")
        if event.get("type") == "turn.completed":
            info["result"] = {"subtype": "success", "result": reply,
                              "usage": event.get("usage")}
        if event.get("type") in ("turn.failed", "error"):
            info["result"] = {"subtype": "error", "is_error": True, "result": reply,
                              "error": event}
        if event.get("type") == "system" and event.get("subtype") == "init" \
                and info["init"] is None:
            info["init"] = {
                "cwd": event.get("cwd"),
                "model": event.get("model"),
                "version": event.get("claude_code_version"),
                "mcp_servers": [m.get("name") for m in event.get("mcp_servers") or []],
                "agent_factory": any("agent-factory" in str(x)
                                     for x in event.get("skills") or []),
            }
        if event.get("type") == "result":
            info["result"] = event
    return info


def invalid_reason(arm: str, exit_code, info: dict):
    if exit_code != 0:
        return f"the agent exited {exit_code}"
    if info["init"] is None:
        return "no init event"
    if info["result"] is None or info["result"].get("is_error") \
            or info["result"].get("subtype") != "success":
        return "no successful result event"
    if info["init"]["mcp_servers"]:
        return f"MCP servers loaded: {info['init']['mcp_servers']}"
    if info["init"]["agent_factory"] != (arm == "skill"):
        return f"agent-factory present={info['init']['agent_factory']} in arm {arm}"
    return None


def canary_verdict(reply: str, repo: Path, token: str):
    """Whether the reply quotes the repository's own heading, and every path
    it names outside the repository. Any such path counts, whether or not it
    exists, so a mangled or made-up one costs a rerun rather than a leak."""
    text = re.sub(r"[*`]", " ", reply)
    own_seen = f"canary {token}" in text.lower()
    names = re.findall(r"(?<![\w/.~-])~?/[^\s|'\"<>()\[\],;]+", text)
    paths = {os.path.expanduser(n.rstrip(".:")) for n in names}
    return own_seen, sorted(p for p in paths if not inside(Path(p), repo))


def check_isolation(work_root: Path, effort: str, out_root: Path,
                    agent: str = "claude") -> None:
    """Exit unless a run in this setup sees only its repository's own
    instruction file, an AGENTS.md whose heading carries a random token
    that appears nowhere else the run can see."""
    stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    name = f"{stamp}-canary-{secrets.token_hex(4)}"
    token = secrets.token_hex(4)
    work = work_root / name
    repo = work / "repo"
    repo.mkdir(parents=True)
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    (repo / "AGENTS.md").write_text(
        f"# Isolation canary {token}\n\nThis repository's own guide.\n")
    transcript = out_root / f"{name}.jsonl"
    model = CANARY_MODEL[agent]
    exit_code = run_claude(
        agent_command(agent, CANARY_PROMPT, model, effort, 0.5, repo),
        repo, agent_env(work, agent), transcript, timeout=300)
    unlink_auth(work)
    reply = (read_transcript(transcript)["result"] or {}).get("result") or ""
    own_seen, foreign = canary_verdict(reply, repo, token)
    print(json.dumps({"canary": "pass" if exit_code == 0 and own_seen and not foreign
                      else "fail", "model": model, "exit": exit_code,
                      "foreign": foreign, "own_seen": own_seen,
                      "transcript": str(transcript)}), flush=True)
    if exit_code == 0 and own_seen and not foreign:
        shutil.rmtree(work, ignore_errors=True)
        return
    if exit_code != 0:
        why = f"the canary run exited {exit_code}"
    elif foreign:
        why = ("the run reports instruction files other than the scenario's "
               "own: " + ", ".join(foreign))
    else:
        why = ("the canary did not report its repository's own AGENTS.md, so "
               "the run's instructions are unconfirmed")
    sys.exit(f"Isolation canary failed: {why}.\nIts reply:\n{reply.strip()}\n"
             f"Transcript: {transcript}\nWork directory kept: {work}\n"
             "No scenario was run.")


def threads(work: Path, run_dir: Path, env: dict, agent: str, model: str,
            effort: str, meta: dict, thread: str) -> dict:
    """Install `delegate` and what it needs; return the env with this run's
    own thread ID, `thread`."""
    shutil.copy(Path(__file__).with_name("delegate.py"), work / "bin" / "delegate")
    (work / "bin" / "delegate").chmod(0o755)
    budget = meta.get("thread_budget_usd", meta.get("max_budget_usd", 5))
    (work / ".threads.json").write_text(json.dumps({
        "agent": agent, "env": env, "out": str(run_dir / "threads"),
        "timeout": meta.get("thread_timeout_sec", 1200),
        "argv": agent_command(agent, "{task}", model, effort, budget, work / "repo",
                              THREADS_NOTE),
    }))
    return {**env, "T3CODE_THREAD_ID": thread}


def run_one(scenario: str, arm: str, ref: str, model: str, effort: str,
            index: int, out_root: Path, work_root: Path, agent: str = "claude") -> dict:
    sdir = SCENARIOS / scenario
    meta = json.loads((sdir / "scenario.json").read_text())
    stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    name = f"{stamp}-{scenario}-{arm}-{agent}-{model}-{index}-{secrets.token_hex(2)}"
    run_dir = out_root / name
    work = work_root / name
    run_dir.mkdir(parents=True)
    work.mkdir(parents=True)
    (run_dir / "work").symlink_to(work)

    thread = f"thread-{secrets.token_hex(4)}"
    setup_env = {**os.environ, "T3CODE_THREAD_ID": thread} if meta.get("threads") else None
    setup = subprocess.run(["bash", str(sdir / "setup.sh"), str(work)],
                           env=setup_env, capture_output=True, text=True)
    if setup.returncode != 0:
        return {"scenario": scenario, "arm": arm, "model": model, "index": index,
                "pass": None, "invalid": "setup failed: " + setup.stderr[-500:],
                "run_dir": str(run_dir), "work_dir": str(work)}
    repo_dir = work / "repo"
    skill_sha = stage_skill(ref, repo_dir) if arm == "skill" else None

    prompt = (sdir / "prompt.md").read_text()
    prompt = prompt.replace("{{skill}}", SKILL_LINE if arm == "skill" else "")
    transcript = run_dir / "transcript.jsonl"
    env, system = agent_env(work, agent), None
    if meta.get("threads"):
        env, system = threads(work, run_dir, env, agent, model, effort, meta,
                              thread), THREADS_NOTE
    cmd = agent_command(agent, prompt, model, effort, meta.get("max_budget_usd", 5),
                        repo_dir, system)
    started = datetime.datetime.now()
    try:
        exit_code = run_claude(cmd, repo_dir, env, transcript,
                               timeout=meta.get("timeout_sec", 1200))
    finally:
        unlink_auth(work)
    seconds = (datetime.datetime.now() - started).total_seconds()

    info = read_transcript(transcript)
    (run_dir / "final.md").write_text((info["result"] or {}).get("result") or "")
    grade = subprocess.run(
        [sys.executable, "-I", str(sdir / "grade.py"), str(work), str(transcript)],
        capture_output=True, text=True,
    )
    try:
        verdict = json.loads(grade.stdout)
    except json.JSONDecodeError:
        verdict = {"pass": None, "error": grade.stdout + grade.stderr}
    invalid = invalid_reason(arm, exit_code, info)
    if invalid:
        verdict = {**verdict, "pass": None, "invalid": invalid}

    result = {
        "scenario": scenario, "arm": arm, "skill_ref": skill_sha,
        "agent": agent, "model": model, "effort": effort, "index": index,
        "exit": exit_code, "seconds": round(seconds), "run_dir": str(run_dir),
        "work_dir": str(work), "init": info["init"],
        **verdict,
    }
    (run_dir / "result.json").write_text(json.dumps(result, indent=1))
    return result


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument("scenarios", nargs="+")
    p.add_argument("--agent", choices=("claude", "codex"), default="claude")
    p.add_argument("--arms", default="none,skill")
    p.add_argument("--ref", default="HEAD", help="git ref of the skill to stage")
    p.add_argument("--model", default="opus")
    p.add_argument("--effort", default="high")
    p.add_argument("--runs", type=int, default=1)
    p.add_argument("--jobs", type=int, default=2)
    p.add_argument("--out", type=Path, default=DEFAULT_OUT,
                   help="where transcripts and results go")
    p.add_argument("--work-root", type=Path, default=DEFAULT_WORK_ROOT,
                   help="where work directories are made; must be outside $HOME")
    p.add_argument("--keep", action="store_true",
                   help="keep work directories (default: keep only on failure)")
    a = p.parse_args()
    if a.agent == "codex" and a.arms != "none":
        p.error("--agent codex runs the none arm only: pass --arms none")
    if inside(a.work_root, Path.home()):
        p.error(f"--work-root {a.work_root} is inside $HOME, where the user's "
                "own CLAUDE.md would load into every run")

    jobs = [(s, arm, i) for s in a.scenarios for arm in a.arms.split(",")
            for i in range(a.runs)]
    results_path = a.out / "results.jsonl"
    a.out.mkdir(parents=True, exist_ok=True)
    a.work_root.mkdir(parents=True, exist_ok=True)
    check_isolation(a.work_root, a.effort, a.out, a.agent)
    with concurrent.futures.ThreadPoolExecutor(a.jobs) as pool:
        futures = [pool.submit(run_one, s, arm, a.ref, a.model, a.effort, i,
                               a.out, a.work_root, a.agent)
                   for s, arm, i in jobs]
        for fut, (s, arm, i) in zip(futures, jobs):
            try:
                r = fut.result()
            except Exception as exc:  # record it; keep the other runs' results
                r = {"scenario": s, "arm": arm, "index": i, "pass": None,
                     "invalid": f"harness error: {exc!r}", "run_dir": ""}
            with results_path.open("a") as f:
                f.write(json.dumps(r) + "\n")
            if r.get("pass") is True and not a.keep and r.get("work_dir"):
                shutil.rmtree(r["work_dir"], ignore_errors=True)
                (Path(r["run_dir"]) / "work").unlink(missing_ok=True)
            print(json.dumps({k: r.get(k) for k in
                              ("scenario", "arm", "index", "pass", "invalid",
                               "exit", "seconds", "run_dir")}), flush=True)


if __name__ == "__main__":
    main()
