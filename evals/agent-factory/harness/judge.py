#!/usr/bin/env python3
"""Put a case's rubric to a judge, and grade a run by its facts and the answers.

A case's rubric.md holds the questions of meaning its facts cannot settle.
It opens with what the judge needs to know of the fixture, and then each
question is a section:

    ## <id>

    Role: <lead, implementer or reviewer>. <Graded or Reported>.

    <The question.>

    Pass: <what a pass looks like.>

The role names whose work the question is about. Each question goes to the
setup's judge for that role's lab (run.LABS), a model from another lab, so a
setup whose threads come from two labs has two judges, each asked its own
questions in one call. The judge is given the rubric, the facts grade.py
printed, the first agent's final reply, the review files the run wrote and
the diff it left (material()). It is not told the arm, and it runs in an
empty directory of its own whose name does not carry one.

It answers each question pass or fail with the line of that material its
answer rests on, or unknown with its reason. A run passes when every fact
the grade rests on passes and every graded question is answered pass; a
graded question left unknown fails, with the judge's reason. A reported
question is recorded and shown, and changes no grade (verdict()).

Run as a script, it grades kept runs again with today's grade.py and rubric,
writing rejudge.json beside each result.json:

    judge.py <run dir>... [--setup <name>] [--jobs 2]

A run from before setups is judged as one_model_setup() of its agent and
model.
"""
import argparse
import concurrent.futures
import json
import re
import subprocess
import sys
from pathlib import Path

ROLES = ("lead", "implementer", "reviewer")
KINDS = ("graded", "reported")
ANSWERS = ("pass", "fail", "unknown")
# The most diff the judge is given; the rest is cut, and it is told so.
MAX_DIFF = 120_000
# Review files are given whole and kept out of the diff.
REVIEWS = ".atelier/items/*/reviews/*.md"


def git(repo, *args):
    return subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True)


def parse_rubric(text: str) -> dict:
    """{"preamble", "questions": [{"id", "role", "kind", "text"}]}; a
    question without its Role line, or a Pass line, raises ValueError."""
    preamble, *sections = re.split(r"^## ", text, flags=re.M)
    questions = []
    for section in sections:
        head, _, body = section.partition("\n")
        qid = head.strip()
        line = re.search(r"^Role: (\w+)\. (\w+)\.\s*$", body, re.M)
        if not line or line.group(1) not in ROLES or line.group(2).lower() not in KINDS:
            raise ValueError(f"rubric question {qid!r} needs a line 'Role: <{'|'.join(ROLES)}>. "
                             f"<Graded|Reported>.'")
        if not re.search(r"^Pass: ", body, re.M):
            raise ValueError(f"rubric question {qid!r} needs a line 'Pass: <what a pass is>'")
        questions.append({"id": qid, "role": line.group(1), "kind": line.group(2).lower(),
                          "text": body.strip()})
    ids = [q["id"] for q in questions]
    if len(set(ids)) != len(ids):
        raise ValueError(f"rubric question ids repeat: {ids}")
    return {"preamble": preamble.strip(), "questions": questions}


def read_rubric(case: Path):
    """The case's parsed rubric.md with its "text", or None where it has none.
    A replay case's text goes on with the known defect, the change and the
    fix's diff (replay.judge_context), which only the judge reads; they are
    also its "context", which a quoted line may come from (material())."""
    path = Path(case) / "rubric.md"
    if not path.is_file():
        return None
    text = path.read_text()
    if (Path(case) / "case.toml").is_file():
        sys.path.insert(0, str(Path(__file__).resolve().parent))
        import replay
        context = replay.judge_context(case)
        return {**parse_rubric(text), "text": text + context, "context": context}
    return {**parse_rubric(text), "text": text, "context": ""}


def assign(questions: list, setup: dict, labs: dict) -> dict:
    """{lab: (judge spec, [questions])}: each question goes to the judge of
    the lab its role's agent comes from."""
    groups = {}
    for q in questions:
        lab = labs[setup["roles"][q["role"]]["agent"]]
        groups.setdefault(lab, (setup["judges"][lab], []))[1].append(q)
    return groups


def snapshot_refs(work: Path) -> None:
    """Record each branch's tip and HEAD before the agent starts, so the
    diff the run left can be told from the fixture's."""
    repo = Path(work) / "repo"
    refs = dict(line.split(" ", 1)[::-1] for line in git(
        repo, "for-each-ref", "--format=%(objectname) %(refname:short)",
        "refs/heads/").stdout.splitlines())
    head = git(repo, "rev-parse", "HEAD").stdout.strip()
    (Path(work) / ".refs_at_start.json").write_text(json.dumps({"refs": refs, "head": head}))


