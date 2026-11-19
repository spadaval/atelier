#!/usr/bin/env python3
"""Run Agent Factory scenarios against Claude Code or Codex and grade the result.

Each scenario directory holds:

- scenario.json: {"description": ..., "role": ..., "timeout_sec": ...,
  "max_budget_usd": ...}, where "role" is the role of the first agent: lead,
  implementer or reviewer.
- prompt.md: the task. "{{skill}}" is replaced by an instruction to use the
  skill in the skill arm and removed in the none arm.
- setup.sh: called with a fresh work directory; must create work/repo (the
  agent's working directory) and may create work/bin (prepended to PATH).
- grade.py: called with the work directory and the transcript path; prints
  one JSON object, {"facts": {<name>: {"value": ..., "pass": bool}}}: named
  values read from the records, the tree and the run's recorded `atelier`
  calls (harness/facts.py), "pass" on those the grade rests on.
- rubric.md, if the case has questions of meaning: each one, the role whose
  work it is about, whether it is graded or reported, and what a pass looks
  like (harness/judge.py).

Grading. After the run the harness runs grade.py, then puts each rubric
question to the setup's judge for the lab of the question's role, with the
rubric, the facts, the final reply, the review files and the diff the run
left, and not the arm (judged). The run passes when every fact its grade
rests on passes and every graded question is answered pass; a reported
answer is recorded and shown and changes no grade. A graded question the
judge cannot answer fails with its reason. Only an invalid run is graded
null. result.json keeps the facts, each answer with the line it rests on and
its judge, and each judge's seconds and tokens (judged).

Setups. A matrix runs in a setup: an agent (Claude Code or Codex), a model
and an effort for each role, and a judge for each lab its threads come from,
from another lab than that one (LABS). --setup names a checked-in one,
harness/setups/<name>.json; --agent, --model and --effort give a setup of
one model in every role instead, with DEFAULT_JUDGES. A run's first agent is
its scenario's role's, and a thread it starts for a role is that role's.

A scenario with "threads": true in scenario.json runs each agent as a thread,
as T3 Code does: it gets its own T3CODE_THREAD_ID, which setup.sh is given
too, so a fixture can leave work that thread already holds. In a setup of
one Codex model and effort a thread starts a subagent with Codex's own
spawn_agent, which the tool counts as a thread of its own and which
inherits its parent's model and effort; the harness writes its transcript
to threads/ from its session record, in the form `codex exec --json`
prints; the system prompt describes it in CODEX_THREADS_NOTE. In every
other setup, work/bin/delegate (harness/delegate.py) is the only way:
`delegate <role> "<task>"` starts that role's agent, model and effort as a
new thread in the caller's directory, waits for it and prints its final
reply, and writes its transcript to threads/. The system prompt describes it
in THREADS_NOTE. Either note says how to start a thread, and nothing about
when or why.

A Codex agent runs `codex exec` at its role's effort, with HOME and
CODEX_HOME in the work directory, so the host's ~/.codex instructions and
~/.agents/skills are not loaded: the first agent's CODEX_HOME is
work/home/.codex, and a role's that delegate starts is work/codex/<role>.
The host's Codex sign-in is linked into each, never copied, and the links
are removed when the run ends. Its scratch config.toml (codex_config) turns
off plugins and apps, which would otherwise download the account's own
skills and connectors.

The fixture's work/bin/atelier is moved to work/libexec/ and
harness/atelier_log.py put in its place, which logs every call. Each
result.json names its setup and gives the run's seconds, each thread's role,
agent, model, effort, seconds and tokens in and out, and its failed
`atelier` calls. harness/table.py prints a matrix's cells from
results.jsonl, and the matrix prints its own when it ends.

Arms: "none" runs with no skills, "skill" stages --skill-dir (default
skills/agent-factory) from a git ref of this repository, without maintainers/,
which describes the expected behaviour, under the name agent-factory: into
the workspace's .claude/skills for Claude Code, and into each scratch
CODEX_HOME's skills/ for Codex, where Codex 0.167 loads a user's skills,
for each agent of the setup the run can start. Runs use no MCP servers and
an allowlisted environment; Claude Code runs use --setting-sources project.
Each agent's init event, and for Codex its session record
($CODEX_HOME/sessions), is checked, every thread's included: a run whose
agent loaded MCP servers or skills from anywhere else, had the skill in the
wrong arm, was offered a way to start an agent the harness has not bounded,
or under Codex ran another model or effort than its role's, or whose first
agent exited non-zero, timed out or produced no result, is recorded with
"pass": null and an "invalid" reason instead of a grade.

Runs at once. A matrix makes its runs side by side, under a cap for each
kind: --cargo-jobs (default 3) on the runs whose agent builds Rust, the
replayed reviews ("cargo": true), since this host holds about three such
builds at once, and --jobs (default 10) on all others, whose agents mostly
wait on their models. Runs going at once share nothing a run writes: each
has its own work directory, with its own `delegate` lock and log, agent
homes, sign-in links and TMPDIR. Only the crates the replayed reviews build
from are shared, in --work-root's cargo-home, which cargo locks.

Within a run, at most two agents are alive: its first agent and one child.
A scenario without threads is one agent: Claude Code's agent-starting tools
(AGENT_TOOLS) are disallowed on its command line, and Codex's spawn_agent is
removed in its scratch config. In a scenario with threads, in a setup of one
Codex model and effort, the only way to start one is spawn_agent, capped by
agents.max_threads = 1, which counts every subagent of the session with a
turn running, grandchildren included, so a child can start none while it
runs, and a parent cannot start a second child or give the first a
follow-up while either runs; `delegate` is not installed. In every other
setup the only way, under Claude Code and Codex alike, is `delegate`:
Codex's spawn_agent is removed, and `delegate` holds the run's lock while
its child runs, so a second call waits for the first child to end, whichever
roles they take, and a child may not call it. When the first agent ends, any
child still running is killed.
Each run records when each of its agents ran (each turn of a Codex subagent,
from its own session record, since it idles between turns), and the matrix
prints the most agents running at once.

A run whose first agent, a thread it started, or a judge ended on its
provider's rate limit or a spent allowance (provider_limit) is invalid with
that reason, and not failed; a spawn_agent subagent's limit is read from its
session record. Running a matrix again into the same --out makes only its
runs that are missing there or invalid: one already graded in the same
setup (its roles and judges unchanged), from the same skill and binary, is
kept and counted in the table.

Claude Code loads instruction files (CLAUDE.md, AGENTS.md, .claude/CLAUDE.md,
.claude/rules/ and the like) from every directory above its working directory,
whatever --setting-sources says, so a run under $HOME sees ~/.claude/CLAUDE.md.
Each run's work directory is therefore made under --work-root, which must lie
outside $HOME (default /var/tmp/af-evals); transcripts and results still go to
--out. As a backstop, every run passes --settings with claudeMdExcludes naming
each instruction file path in the directories above its working directory.
Before a matrix, a canary run of each agent of the setup, on a cheap model
(CANARY), lists the instruction files it was given, and the harness aborts
unless the canary repository's own AGENTS.md is the only one, and its own
record shows no agent-starting tool and no foreign skill.

The agent runs as the user with permissions skipped and full access to
$HOME and the network; fixtures use local remotes, but nothing confines it,
except in a scenario with "confine": true in scenario.json. That one runs
its agent in harness/confine.py's sandbox: it sees only its own work
directory, the system, its program, the toolchain and its sign-in, reaches
only its model's API, and is not offered a web search or fetch tool of its
own (Claude Code's are disallowed, Codex's web_search is "disabled"). The
replay cases (harness/replay.py) confine, since they must not reach the
later fix; a run whose transcript shows the fix anyway is invalid
(replay.leak), and result.json lists what the proxy refused
(proxy_refused). A confined scenario cannot have threads. One with "cargo":
true has its crates fetched into --work-root's cargo-home before the run,
which then builds offline in its own target directory, and counts against
--cargo-jobs. Once a run is graded, every cargo target directory under its
work directory is deleted, whatever its grade (drop_build_output).
"""

import argparse
import concurrent.futures
import datetime
import json
import os
import re
import secrets
import shlex
import shutil
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import confine  # noqa: E402
import judge as judging  # noqa: E402
import replay  # noqa: E402
import table  # noqa: E402

REPO = Path(__file__).resolve().parents[3]
SCENARIOS = Path(__file__).resolve().parents[1] / "scenarios"
# Scenarios renamed on 2026-11-18, by the name a saved result gives them,
# and the name now; None for one deleted.
RENAMED = {f"next-{name}": name for name in (
    "loop", "loop-changes", "non-atelier-repo", "refused-push", "settle-within-authority",
    "vacuous-test-review", "costly-hardening")} | {"next-done-when-rewrite": None}
DEFAULT_OUT = Path.home() / ".cache" / "af-evals" / "runs"
DEFAULT_WORK_ROOT = Path("/var/tmp/af-evals")
DEFAULT_SKILL_DIR = "skills/agent-factory"
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
# Claude Code 2.1.322's tools that start or schedule another agent run. They
# are disallowed in every run, so `delegate` is the only way to start one.
AGENT_TOOLS = ("Agent", "Task", "Workflow", "RemoteTrigger", "CronCreate",
               "ScheduleWakeup")
# Codex skill roots, under CODEX_HOME, a run may load from: the staged skill
# and Codex's bundled system skills. Any other (an account's plugin) is foreign.
CODEX_SKILL_ROOTS = ("skills", "skills/.system")
# The canary's cheap model and effort for each agent.
CANARY = {"claude": ("haiku", "high"), "codex": ("gpt-6-luna", "low")}
SETUPS = Path(__file__).resolve().parent / "setups"
ROLES = ("lead", "implementer", "reviewer")
LABS = {"claude": "anthropic", "codex": "openai"}
# The judge a one-model setup from the command line names for its threads' lab.
DEFAULT_JUDGES = {
    "anthropic": {"agent": "codex", "model": "gpt-6.1-sol", "effort": "xhigh"},
    "openai": {"agent": "claude", "model": "claude-opus-5-5", "effort": "high"},
}
# What a thread is told about starting another: how, and nothing about when
# or why, since the scenario is what tests that. Both kinds of agent hear the
# same sentences, about the tool each starts threads with.
THREADS_LEAD = ("Agents here run as threads, each with its own thread ID in "
                "T3CODE_THREAD_ID. To start another agent as a new thread, ")
THREADS_NOTE = THREADS_LEAD + (
    "run `delegate <role> \"<task>\"` in a shell, where <role> is the role the "
    "thread takes: lead, implementer or reviewer. It starts a fresh agent for "
    "that role in the current directory with that task as its only prompt, "
    "waits for it to finish, and prints its final reply."
)
# In a setup of one Codex model and effort, Codex starts threads with its own
# spawn_agent, whose subagent inherits both, so it is told of that and not of
# delegate. fork_turns "none" keeps the parent's history out, as delegate does.
CODEX_THREADS_NOTE = THREADS_LEAD + (
    "call `spawn_agent` with fork_turns \"none\" and the task as its message. "
    "It starts a fresh agent on your model and effort in the current directory "
    "with that task as its only prompt, and its final reply comes to you when "
    "it finishes; `wait_agent` waits for it."
)
CODEX_AUTH = Path.home() / ".codex" / "auth.json"
# A run that ends on its provider's limit is invalid with this reason, and
# made again when its matrix is run again (provider_limit).
LIMIT_REASON = "rate limit or spent allowance"
# The words of a provider's limit in an error: a rate limit (HTTP 429), a
# subscription's usage limit, an API quota or an empty credit balance.
LIMIT_WORDS = re.compile(r"usage limit|rate[ _-]?limit|\b429\b|too many requests|quota|"
                         r"hit your limit|credit balance|billing", re.I)
# Claude Code's kinds of API error that mean a limit, on the message it
# puts in place of a reply.
LIMIT_ERRORS = ("rate_limit", "billing_error")
# The session ID at the end of a Codex session record's file name.
SESSION_ID = re.compile(r"[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}(?=\.jsonl$)")
# The default caps on runs at once, by kind: those that build Rust (the
# replayed reviews), and all others.
CARGO_JOBS, JOBS = 3, 10
JUDGE_TIMEOUT_SEC = 900
JUDGE_BUDGET_USD = 5
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


def claude_command(prompt: str, model: str, effort: str, budget, cwd: Path,
                   confined: bool = False) -> list:
    """Claude Code's command; a confined run is not offered its web tools
    (confine.WEB_TOOLS)."""
    settings = {"claudeMdExcludes": instruction_excludes(cwd)}
    disallowed = AGENT_TOOLS + (confine.WEB_TOOLS if confined else ())
    return [
        "claude", "-p", prompt,
        "--model", model, "--effort", effort,
        "--setting-sources", "project",
        "--settings", json.dumps(settings),
        "--output-format", "stream-json", "--verbose",
        "--dangerously-skip-permissions",
        "--no-session-persistence",
        "--max-budget-usd", str(budget),
        "--disallowedTools", ",".join(disallowed),
        *NO_MCP,
    ]


def codex_command(prompt: str, model: str, effort: str, system=None) -> list:
    """codex exec at `effort`, in the directory it is run in. Without
    model_reasoning_effort Codex runs its model's default, which is low for
    gpt-6.1-sol and gpt-6.2-sol."""
    extra = ["-c", f"developer_instructions={json.dumps(system)}"] if system else []
    return ["codex", "exec", "--json", "--dangerously-bypass-approvals-and-sandbox",
            "--skip-git-repo-check", "-m", model,
            "-c", f"model_reasoning_effort={json.dumps(effort)}", *extra, prompt]


def agent_command(agent: str, prompt: str, model: str, effort: str, budget,
                  cwd: Path, system=None, confined: bool = False) -> list:
    """An agent's command. A confined Codex run's web search is turned off
    in its scratch config (codex_config), not here."""
    if agent == "codex":
        return codex_command(prompt, model, effort, system)
    cmd = claude_command(prompt, model, effort, budget, cwd, confined)
    return cmd + (["--append-system-prompt", system] if system else [])


def judge_command(spec: dict, cwd: Path) -> list:
    """A judge's command, which reads its prompt from stdin and may use no
    tool: Claude Code with none, Codex in a read-only sandbox."""
    if spec["agent"] == "codex":
        return ["codex", "exec", "--json", "--skip-git-repo-check", "--sandbox", "read-only",
                "-m", spec["model"],
                "-c", f"model_reasoning_effort={json.dumps(spec['effort'])}", "-"]
    return ["claude", "-p", "--model", spec["model"], "--effort", spec["effort"],
            "--setting-sources", "project",
            "--settings", json.dumps({"claudeMdExcludes": instruction_excludes(cwd)}),
            "--output-format", "stream-json", "--verbose", "--no-session-persistence",
            "--max-budget-usd", str(JUDGE_BUDGET_USD), "--tools", "", *NO_MCP]