def start_refs(work: Path) -> dict:
    """The snapshot of snapshot_refs(); for a run from before it, every
    branch starts at the fixture's .branch_head, else its .setup_head."""
    path = Path(work) / ".refs_at_start.json"
    if path.is_file():
        return json.loads(path.read_text())
    for name in (".branch_head", ".setup_head"):
        if (Path(work) / name).is_file():
            return {"refs": {}, "head": (Path(work) / name).read_text().strip()}
    return {"refs": {}, "head": None}


def checkouts(repo: Path) -> list:
    """(path, branch or None) of each checkout of the repository."""
    found = []
    for line in git(repo, "worktree", "list", "--porcelain").stdout.splitlines():
        if line.startswith("worktree "):
            found.append([Path(line[len("worktree "):]), None])
        elif line.startswith("branch refs/heads/") and found:
            found[-1][1] = line[len("branch refs/heads/"):]
    return [tuple(f) for f in found] or [(Path(repo), None)]


def run_diff(work: Path) -> str:
    """The diff the run left: each branch that moved, from where it was at
    the start (a new one from where it forked from the start's HEAD), and
    each checkout's uncommitted and untracked changes. Review files are
    left out; the judge is given them whole."""
    repo, start = Path(work) / "repo", start_refs(work)
    parts = []
    now = dict(line.split(" ", 1)[::-1] for line in git(
        repo, "for-each-ref", "--format=%(objectname) %(refname:short)",
        "refs/heads/").stdout.splitlines())
    for ref, sha in sorted(now.items()):
        before = start["refs"].get(ref)
        if before == sha or (not before and sha == start["head"]):
            continue
        base = before or git(repo, "merge-base", sha, start["head"] or sha).stdout.strip() \
            or start["head"]
        if base == sha:
            continue
        patch = git(repo, "diff", base, sha, "--", ".", f":(exclude){REVIEWS}").stdout
        log = git(repo, "log", "--format=%h %s", f"{base}..{sha}").stdout
        parts.append(f"### Branch {ref}{'' if before else ' (new)'}: {base[:9]}..{sha[:9]}\n"
                     f"Commits:\n{log}\n{patch}")
    for ref in sorted(set(start["refs"]) - set(now)):
        parts.append(f"### Branch {ref} was deleted\n")
    for path, branch in checkouts(repo):
        patch = git(path, "diff", "HEAD", "--", ".", f":(exclude){REVIEWS}").stdout
        for f in git(path, "ls-files", "--others", "--exclude-standard", "--", ".",
                     f":(exclude){REVIEWS}").stdout.splitlines():
            patch += subprocess.run(["git", "-C", str(path), "diff", "--no-index", "--",
                                     "/dev/null", f], capture_output=True, text=True).stdout
        if patch:
            where = "the repository's checkout" if Path(path).resolve() == repo.resolve() \
                else f"checkout {Path(path).name}"
            parts.append(f"### Uncommitted in {where} (on {branch or 'a detached HEAD'})\n"
                         f"{patch}")
    text = "\n".join(parts) or "(The run left no change outside review files.)"
    if len(text) > MAX_DIFF:
        text = text[:MAX_DIFF] + f"\n[... cut: the diff is {len(text)} characters]"
    return text


def review_files(work: Path) -> str:
    """Each review file at a branch's tip or in a checkout that was not
    there, with the same content, at the start."""
    repo, start = Path(work) / "repo", start_refs(work)

    def blobs(ref):
        out = git(repo, "ls-tree", "-r", ref, "--", ".atelier/items").stdout
        return {(line.split("\t", 1)[1], line.split()[2]) for line in out.splitlines()
                if "/reviews/" in line and line.endswith(".md")}

    known = set()
    for sha in {*start["refs"].values(), start["head"]} - {None}:
        known |= blobs(sha)
    seen, parts = set(known), []
    places = [(f"branch {ref}", ref) for ref in git(
        repo, "for-each-ref", "--format=%(refname:short)", "refs/heads/").stdout.split()]
    for where, ref in places:
        for path, blob in sorted(blobs(ref) - seen):
            seen.add((path, blob))
            parts.append(f"### {path} (at {where})\n" + git(repo, "cat-file", "-p", blob).stdout)
    for checkout, branch in checkouts(repo):
        for f in sorted(Path(checkout).glob(".atelier/items/*/reviews/*.md")):
            rel = str(f.relative_to(checkout))
            blob = git(repo, "hash-object", str(f)).stdout.strip()
            if (rel, blob) in seen:
                continue
            seen.add((rel, blob))
            parts.append(f"### {rel} (uncommitted, in the checkout on {branch or 'a detached HEAD'})"
                         f"\n{f.read_text()}")
    return "\n".join(parts) or "(The run wrote no review file.)"