def run_judge(spec: dict, prompt: str, transcript: Path, work_root: Path):
    """Run one judge on `prompt` in an empty directory of its own, under
    work_root and outside the run's work directory, whose name carries no
    arm. Return its reply, or None, and its seconds, exit and tokens."""
    cwd = Path(tempfile.mkdtemp(prefix="judge-", dir=work_root))
    env = agent_env(cwd, spec["agent"])
    started = time.time()
    with transcript.open("w") as out:
        proc = subprocess.Popen(judge_command(spec, cwd), cwd=cwd, env=env,
                                stdin=subprocess.PIPE, stdout=out, stderr=subprocess.STDOUT,
                                text=True, start_new_session=True)
        try:
            proc.communicate(prompt, timeout=JUDGE_TIMEOUT_SEC)
            exit_code = proc.returncode
        except subprocess.TimeoutExpired:
            exit_code = "timeout"
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        proc.wait()
    ended = time.time()
    unlink_auth(cwd)
    info = read_transcript(transcript)
    tokens = info["tokens"]
    if spec["agent"] == "codex":
        tokens = codex_session(cwd / "home" / ".codex").get("tokens") or tokens
    shutil.rmtree(cwd, ignore_errors=True)
    result = info["result"] or {}
    reply = result.get("result") if exit_code == 0 and not result.get("is_error") else None
    return reply, {"seconds": round(ended - started), "exit": exit_code, "tokens": tokens,
                   "start": started, "end": ended}


def case_dir(name: str):
    """The directory of the scenario a result names, under the name it had
    when the run was saved (RENAMED); None for a scenario since deleted, or
    whose fixture was replaced under a new name, which no saved run's work
    directory fits."""
    name = RENAMED.get(name, name)
    path = None if name is None else SCENARIOS / name
    return path if path is not None and path.is_dir() else None


def grade(case: Path, work: Path, transcript: Path) -> dict:
    """What the case's grade.py prints, or {"error": ...} if it printed no JSON."""
    out = subprocess.run([sys.executable, "-I", str(case / "grade.py"), str(work),
                          str(transcript)], capture_output=True, text=True)
    try:
        return json.loads(out.stdout)
    except json.JSONDecodeError:
        return {"pass": None, "error": (out.stdout + out.stderr)[-2000:]}


def judged(case: Path, setup: dict, graded: dict, work: Path, final: str, run_dir: Path,
           work_root: Path, name: str = "judge", invalid=None) -> dict:
    """A run's grade from grade.py's facts and the judges' answers to the
    case's rubric, one judge for each lab whose role a question is about,
    each writing its transcript to <name>-<lab>.jsonl. An invalid run is
    not judged, and a run whose judge ended on its provider's limit
    (provider_limit) is made invalid. A grade.py that prints no facts is the
    case's grade as it printed it."""
    if "facts" not in graded:
        return graded
    facts = graded["facts"]
    rubric = judging.read_rubric(case)
    if invalid or not rubric:
        return {"facts": facts, "rubric": [], "judged": [],
                **({"pass": None} if invalid else judging.verdict(facts, []))}
    groups = judging.assign(rubric["questions"], setup, LABS)
    given = judging.material(work, final, facts, rubric["context"])
    answered, runs, limited = {}, [], None
    for lab, (spec, questions) in groups.items():
        transcript = run_dir / f"{name}-{lab}.jsonl"
        reply, info = run_judge(spec, judging.prompt(rubric["text"], questions, given),
                                transcript, work_root)
        answered.update(judging.parse_answers(reply, questions, given) if reply is not None
                        else judging.unanswered(questions, f"the judge ({spec['model']}) "
                                                f"exited {info['exit']} with no reply"))
        runs.append({"lab": lab, **spec, **info, "questions": [q["id"] for q in questions]})
        said = provider_limit(transcript)
        limited = limited or (said and f"{LIMIT_REASON}: the judge ({spec['model']}) "
                              f"ended on {said!r}")
    answers = judging.answers_list(rubric, groups, answered)
    return {"facts": facts, "rubric": answers, "judged": runs,
            **judging.verdict(facts, answers),
            **({"pass": None, "invalid": limited} if limited else {})}


def setup_problem(setup: dict):
    """Why a setup cannot run, or None. Each role names an agent, a model and
    an effort, and each lab a role's agent comes from has a judge from
    another lab, and no other lab has one."""
    roles = setup.get("roles") or {}
    if sorted(roles) != sorted(ROLES):
        return f"its roles are {sorted(roles)}, not {', '.join(ROLES)}"
    for name, spec in [*roles.items(), *(setup.get("judges") or {}).items()]:
        if spec.get("agent") not in LABS or not spec.get("model") or not spec.get("effort"):
            return (f"{name} needs an agent ({' or '.join(LABS)}), a model and an "
                    f"effort: {spec}")
    labs = {LABS[r["agent"]] for r in roles.values()}
    judges = setup.get("judges") or {}
    if set(judges) != labs:
        return (f"it names judges for {sorted(judges)}, and its threads come from "
                f"{sorted(labs)}: name one judge for each of those labs")
    for lab, judge in judges.items():
        if LABS[judge["agent"]] == lab:
            return f"the judge of {lab}'s threads is from {lab}; it must be from another lab"
    return None


def load_setup(name: str) -> dict:
    """The checked-in setup harness/setups/<name>.json."""
    path = SETUPS / f"{name}.json"
    if not path.is_file():
        known = ", ".join(sorted(p.stem for p in SETUPS.glob("*.json")))
        raise ValueError(f"no setup {name!r}; the setups are {known}")
    setup = {"name": name, **json.loads(path.read_text())}
    problem = setup_problem(setup)
    if problem:
        raise ValueError(f"setup {name} cannot run: {problem}")
    return setup


def one_model_setup(agent: str, model: str, effort: str) -> dict:
    """A setup of one model and effort in every role, for a model's first run."""
    role = {"agent": agent, "model": model, "effort": effort}
    lab = LABS[agent]
    return {"name": f"{model}-{effort}", "roles": {r: dict(role) for r in ROLES},
            "judges": {lab: dict(DEFAULT_JUDGES[lab])}}


def spawns(setup: dict) -> bool:
    """Whether threads start with Codex's spawn_agent: only in a setup of one
    Codex model and effort, since a subagent inherits its parent's. Every
    other setup starts them with `delegate <role>`."""
    first, *rest = setup["roles"].values()
    return first["agent"] == "codex" and all(r == first for r in rest)


def codex_config(threads: bool, confined: bool = False) -> str:
    """The scratch config.toml. Codex 0.167 runs gpt-6 models on multi-agent
    v2 whatever features.multi_agent says, and there disable_native_tools
    removes spawn_agent and the other collaboration tools. agents.max_threads
    caps the subagents with a turn running across the session, so with 1 a
    child can start no grandchild, and a parent no second child while the
    first runs; max_depth is v1's, set for a model on v1. A confined run
    has its web search turned off."""
    return (
        ('web_search = "disabled"\n\n' if confined else "") +
        "[features]\nplugins = false\nremote_plugin = false\napps = false\n\n"
        "[features.multi_agent_v2]\n"
        f"disable_native_tools = {'false' if threads else 'true'}\n\n"
        "[agents]\nmax_threads = 1\nmax_depth = 1\n")


def codex_home(home: Path, threads: bool = False, confined: bool = False) -> Path:
    """A scratch CODEX_HOME holding its config and a link to the host's sign-in."""
    home.mkdir(parents=True, exist_ok=True)
    (home / "config.toml").write_text(codex_config(threads, confined))
    link = home / "auth.json"
    if not link.is_symlink():
        link.symlink_to(CODEX_AUTH)
    return home


def role_home(work: Path, role: str) -> Path:
    """The CODEX_HOME of the threads `delegate` starts for a Codex role. The
    first agent's is work/home/.codex."""
    return work / "codex" / role


def unlink_auth(work: Path) -> None:
    """Remove the sign-in from every scratch home, whether link or file."""
    (work / "home" / ".codex" / "auth.json").unlink(missing_ok=True)
    for link in work.glob("codex/*/auth.json"):
        link.unlink(missing_ok=True)


def agent_env(work: Path, agent: str = "claude", threads: bool = False,
              home: Path = None, confined: bool = False) -> dict:
    """The environment of an agent of a run. A Codex agent's HOME is
    work/home, and its CODEX_HOME `home` (by default work/home/.codex), with
    spawn_agent only if `threads`."""
    env = {k: os.environ[k] for k in ENV_KEEP if k in os.environ}
    if agent == "codex":
        env["CODEX_HOME"] = str(codex_home(home or work / "home" / ".codex", threads,
                                           confined))
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


def stage_skill(ref: str, skills: Path, skill_dir: str = DEFAULT_SKILL_DIR) -> str:
    """Install skill_dir at ref as skills/agent-factory, without maintainers/."""
    skill_dir = skill_dir.strip("/")
    sha = subprocess.run(
        ["git", "-C", str(REPO), "rev-parse", "--short", ref],
        check=True, capture_output=True, text=True,
    ).stdout.strip()
    dest = skills / "agent-factory"
    dest.mkdir(parents=True, exist_ok=True)
    archive = subprocess.run(
        ["git", "-C", str(REPO), "archive", ref, skill_dir],
        check=True, capture_output=True,
    ).stdout
    subprocess.run(
        ["tar", "-x", f"--strip-components={skill_dir.count('/') + 1}", "-C", str(dest),
         f"--exclude={skill_dir}/maintainers"],
        input=archive, check=True,
    )
    return sha


def stage_for(agent: str, ref: str, work: Path, skill_dir: str, home: Path = None) -> str:
    """Stage the skill where `agent` loads a user's skills from: a Codex
    agent's CODEX_HOME `home` (by default the first agent's), or the
    workspace, which every Claude Code agent of the run shares."""
    if agent == "codex":
        return stage_skill(ref, (home or work / "home" / ".codex") / "skills", skill_dir)
    repo_dir = work / "repo"
    staged = (repo_dir / ".claude" / "skills" / "agent-factory").is_dir()
    sha = stage_skill(ref, repo_dir / ".claude" / "skills", skill_dir)
    exclude = repo_dir / ".git" / "info" / "exclude"
    if exclude.parent.exists() and not staged:
        with exclude.open("a") as f:
            f.write("\n.claude/\n")
    return sha


def read_transcript(transcript: Path) -> dict:
    """The init event's isolation facts and the final result, if any. A
    Codex run's result is its last reply once its turn completed; what it
    loaded is in its session record (codex_session), not in this stream."""
    info = {"init": None, "result": None}
    reply, usages = None, []
    for line in (transcript.read_text().splitlines() if transcript.exists() else []):
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
            usages.append(event.get("usage") or {})
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
                "agent_tools": [t for t in event.get("tools") or [] if t in AGENT_TOOLS],
            }
        if event.get("type") == "result":
            # Claude Code's result gives the whole session's usage.
            usages = [event.get("usage") or {}]
            info["result"] = event
    info["tokens"] = token_counts(usages)
    return info


def token_counts(usages: list) -> dict:
    """Tokens in, the cached part of them, and out, over Claude Code or Codex
    usage records. Codex's input_tokens counts its cached tokens; Claude
    Code's counts neither the tokens read from its cache nor those written."""
    tokens = {"in": 0, "cached_in": 0, "out": 0}
    for u in usages:
        if "cached_input_tokens" in u:
            tokens["in"] += u.get("input_tokens") or 0
            tokens["cached_in"] += u.get("cached_input_tokens") or 0
        else:
            cached = u.get("cache_read_input_tokens") or 0
            tokens["in"] += ((u.get("input_tokens") or 0)
                             + (u.get("cache_creation_input_tokens") or 0) + cached)
            tokens["cached_in"] += cached
        tokens["out"] += u.get("output_tokens") or 0
    return tokens


def rollouts(home: Path) -> list:
    """Each session record under a CODEX_HOME, as (meta, events, path)."""
    found = []
    for path in sorted((home / "sessions").glob("**/rollout-*.jsonl")):
        events = []
        for line in path.read_text().splitlines():
            try:
                events.append(json.loads(line))
            except json.JSONDecodeError:
                continue
        meta = next((e.get("payload") or {} for e in events
                     if e.get("type") == "session_meta"), {})
        found.append((meta, events, path))
    return found


def codex_given(items: list, home: Path) -> dict:
    """What Codex gave a session, from the items of its model input (a
    session record's payloads, or `codex debug prompt-input`): each skill
    and the root it loaded from, and whether spawn_agent was offered."""
    facts = {"skills": [], "foreign_skills": [], "agent_factory": False,
             "spawn_agent": False, "instructions": []}
    allowed = {str(home / r) for r in CODEX_SKILL_ROOTS}
    for item in items:
        if item.get("type") == "additional_tools":
            facts["spawn_agent"] |= '"spawn_agent"' in json.dumps(item.get("tools"))
        if item.get("type") != "message":
            continue
        for c in item.get("content") or []:
            text = (c.get("text") or "") if isinstance(c, dict) else ""
            given = re.match(r"# AGENTS\.md instructions for (\S+)\n", text)
            if given:
                facts["instructions"].append({"dir": given.group(1), "text": text})
            if "<skills_instructions>" not in text:
                continue
            roots = dict(re.findall(r"^- `(r\d+)` = `([^`]+)`", text, re.M))
            for name, file in re.findall(r"^- ([\w:.-]+): .*\(file: ([^)]+)\)\s*$",
                                         text, re.M):
                alias = file.split("/", 1)[0]
                root = roots.get(alias, os.path.dirname(os.path.dirname(file)))
                facts["skills"].append(name)
                if name == "agent-factory" and root == str(home / "skills"):
                    facts["agent_factory"] = True
                if root not in allowed:
                    facts["foreign_skills"].append(f"{name} ({root})")
    facts["skills"] = sorted(set(facts["skills"]))
    facts["foreign_skills"] = sorted(set(facts["foreign_skills"]))
    return facts


def codex_record(events: list, home: Path) -> dict:
    """What a session record says its session was given (codex_given), the
    model and effort its last turn ran at, and its tokens, from its last
    running total, or None where it has none. The record counts a turn
    that never completed, which `codex exec --json` gives no usage for, so
    a Codex thread's tokens are taken from here."""
    facts = {**codex_given([e.get("payload") or {} for e in events], home),
             "model": None, "effort": None}
    total = None
    for e in events:
        payload = e.get("payload") or {}
        if e.get("type") == "turn_context":
            facts["model"], facts["effort"] = payload.get("model"), payload.get("effort")
        if e.get("type") == "event_msg" and payload.get("type") == "token_count":
            total = (payload.get("info") or {}).get("total_token_usage") or total
    facts["tokens"] = token_counts([total]) if total else None
    return facts


def codex_session(home: Path, session: str = None) -> dict:
    """codex_record of the session `session`, or of the root session, under
    a CODEX_HOME."""
    for meta, events, _ in rollouts(home):
        if meta.get("id") == session if session else not meta.get("parent_thread_id"):
            return codex_record(events, home)
    return codex_record([], home)


def codex_thread_events(events: list) -> list:
    """A subagent's session record as the events `codex exec --json` prints
    for its own commands and replies, which the graders read."""
    out = []
    for e in events:
        payload = e.get("payload") or {}
        if e.get("type") != "event_msg" or payload.get("type") != "item_completed":
            continue
        item = payload.get("item") or {}
        kind = item.get("type")
        if kind == "CommandExecution":
            argv = item.get("command") or []
            command = argv if isinstance(argv, str) else (
                f"{argv[0]} {argv[1]} {shlex.quote(argv[2])}"
                if len(argv) == 3 and argv[1] in ("-lc", "-c") else shlex.join(argv))
            out.append({"type": "item.completed", "item": {
                "type": "command_execution", "command": command,
                "exit_code": item.get("exit_code"),
                "aggregated_output": item.get("aggregated_output") or ""}})
        elif kind == "AgentMessage":
            text = "".join(c.get("text", "") for c in item.get("content") or []
                           if isinstance(c, dict))
            out.append({"type": "item.completed",
                        "item": {"type": "agent_message", "text": text}})
        elif kind not in (None, "Reasoning", "UserMessage", "SubAgentActivity"):
            out.append({"type": "item.completed", "item": {"type": kind.lower()}})
    return out + [{"type": "turn.completed"}]