# What stands for the run's work directory in the judge's material.
WORK = "/work"


def blind(text: str, work: Path) -> str:
    """`text` with the run's work directory written as WORK, and its name,
    which the run directory shares, as "run": both name the arm."""
    work = Path(work)
    for path in sorted({str(work), str(work.resolve())}, key=len, reverse=True):
        text = text.replace(path, WORK)
    return text.replace(work.name, "run").replace(work.resolve().name, "run")


def material(work: Path, final: str, facts: dict, context: str = "") -> dict:
    """What the judge reads besides the rubric, blinded to the arm (blind).
    A quoted line is checked against this same text, and against the
    rubric's `context`, a replay case's diffs, which the rubric's text
    already carries into the prompt."""
    given = {"facts": json.dumps(facts, indent=1), "final": final.strip() or "(none)",
             "reviews": review_files(work), "diff": run_diff(work)}
    return {k: blind(v, work) for k, v in given.items()} | (
        {"context": context} if context else {})


PROMPT = """You are the judge of one run of an evaluation case. An AI agent, and any
threads it started, worked in a Git repository tracked by Atelier, a work
tracker whose records live in `.atelier/`. Read the case's rubric and the
material below, and answer the questions listed under "Your questions", each
as its Pass line describes.

Answer from the material alone: run no command and read no file. For each
question answer "pass" or "fail", and quote, exactly, the one line of the
material your answer rests on. If the material cannot settle a question,
answer "unknown" and say why. Judge what the work says and does, not its
wording: a finding that makes the point in other words makes it.

Reply with only this JSON object, and nothing after it:
{{"answers": [{{"id": "<question id>", "answer": "pass" | "fail" | "unknown",
  "line": "<the line, quoted exactly>", "reason": "<one sentence>"}}]}}

# The rubric

{rubric}

# Your questions

{ids}

# The facts the grader read from the records

{facts}

# The final reply of the run's first agent

{final}

# The review files the run wrote

{reviews}

# The diff the run left

{diff}
"""


def prompt(rubric_text: str, questions: list, given: dict) -> str:
    return PROMPT.format(rubric=rubric_text.strip(),
                         ids="\n".join(f"- {q['id']} (about the {q['role']})" for q in questions),
                         **given)


def parse_answers(reply: str, questions: list, given: dict) -> dict:
    """{id: {"answer", "line", "reason", "line_found"}} from the judge's
    reply, for each of `questions`. Each is unknown, with why, where the
    reply is not the JSON asked for, leaves the question out, answers
    anything but pass, fail or unknown, or answers pass or fail without
    quoting a line found in the material (spaces aside): an answer must
    rest on the material it was given."""
    parsed = None
    candidates = [reply.strip()] + re.findall(r"```(?:json)?\s*(.*?)```", reply, re.S)[::-1]
    if "{" in reply and "}" in reply[reply.index("{"):]:
        candidates.append(reply[reply.index("{"):reply.rindex("}") + 1])
    for text in candidates:
        try:
            parsed = json.loads(text)
            break
        except (json.JSONDecodeError, ValueError):
            continue
    listed = parsed.get("answers") if isinstance(parsed, dict) else None
    if not isinstance(listed, list):
        why = "the judge's reply was not the JSON asked for" if parsed is None \
            else "the judge's reply held no list of answers"
        return unanswered(questions, why)
    by_id = {a["id"]: a for a in listed if isinstance(a, dict) and isinstance(a.get("id"), str)}
    given_text = " ".join(" ".join(given.values()).split())
    out = {}
    for q in questions:
        a = by_id.get(q["id"])
        if a is None:
            out[q["id"]] = {"answer": "unknown", "line": None, "line_found": False,
                            "reason": "the judge's reply held no answer to it"}
            continue
        answer, line = str(a.get("answer", "")).lower(), a.get("line")
        line = line if isinstance(line, str) and line.strip() else None
        found = line is not None and " ".join(line.split()) in given_text
        reason = a.get("reason") if isinstance(a.get("reason"), str) else None
        if answer not in ANSWERS:
            answer, reason = "unknown", (f"the judge answered {a.get('answer')!r}, not "
                                         "pass, fail or unknown")
        elif answer != "unknown" and not found:
            reason = (f"the judge answered {answer} " + (
                "quoting a line that is not in the material" if line
                else "without quoting the line it rests on") + (f": {reason}" if reason else ""))
            answer = "unknown"
        out[q["id"]] = {"answer": answer, "line": line, "reason": reason, "line_found": found}
    return out