def epoch(stamp: str) -> float:
    return datetime.datetime.fromisoformat(stamp.replace("Z", "+00:00")).timestamp()


def codex_turns(events: list, ended: float) -> list:
    """Each turn of a session record as (start, end): from its task_started
    to the next task_complete or turn_aborted. A subagent idles between
    turns, when a parent's follow-up starts its next one, and an idle
    subagent runs nothing and does not count against agents.max_threads, so
    the span of its whole record is not when it was alive. A turn with no end
    in the record ends at `ended`, when the run's first agent ended."""
    turns, start = [], None
    for e in events:
        kind = (e.get("payload") or {}).get("type") if e.get("type") == "event_msg" else None
        if kind == "task_started" and start is None:
            start = epoch(e["timestamp"])
        elif kind in ("task_complete", "turn_aborted") and start is not None:
            turns.append((start, epoch(e["timestamp"])))
            start = None
    if start is not None:
        turns.append((start, max(ended, start)))
    return turns


def codex_threads(home: Path, run_dir: Path, ended: float, threads: list = None) -> list:
    """Write each subagent's transcript to threads/ and its raw record to
    sessions/; return each of its turns as (thread, start, end), from its
    own record (codex_turns). Each subagent's facts (thread_facts) are
    added to `threads`, as a thread with no role: spawn_agent names none."""
    agents = []
    for meta, events, path in rollouts(home):
        if not meta.get("parent_thread_id"):
            continue
        (run_dir / "threads").mkdir(exist_ok=True)
        (run_dir / "sessions").mkdir(exist_ok=True)
        thread = f"codex-{meta.get('id')}"
        (run_dir / "threads" / f"{thread}.jsonl").write_text(
            "".join(json.dumps(e) + "\n" for e in codex_thread_events(events)))
        shutil.copy(path, run_dir / "sessions" / path.name)
        turns = [{"thread": thread, "start": start, "end": end}
                 for start, end in codex_turns(events, ended)]
        agents += turns
        if threads is not None:
            facts = codex_record(events, home)
            facts.pop("instructions")
            threads.append({"thread": thread, "role": None, "agent": "codex",
                            "model": facts["model"], "effort": facts["effort"],
                            "seconds": round(sum(t["end"] - t["start"] for t in turns)),
                            "exit": None,
                            "tokens": facts.pop("tokens") or token_counts([]),
                            "init": {**facts, "mcp_servers": []}})
    return agents


def delegated_threads(work: Path) -> list:
    """Each thread delegate started, from its log, with its role and exit;
    any still running is killed, and ends now."""
    log, started, ended = work / ".agents.jsonl", {}, {}
    for line in (log.read_text().splitlines() if log.exists() else []):
        entry = json.loads(line)
        (started if entry["event"] == "start" else ended)[entry["thread"]] = entry
    agents = []
    for thread, entry in started.items():
        if thread not in ended:
            try:
                os.killpg(entry["pgid"], signal.SIGKILL)
            except ProcessLookupError:
                pass
        agents.append({"thread": thread, "role": entry.get("role"), "start": entry["time"],
                       "end": ended.get(thread, {}).get("time", time.time()),
                       "exit": ended.get(thread, {}).get("exit", "killed")})
    return agents


def thread_facts(entry: dict, setup: dict, run_dir: Path, work: Path) -> dict:
    """A thread `delegate` started for a role: the agent, model and effort its
    role gives it, its seconds, exit and tokens, and what its own record says
    it was given (its init event, or for Codex its session record, in the
    role's CODEX_HOME)."""
    spec = setup["roles"].get(entry.get("role")) or {}
    info = read_transcript(run_dir / "threads" / f"{entry['thread']}.jsonl")
    init, tokens = info["init"], info["tokens"]
    if spec.get("agent") == "codex" and init is not None:
        record = codex_session(role_home(work, entry["role"]), init.get("thread_id"))
        tokens = record.pop("tokens") or tokens
        record.pop("instructions")
        init = {**init, **record}
    return {"thread": entry["thread"], "role": entry.get("role"), **spec,
            "seconds": round(entry["end"] - entry["start"]), "exit": entry.get("exit"),
            "tokens": tokens, "init": init}


def peak(agents: list) -> int:
    """The most agents alive at once among (start, end) intervals."""
    edges = sorted([(a["start"], 1) for a in agents] + [(a["end"], -1) for a in agents],
                   key=lambda e: (e[0], e[1]))
    alive = top = 0
    for _, step in edges:
        alive += step
        top = max(top, alive)
    return top


def provider_limit(transcript: Path):
    """What the provider said, if the agent whose transcript this is ended
    on its rate limit or a spent allowance; else None. A limit is the last
    word only until the agent next gets a reply, so one it waited out or
    retried past is not counted. Claude Code reports a limit as a rejected
    rate_limit_event, as an API error of kind LIMIT_ERRORS in place of a
    reply, or in an error result; `codex exec` in an error or a failed turn;
    and a Codex session record, a spawn_agent subagent's, in an error
    event_msg."""
    said = None
    for line in (transcript.read_text().splitlines() if transcript.exists() else []):
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        kind = event.get("type")
        if kind == "event_msg":
            # A session record's event, read as `codex exec` would print it;
            # a message from the parent is not a reply.
            event = event.get("payload") or {}
            item = (event.get("item") or {}).get("type")
            kind = {"error": "error", "stream_error": "error"}.get(event.get("type")) or (
                "item.completed" if event.get("type") == "item_completed"
                and item != "UserMessage" else None)
        if kind == "rate_limit_event":
            info = event.get("rate_limit_info") or {}
            if info.get("status") == "rejected":
                said = f"Claude's {info.get('rateLimitType') or 'usage'} limit: rejected"
        elif kind == "assistant":
            if event.get("error") in LIMIT_ERRORS:
                content = (event.get("message") or {}).get("content") or []
                said = " ".join(c.get("text", "") for c in content
                                if isinstance(c, dict)).strip() or event["error"]
            else:
                said = None
        elif kind == "result":
            text = str(event.get("result") or "")
            if not event.get("is_error"):
                said = None
            elif LIMIT_WORDS.search(text):
                said = text
        elif kind in ("turn.completed", "item.completed"):
            said = None
        elif kind in ("turn.failed", "error"):
            error = event.get("error") if kind == "turn.failed" else event
            message = str((error or {}).get("message") or "")
            if LIMIT_WORDS.search(message):
                said = message
            elif kind == "turn.failed":
                said = None
    return said.strip()[:300] if said else None


def run_limit(transcript: Path, run_dir: Path):
    """Why a run is invalid because its first agent or a thread it started
    ended on its provider's limit (provider_limit), or None: a thread
    `delegate` started from its transcript, and a spawn_agent subagent from
    its session record, which keeps the errors its transcript in threads/
    (codex_thread_events) leaves out."""
    records = sorted((run_dir / "sessions").glob("rollout-*.jsonl"))
    agents = [("the first agent", transcript),
              *((f"thread {p.stem}", p) for p in sorted((run_dir / "threads").glob("*.jsonl"))),
              *((f"thread codex-{SESSION_ID.search(p.name).group(0)}"
                 if SESSION_ID.search(p.name) else f"subagent record {p.name}", p)
                for p in records)]
    for who, path in agents:
        said = provider_limit(path)
        if said:
            return f"{LIMIT_REASON}: {who} ended on {said!r}"
    return None


def invalid_reason(arm: str, exit_code, info: dict, threads: bool = False):
    """Why a run's first agent makes it invalid, or None. `threads` is
    whether it may start threads with spawn_agent."""
    if exit_code != 0:
        return f"the agent exited {exit_code}"
    if info["init"] is None:
        return "no init event"
    if info["result"] is None or info["result"].get("is_error") \
            or info["result"].get("subtype") != "success":
        return "no successful result event"
    return init_problem(arm, info["init"], threads)


def init_problem(arm: str, init: dict, spawn: bool = False):
    """Why what an agent's own record says it was given makes its run
    invalid, or None. Every agent of a run is checked, the threads it
    started included; `spawn` is whether it may have spawn_agent."""
    if init is None:
        return "no init event"
    if init.get("mcp_servers"):
        return f"MCP servers loaded: {init['mcp_servers']}"
    if init.get("agent_tools"):
        return f"agent-starting tools offered: {init['agent_tools']}"
    if init.get("foreign_skills"):
        return f"skills loaded from outside the scratch home: {init['foreign_skills']}"
    if init.get("spawn_agent") and not spawn:
        return "spawn_agent was offered where threads start with delegate, or none start"
    if init.get("agent_factory") != (arm == "skill"):
        return f"agent-factory present={init.get('agent_factory')} in arm {arm}"
    return None


def thread_problem(arm: str, thread: dict, spec: dict, spawn: bool = False):
    """Why a thread makes its run invalid, or None: init_problem, and for
    Codex, which records them, a model or effort other than its role's."""
    problem = init_problem(arm, thread["init"], spawn)
    if problem is None and spec["agent"] == "codex":
        ran = (thread["init"].get("model"), thread["init"].get("effort"))
        if ran != (spec["model"], spec["effort"]):
            problem = (f"it ran {ran[0]} at {ran[1]}, and its role is "
                       f"{spec['model']} at {spec['effort']}")
    if problem is None:
        return None
    return f"thread {thread['thread']} ({thread.get('role') or 'no role'}): {problem}"


def canary_verdict(reply: str, repo: Path, token: str):
    """Whether the reply quotes the repository's own heading, and every path
    it names outside the repository. Any such path counts, whether or not it
    exists, so a mangled or made-up one costs a rerun rather than a leak."""
    text = re.sub(r"[*`]", " ", reply)
    own_seen = f"canary {token}" in text.lower()
    names = re.findall(r"(?<![\w/.~-])~?/[^\s|'\"<>()\[\],;]+", text)
    paths = {os.path.expanduser(n.rstrip(".:")) for n in names}
    return own_seen, sorted(p for p in paths if not inside(Path(p), repo))


def record_verdict(given: list, repo: Path, token: str):
    """Whether a Codex session's record gave the model exactly the
    repository's own AGENTS.md, with the token, and each other instruction
    file it gave (codex_given's "instructions")."""
    own = [g for g in given if Path(g["dir"]) == repo and f"canary {token}" in g["text"].lower()]
    others = [f"{g['dir']}/AGENTS.md" for g in given if Path(g["dir"]) != repo]
    return len(own) == 1 and len(given) == 1, others


def check_isolation(work_root: Path, out_root: Path, agent: str = "claude") -> None:
    """Exit unless a run of `agent`, on its cheap model (CANARY), sees only
    its repository's own instruction file, an AGENTS.md whose heading
    carries a random token that appears nowhere else the run can see, and
    its record shows no agent-starting tool and no foreign skill. A matrix
    runs one for each agent its setup names."""
    stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    name = f"{stamp}-canary-{agent}-{secrets.token_hex(4)}"
    token = secrets.token_hex(4)
    work = work_root / name
    repo = work / "repo"
    repo.mkdir(parents=True)
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    (repo / "AGENTS.md").write_text(
        f"# Isolation canary {token}\n\nThis repository's own guide.\n")
    transcript = out_root / f"{name}.jsonl"
    model, effort = CANARY[agent]
    exit_code = run_claude(
        agent_command(agent, CANARY_PROMPT, model, effort, 0.5, repo),
        repo, agent_env(work, agent), transcript, timeout=300)
    unlink_auth(work)
    info = read_transcript(transcript)
    if agent == "codex" and info["init"] is not None:
        info["init"].update(codex_session(work / "home" / ".codex"))
    reply = (info["result"] or {}).get("result") or ""
    own_seen, foreign = canary_verdict(reply, repo, token)
    if agent == "codex" and info["init"] is not None:
        # Codex records the instruction files it gave the model, so read
        # them there rather than trust a cheap model to copy a heading back:
        # gpt-6-luna once answered "<repo>/AGENTS.md | NONE" with the file in
        # its input, and aborted a matrix. The reply still fails the canary
        # if it names a file outside the repository.
        own_seen, recorded = record_verdict(info["init"].pop("instructions"), repo, token)
        foreign = sorted(set(foreign) | set(recorded))
    unbounded = invalid_reason("none", 0, info) if exit_code == 0 else None
    ok = exit_code == 0 and own_seen and not foreign and not unbounded
    print(json.dumps({"canary": "pass" if ok else "fail", "model": model,
                      "exit": exit_code, "foreign": foreign, "own_seen": own_seen,
                      "invalid": unbounded, "transcript": str(transcript)}), flush=True)
    if ok:
        shutil.rmtree(work, ignore_errors=True)
        return
    if exit_code != 0:
        why = f"the canary run exited {exit_code}"
    elif unbounded:
        why = unbounded
    elif foreign:
        why = ("the run reports instruction files other than the scenario's "
               "own: " + ", ".join(foreign))
    else:
        why = ("the canary did not report its repository's own AGENTS.md, so "
               "the run's instructions are unconfirmed")
    sys.exit(f"Isolation canary of {agent} failed: {why}.\nIts reply:\n{reply.strip()}\n"
             f"Transcript: {transcript}\nWork directory kept: {work}\n"
             "No scenario was run.")


def threads(work: Path, run_dir: Path, env: dict, setup: dict, meta: dict,
            thread: str, stage=None) -> tuple:
    """Set up threads for a run; return its first agent's env, with its own
    thread ID `thread`, and its system note. In a setup of one Codex model
    and effort, Codex starts threads with spawn_agent. In any other,
    install `delegate` and what it needs: for each role, the command and
    environment of its agent, with a CODEX_HOME of its own for a Codex role
    (role_home). `stage(agent, home)` stages the skill for an agent, in the
    skill arm."""
    if spawns(setup):
        return {**env, "T3CODE_THREAD_ID": thread}, CODEX_THREADS_NOTE
    shutil.copy(Path(__file__).with_name("delegate.py"), work / "bin" / "delegate")
    (work / "bin" / "delegate").chmod(0o755)
    budget = meta.get("thread_budget_usd", meta.get("max_budget_usd", 5))
    roles = {}
    for role, spec in setup["roles"].items():
        home = role_home(work, role) if spec["agent"] == "codex" else None
        env_role = agent_env(work, spec["agent"], home=home)
        if stage:
            stage(spec["agent"], home)
        roles[role] = {**spec, "env": env_role,
                       "argv": agent_command(spec["agent"], "{task}", spec["model"],
                                             spec["effort"], budget, work / "repo",
                                             THREADS_NOTE)}
    (work / ".threads.json").write_text(json.dumps({
        "roles": roles, "out": str(run_dir / "threads"),
        "log": str(work / ".agents.jsonl"), "lock": str(work / ".threads.lock"),
        "timeout": meta.get("thread_timeout_sec", 1200),
    }))
    return {**env, "T3CODE_THREAD_ID": thread}, THREADS_NOTE


def log_atelier(work: Path) -> None:
    """Put atelier_log.py in place of the binary a fixture copied to
    work/bin/atelier, which moves to work/libexec/, so every call is logged."""
    binary = work / "bin" / "atelier"
    if not binary.is_file():
        return
    (work / "libexec").mkdir(exist_ok=True)
    binary.rename(work / "libexec" / "atelier")
    shutil.copy(Path(__file__).with_name("atelier_log.py"), binary)
    binary.chmod(0o755)