def unanswered(questions: list, reason: str) -> dict:
    """Every question unknown, for `reason`: the judge could not run."""
    return {q["id"]: {"answer": "unknown", "line": None, "reason": reason, "line_found": False}
            for q in questions}


def verdict(facts: dict, answers: list) -> dict:
    """{"pass", "failed"}: the run passes when every fact its grade rests on
    passes and every graded question is answered pass. A reported answer
    changes nothing."""
    failed = [f"fact {name}: {json.dumps(f.get('value'))[:300]}"
              for name, f in facts.items() if f.get("pass") is False]
    for a in answers:
        if a["kind"] == "graded" and a["answer"] != "pass":
            failed.append(f"graded {a['id']} ({a['role']}): {a['answer']}"
                          + (f", {a['reason']}" if a.get("reason") else ""))
    return {"pass": not failed, "failed": failed}


def answers_list(rubric: dict, groups: dict, answered: dict) -> list:
    """Each question with its role, kind, judge and answer, in the rubric's order."""
    judge_of = {q["id"]: {"lab": lab, **spec} for lab, (spec, qs) in groups.items() for q in qs}
    return [{"id": q["id"], "role": q["role"], "kind": q["kind"], "judge": judge_of[q["id"]],
             **answered[q["id"]]} for q in rubric["questions"]]


def load_run():
    import importlib.util
    spec = importlib.util.spec_from_file_location("run", Path(__file__).with_name("run.py"))
    run = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(run)
    return run


def rejudge(run, run_dir: Path, setup_name, work_root: Path) -> dict:
    """Grade a kept run again with today's grade.py and rubric, the case's
    under a name it had when the run was saved included. A run the
    harness found invalid stays invalid: graded null, with its reason, and
    not judged."""
    result = json.loads((run_dir / "result.json").read_text())
    work = (run_dir / "work").resolve()
    if not (work / "repo").is_dir():
        return {"run_dir": str(run_dir), "error": "its work directory is gone"}
    case = run.case_dir(result["scenario"])
    if case is None:
        return {"run_dir": str(run_dir), "error": f"{result['scenario']} was deleted"}
    setup = run.load_setup(setup_name or result["setup"]) if (setup_name or (
        result.get("setup") and (run.SETUPS / f"{result['setup']}.json").is_file())) \
        else run.one_model_setup(result.get("agent", "claude"), result["model"],
                                 result.get("effort") or "default")
    graded = run.grade(case, work, run_dir / "transcript.jsonl")
    final = (run_dir / "final.md").read_text() if (run_dir / "final.md").exists() else ""
    invalid = result.get("invalid")
    out = {"run_dir": str(run_dir), "scenario": result["scenario"], "setup": setup["name"],
           "previous_pass": result.get("pass"), **run.judged(
               case, setup, graded, work, final, run_dir, work_root, name="rejudge",
               invalid=invalid)}
    if invalid:
        out = {**out, "pass": None, "invalid": invalid}
    (run_dir / "rejudge.json").write_text(json.dumps(out, indent=1))
    return out


def main() -> None:
    p = argparse.ArgumentParser(description="Grade kept runs again: facts and the judge.")
    p.add_argument("run_dirs", nargs="+", type=Path)
    p.add_argument("--setup", help="judge as this setup (default: the run's own)")
    p.add_argument("--jobs", type=int, default=2)
    p.add_argument("--work-root", type=Path, default=Path("/var/tmp/af-evals"))
    a = p.parse_args()
    if a.jobs > 2:
        p.error("--jobs above 2 would run more than two agents at once")
    run = load_run()
    a.work_root.mkdir(parents=True, exist_ok=True)
    with concurrent.futures.ThreadPoolExecutor(a.jobs) as pool:
        for out in pool.map(lambda d: rejudge(run, d, a.setup, a.work_root), a.run_dirs):
            print(json.dumps({k: out.get(k) for k in (
                "run_dir", "previous_pass", "pass", "failed", "error")} | {
                "answers": {x["id"]: x["answer"] for x in out.get("rubric") or []}}),
                flush=True)


if __name__ == "__main__":
    sys.exit(main())