def atelier_calls(work: Path, first: str = None) -> list:
    """Each `atelier` call the run made, from atelier_log.py's log, with the
    thread of `threads` in result.json that made it ("by"): a Codex
    subagent's `codex-<CODEX_THREAD_ID>`, else its T3CODE_THREAD_ID, else
    the first agent's, `first`, in a run without threads."""
    log = work / ".atelier-calls.jsonl"
    calls = [json.loads(line) for line in
             (log.read_text().splitlines() if log.exists() else []) if line.strip()]
    for call in calls:
        env = call.get("env") or {}
        child, session = env.get("CODEX_THREAD_ID"), env.get("CODEX_SESSION_ID")
        call["by"] = (f"codex-{child}" if child and session and child != session
                      else env.get("T3CODE_THREAD_ID") or first)
    return calls


def run_one(scenario: str, arm: str, ref: str, setup: dict, index: int,
            out_root: Path, work_root: Path, skill_dir: str = DEFAULT_SKILL_DIR) -> dict:
    """Run a scenario once in a setup: its first agent is its role's, and a
    thread it starts for a role is that role's."""
    sdir = SCENARIOS / scenario
    meta = json.loads((sdir / "scenario.json").read_text())
    has_threads = bool(meta.get("threads"))
    confined = bool(meta.get("confine"))
    role = meta["role"]
    first = setup["roles"][role]
    agent, model, effort = first["agent"], first["model"], first["effort"]
    spawn = has_threads and spawns(setup)
    stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    name = f"{stamp}-{scenario}-{arm}-{setup['name']}-{index}-{secrets.token_hex(2)}"
    run_dir = out_root / name
    work = work_root / name
    run_dir.mkdir(parents=True)
    work.mkdir(parents=True)
    (run_dir / "work").symlink_to(work)
    named = {"scenario": scenario, "arm": arm, "setup": setup["name"], "role": role,
             "agent": agent, "model": model, "effort": effort, "index": index}

    thread = f"thread-{secrets.token_hex(4)}"
    setup_env = {**os.environ, "T3CODE_THREAD_ID": thread} if has_threads else None
    fixture = subprocess.run(["bash", str(sdir / "setup.sh"), str(work)],
                             env=setup_env, capture_output=True, text=True)
    if fixture.returncode != 0:
        return {**named, "pass": None, "invalid": "setup failed: " + fixture.stderr[-500:],
                "run_dir": str(run_dir), "work_dir": str(work)}
    log_atelier(work)
    judging.snapshot_refs(work)
    repo_dir = work / "repo"
    cargo_home = work_root / "cargo-home" if meta.get("cargo") else None
    if cargo_home:
        try:
            replay.cargo_fetch(repo_dir, cargo_home)
        except subprocess.CalledProcessError as exc:
            return {**named, "pass": None, "run_dir": str(run_dir), "work_dir": str(work),
                    "invalid": "cargo fetch failed: " + (exc.stderr or b"").decode()[-500:]}
    env, system = agent_env(work, agent, spawn, confined=confined), None
    staged = {}

    def stage(kind, home=None):
        if arm == "skill":
            staged[kind] = stage_for(kind, ref, work, skill_dir, home)

    stage(agent)
    prompt = (sdir / "prompt.md").read_text()
    prompt = prompt.replace("{{skill}}", SKILL_LINE if arm == "skill" else "")
    transcript = run_dir / "transcript.jsonl"
    if has_threads:
        env, system = threads(work, run_dir, env, setup, meta, thread, stage)
    cmd = agent_command(agent, prompt, model, effort, meta.get("max_budget_usd", 5),
                        repo_dir, system, confined)
    started = time.time()
    try:
        if confined:
            with confine.Sandbox(work, agent, cargo_home) as box:
                argv, inner = box.wrap(cmd, env)
                exit_code = run_claude(argv, repo_dir, inner, transcript,
                                       timeout=meta.get("timeout_sec", 1200))
        else:
            exit_code = run_claude(cmd, repo_dir, env, transcript,
                                   timeout=meta.get("timeout_sec", 1200))
    finally:
        unlink_auth(work)
        delegated = delegated_threads(work)
        agents = [{"thread": thread if has_threads else "main", "start": started,
                   "end": time.time()}] + delegated
    seconds = agents[0]["end"] - started

    try:
        info = read_transcript(transcript)
        first_thread = {"thread": agents[0]["thread"], "role": role, **first,
                        "seconds": round(seconds), "exit": exit_code,
                        "tokens": info["tokens"], "init": info["init"]}
        started_threads = [thread_facts(t, setup, run_dir, work) for t in delegated]
        if agent == "codex":
            home = work / "home" / ".codex"
            agents += codex_threads(home, run_dir, agents[0]["end"], started_threads)
            if info["init"] is not None:
                record = codex_session(home)
                first_thread["tokens"] = record.pop("tokens") or first_thread["tokens"]
                record.pop("instructions")
                info["init"].update(record)
        calls = atelier_calls(work, first_thread["thread"])
        final = (info["result"] or {}).get("result") or ""
        (run_dir / "final.md").write_text(final)
        graded = grade(sdir, work, transcript)
        every = [first_thread, *started_threads]
        invalid = run_limit(transcript, run_dir) or invalid_reason(arm, exit_code, info, spawn) or next(
            (problem for t in every
             if (problem := thread_problem(arm, t, setup["roles"].get(t["role"]) or first,
                                           spawn))), None) or (
            f"grade.py printed no JSON: {graded['error'][-500:]}" if "error" in graded else None) or (
            replay.leak(sdir, [transcript, *sorted((run_dir / "threads").glob("*.jsonl"))])
            if (sdir / "case.toml").is_file() else None)
        verdict = judged(sdir, setup, graded, work, final, run_dir, work_root, invalid=invalid)
        agents += [{"thread": f"judge-{j['lab']}", "start": j.pop("start"), "end": j.pop("end")}
                   for j in verdict.get("judged") or []]
        if invalid:
            verdict = {**verdict, "pass": None, "invalid": invalid}

        result = {
            **named, "skill_ref": staged.get(agent),
            "skill_dir": skill_dir if arm == "skill" else None,
            "atelier": os.environ.get("EVAL_ATELIER"),
            "roles": setup["roles"], "judges": setup["judges"],
            "exit": exit_code, "seconds": round(seconds), "run_dir": str(run_dir),
            "work_dir": str(work), "init": info["init"],
            "tokens": {k: sum(t["tokens"][k] for t in every) for k in ("in", "cached_in", "out")},
            "threads": [{k: v for k, v in t.items() if k != "init"} for t in every],
            "atelier_calls": len(calls),
            "atelier_failed": [c for c in calls if c["exit"] != 0],
            **({"proxy_refused": confine.refused(work)} if confined else {}),
            "agents": agents, "peak_agents": peak(agents),
            **verdict,
        }
        (run_dir / "result.json").write_text(json.dumps(result, indent=1))
        return result
    finally:
        # Pass, fail or harness error, the run's build output goes once
        # grading is over; the rest of its work directory stays.
        drop_build_output(work)


def build_outputs(work: Path) -> list:
    """Each cargo target directory under a run's work directory: one that
    holds cargo's CACHEDIR.TAG and .rustc_info.json, wherever the agent put
    it (repo/target, or a CARGO_TARGET_DIR of its own under tmp/). Cargo's
    registry cache, tagged too, has no .rustc_info.json. Symlinks are not
    followed, and .git directories are not searched."""
    found = []
    for root, dirs, files in os.walk(work):
        if "CACHEDIR.TAG" in files and ".rustc_info.json" in files:
            found.append(Path(root))
            dirs.clear()
        else:
            dirs[:] = [d for d in dirs if d != ".git"]
    return found


def drop_build_output(work: Path) -> None:
    """Delete a run's build output once it is graded, whether the run passed
    or not, so a matrix of replayed reviews, each building in a target of
    about 2.5G, cannot fill the disk. Grading reads no build output."""
    for target in build_outputs(work):
        shutil.rmtree(target, ignore_errors=True)


def discard_work(result: dict, keep: bool) -> bool:
    """Whether a run's work directory goes: only a pass nobody needs to read
    by hand, unless --keep."""
    return result.get("pass") is True and not result.get("needs_hand_grade") \
        and not keep and bool(result.get("work_dir"))


def kept_runs(results: list, setup: dict, ref, skill_dir: str, atelier) -> dict:
    """The runs a matrix already made into its output that it keeps, by
    (scenario, arm, index): graded, in this setup, its name, roles and
    judges as they are now, on the same binary (where
    both name one), and in the skill arm from the skill at `ref`, a full
    SHA. Running the matrix again makes only the others: the runs missing
    and those graded null, the ones that ended on a provider's limit
    included."""
    kept = {}
    for r in results:
        if r.get("pass") is None or r.get("setup") != setup["name"] \
                or r.get("roles") != setup["roles"] or r.get("judges") != setup["judges"]:
            continue
        if atelier and r.get("atelier") and r["atelier"] != atelier:
            continue
        if r.get("arm") == "skill" and not (
                ref and r.get("skill_ref") and ref.startswith(r["skill_ref"])
                and r.get("skill_dir") == skill_dir):
            continue
        kept.setdefault((r.get("scenario"), r.get("arm"), r.get("index")), r)
    return kept


def builds_rust(scenario: str) -> bool:
    """Whether a scenario's agent builds Rust ("cargo": true, the replayed
    reviews), so its runs count against --cargo-jobs and not --jobs."""
    return bool(json.loads((SCENARIOS / scenario / "scenario.json").read_text()).get("cargo"))


def check_confined(scenarios: list) -> str:
    """Why these scenarios cannot run here, or None: a confined one needs
    a sandbox (confine.available) and no threads."""
    metas = {s: json.loads((SCENARIOS / s / "scenario.json").read_text()) for s in scenarios}
    confined = [s for s, m in metas.items() if m.get("confine")]
    both = [s for s in confined if metas[s].get("threads")]
    if both:
        return f"{', '.join(both)} confine(s) a run and start(s) threads, which it cannot"
    problem = confine.available() if confined else None
    return f"{', '.join(confined)} must run confined, and {problem}" if problem else None


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument("scenarios", nargs="+")
    p.add_argument("--setup", help="a checked-in setup, harness/setups/<name>.json: "
                   + ", ".join(sorted(f.stem for f in SETUPS.glob("*.json"))))
    p.add_argument("--agent", choices=tuple(LABS),
                   help="with --model and --effort, a setup of one model in every "
                   "role (default claude)")
    p.add_argument("--arms", default="none,skill")
    p.add_argument("--ref", default="HEAD", help="git ref of the skill to stage")
    p.add_argument("--skill-dir", default=DEFAULT_SKILL_DIR,
                   help="the skill's directory in this repository, staged as agent-factory")
    p.add_argument("--model", help="the one model's name (default opus)")
    p.add_argument("--effort", help="the one model's effort (default high)")
    p.add_argument("--runs", type=int, default=1)
    p.add_argument("--jobs", type=int, default=JOBS,
                   help=f"the most runs at once that build no Rust (default {JOBS})")
    p.add_argument("--cargo-jobs", type=int, default=CARGO_JOBS,
                   help="the most runs at once that build Rust, the replayed reviews "
                   f"(default {CARGO_JOBS})")
    p.add_argument("--out", type=Path, default=DEFAULT_OUT,
                   help="where transcripts and results go; a matrix run again into "
                   "the same one makes only its runs missing or invalid there")
    p.add_argument("--work-root", type=Path, default=DEFAULT_WORK_ROOT,
                   help="where work directories are made; must be outside $HOME")
    p.add_argument("--keep", action="store_true",
                   help="keep work directories, without their build output "
                   "(default: keep only on failure)")
    a = p.parse_args()
    if inside(a.work_root, Path.home()):
        p.error(f"--work-root {a.work_root} is inside $HOME, where the user's "
                "own CLAUDE.md would load into every run")
    if a.jobs < 1 or a.cargo_jobs < 1:
        p.error("--jobs and --cargo-jobs are each at least 1")
    problem = check_confined(a.scenarios)
    if problem:
        p.error(problem)
    roleless = [s for s in a.scenarios if json.loads(
        (SCENARIOS / s / "scenario.json").read_text()).get("role") not in ROLES]
    if roleless:
        p.error(f"{', '.join(roleless)} name(s) no role ({', '.join(ROLES)}) for the "
                "first agent in scenario.json")
    if a.setup and (a.agent or a.model or a.effort):
        p.error("--setup gives every role its agent, model and effort; leave out "
                "--agent, --model and --effort, or give them without --setup")
    try:
        setup = load_setup(a.setup) if a.setup else one_model_setup(
            a.agent or "claude", a.model or "opus", a.effort or "high")
    except ValueError as exc:
        p.error(str(exc))
    arms = a.arms.split(",")
    ref = None
    if "skill" in arms:
        found = subprocess.run(["git", "-C", str(REPO), "rev-parse", "--verify", "-q",
                                f"{a.ref}^{{commit}}"], capture_output=True, text=True)
        if found.returncode != 0:
            p.error(f"--ref {a.ref} names no commit of this repository")
        ref = found.stdout.strip()

    every = [(s, arm, i) for s in a.scenarios for arm in arms for i in range(a.runs)]
    results_path = a.out / "results.jsonl"
    a.out.mkdir(parents=True, exist_ok=True)
    a.work_root.mkdir(parents=True, exist_ok=True)
    made = kept_runs(table.read([results_path]) if results_path.exists() else [],
                     setup, ref, a.skill_dir, os.environ.get("EVAL_ATELIER"))
    kept = [made[run] for run in every if run in made]
    jobs = [run for run in every if run not in made]
    print(json.dumps({"kept": len(kept), "making": len(jobs), "out": str(a.out)}), flush=True)
    if jobs:
        for agent in sorted({r["agent"] for r in setup["roles"].values()}):
            check_isolation(a.work_root, a.out, agent)
    rust = {s for s in a.scenarios if builds_rust(s)}
    alive, results = [], []
    with concurrent.futures.ThreadPoolExecutor(a.cargo_jobs) as builds, \
            concurrent.futures.ThreadPoolExecutor(a.jobs) as others:
        futures = {(builds if s in rust else others).submit(
            run_one, s, arm, ref or a.ref, setup, i, a.out, a.work_root, a.skill_dir): (s, arm, i)
            for s, arm, i in jobs}
        for fut in concurrent.futures.as_completed(futures):
            s, arm, i = futures[fut]
            try:
                r = fut.result()
            except Exception as exc:  # record it; keep the other runs' results
                r = {"scenario": s, "arm": arm, "setup": setup["name"], "index": i,
                     "pass": None, "invalid": f"harness error: {exc!r}", "run_dir": ""}
            results.append(r)
            alive += r.get("agents") or []
            with results_path.open("a") as f:
                f.write(json.dumps(r) + "\n")
            if discard_work(r, a.keep):
                shutil.rmtree(r["work_dir"], ignore_errors=True)
                (Path(r["run_dir"]) / "work").unlink(missing_ok=True)
            print(json.dumps({**{k: r.get(k) for k in
                                 ("scenario", "arm", "index", "pass", "invalid", "failed",
                                  "exit", "seconds", "peak_agents", "run_dir")},
                              "reported": {a["id"]: a["answer"] for a in r.get("rubric") or []
                                           if a["kind"] == "reported"}}), flush=True)
    print(json.dumps({"matrix_peak_agents": peak(alive), "runs": len(jobs),
                      "kept": len(kept)}), flush=True)
    print(table.table(kept + results))


if __name__ == "__main__":
    main()
