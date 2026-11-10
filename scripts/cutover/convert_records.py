#!/usr/bin/env python3
"""Convert a repository's Atelier records from the old layout to the new one.

Run at the cutover, once per repository, and deleted after it
(docs/strategies/redesign-architecture.md, Transition).

    convert_records.py                     replace .atelier/ with the new layout
    convert_records.py --verify <commit>   compare this tree with a fresh conversion of <commit>

The first form reads the working tree's old `.atelier/` and the strategy files
its open missions link, and writes `config.toml`, `missions/` and `items/`.
It makes no commit. It writes nothing when an old record does not read.
"""
import json
import os
import posixpath
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib

OPEN = ("draft", "backlog", "todo", "in_progress")
CLOSED = ("done", "declined", "superseded")
TYPES = ("bug", "decision", "feature", "loose_end", "mission", "spike", "task", "validation")
NEW_STATUS = {"draft": "draft", "backlog": "draft", "todo": "ready", "in_progress": "ready"}
PRIORITY = {"P0": "critical", "P1": "high", "P2": "medium", "P3": "low"}
MISSION_SECTIONS = ("Why", "Outcome", "Success criteria", "Scope", "Authority")
UNSPECIFIED = "Not specified in the old record; written at the conversion to the new layout."
DECISION_DONE_WHEN = "- [ ] The question is answered and the answer is written in this item's Report."
# A link's destination, inline or in a reference definition, bare or in <...>; each
# alternative is (what comes before it, the destination).
LINK = re.compile(r"(\]\(\s*<)([^>\n]+)(?=>)|(\]\(\s*)([^)\s<]+)(?=[\s)])"
                  r"|^( {0,3}\[[^\]]+\]:[ \t]*<)([^>\n]+)(?=>)|^( {0,3}\[[^\]]+\]:[ \t]*)([^\s<]+)", re.M)
REFERENCE = re.compile(r"^ {0,3}\[([^\]]+)\]:.*$", re.M)


def destination(m):
    """(what comes before a LINK match's destination, the destination)."""
    return next((m.group(i), m.group(i + 1)) for i in (1, 3, 5, 7) if m.group(i) is not None)
TIME = re.compile(r"\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(\.\d+)?(Z|[+-]\d\d:\d\d)")


class Unreadable(Exception):
    pass


# ---------------------------------------------------------------- old YAML


def parse_yaml(text):
    """The subset of YAML the old tool wrote: block maps and lists, flow lists
    of scalars, plain, single- and double-quoted scalars, full-line comments.
    Anything else is refused."""
    lines = []
    for raw in text.split("\n"):
        stripped = raw.lstrip(" ")
        if stripped and not stripped.startswith("#"):
            lines.append([len(raw) - len(stripped), stripped.rstrip()])
    if not lines:
        return {}
    value, i = _block(lines, 0, lines[0][0])
    if i != len(lines):
        raise Unreadable(f"unexpected indentation at `{lines[i][1]}`")
    return value


def _block(lines, i, indent):
    if lines[i][1] == "-" or lines[i][1].startswith("- "):
        items = []
        while i < len(lines) and lines[i][0] == indent and (lines[i][1] == "-" or lines[i][1].startswith("- ")):
            rest = lines[i][1][2:]
            if rest:
                lines[i] = [indent + 2, rest]
                value, i = _block(lines, i, indent + 2)
            elif i + 1 < len(lines) and lines[i + 1][0] > indent:
                value, i = _block(lines, i + 1, lines[i + 1][0])
            else:
                value, i = None, i + 1
            items.append(value)
        return items, i
    key, sep, rest = lines[i][1].partition(":")
    if not sep or rest[:1] not in ("", " ") or key.startswith(("'", '"', "[", "{")):
        return _scalar(lines[i][1]), i + 1
    mapping = {}
    while i < len(lines) and lines[i][0] == indent:
        key, sep, rest = lines[i][1].partition(":")
        if not sep or rest[:1] not in ("", " "):
            raise Unreadable(f"`{lines[i][1]}` is not `key: value`")
        if key in mapping:
            raise Unreadable(f"`{key}` appears twice")
        i += 1
        if rest.strip():
            mapping[key] = _scalar(rest.strip())
        elif i < len(lines) and (
            lines[i][0] > indent or (lines[i][0] == indent and lines[i][1].startswith("- "))
        ):
            mapping[key], i = _block(lines, i, lines[i][0])
        else:
            mapping[key] = None
    return mapping, i


def _scalar(text):
    if text.startswith('"'):
        try:
            return json.loads(text)
        except json.JSONDecodeError:
            raise Unreadable(f"cannot read the quoted string {text}") from None
    if text.startswith("'"):
        if len(text) < 2 or not text.endswith("'"):
            raise Unreadable(f"cannot read the quoted string {text}")
        return text[1:-1].replace("''", "'")
    if text.startswith("[") and text.endswith("]"):
        inner = text[1:-1].strip()
        return [_scalar(part.strip()) for part in inner.split(",")] if inner else []
    if text == "{}":
        return {}
    if text.startswith(("{", "[", "&", "*", "!", "|", ">")):
        raise Unreadable(f"cannot read `{text}`")
    return None if text in ("null", "~") else text


# ---------------------------------------------------------------- Markdown


def unfenced(text):
    """Each line of `text` with its offset and whether it is outside a fenced
    code block (fence lines count as inside), as in CommonMark."""
    out, fence, offset = [], None, 0
    for line in text.splitlines(keepends=True):
        bare = line.rstrip("\n")
        marks = bare.lstrip(" ")
        char = marks[:1]
        run = len(marks) - len(marks.lstrip(char)) if char in ("`", "~") else 0
        is_fence = run >= 3 and len(bare) - len(marks) <= 3 and not (char == "`" and "`" in marks[run:])
        if fence:
            if is_fence and char == fence[0] and run >= fence[1] and not marks[run:].strip():
                fence = None
            out.append((offset, bare, False))
        elif is_fence:
            fence = (char, run)
            out.append((offset, bare, False))
        else:
            out.append((offset, bare, True))
        offset += len(line)
    return out


def heading(line):
    hashes = len(line) - len(line.lstrip("#"))
    if 1 <= hashes <= 6 and (len(line) == hashes or line[hashes] == " "):
        return hashes, line[hashes:].strip()
    return None


def split_sections(text):
    """(preamble, [(title, heading line, content)]) at the shallowest heading
    level of `text`, ignoring headings in fenced code blocks."""
    heads = [(off, line, heading(line)) for off, line, outside in unfenced(text) if outside and heading(line)]
    if not heads:
        return text, []
    level = min(h[0] for _, _, h in heads)
    starts = [(off, line, h[1]) for off, line, h in heads if h[0] == level]
    sections = []
    for n, (off, line, title) in enumerate(starts):
        end = starts[n + 1][0] if n + 1 < len(starts) else len(text)
        body_start = min(off + len(line) + 1, end)
        sections.append((title, text[off:body_start], text[body_start:end]))
    return text[: starts[0][0]], sections


def shift_headings(text, by):
    if by == 0:
        return text
    lines = text.splitlines(keepends=True)
    for n, (_, line, outside) in enumerate(unfenced(text)):
        if outside and heading(line):
            lines[n] = "#" * by + lines[n]
    return "".join(lines)


def slug(title):
    return re.sub(r"[^\w\- ]", "", title.strip().lower()).replace(" ", "-")


def anchors(text):
    return {slug(line[h[0]:]) for _, line, outside in unfenced(text) if outside and (h := heading(line))}


def render_body(preamble, sections):
    """A body of `## <title>` sections, each separated by one blank line."""
    parts = [preamble.strip("\n") + "\n"] if preamble.strip() else []
    for title, content in sections:
        content = content.strip("\n")
        parts.append(f"## {title}\n\n{content}\n" if content else f"## {title}\n")
    return "\n" + "\n".join(parts)


# ---------------------------------------------------------------- new YAML and TOML

YAML_PLAIN_UNSAFE = re.compile(
    r"""^$|^[\-?:,\[\]{}#&*!|>'"%@`]|: |:$| #|^\s|\s$|^(~|null|Null|NULL|true|True|TRUE|false|False|FALSE)$"""
    r"""|^[-+]?(\d[\d_]*|0x[0-9a-fA-F]+|0o[0-7]+|0b[01]+|\d*\.\d+([eE][-+]?\d+)?|\d+[eE][-+]?\d+|\.(inf|Inf|INF|nan|NaN|NAN))$"""
)


def yaml_string(value):
    """As serde_yaml writes it: plain where that reads back as the same
    string, otherwise single-quoted."""
    if any(ch < " " for ch in value):
        return json.dumps(value, ensure_ascii=False)
    if value.startswith(("- ", "? ")) or value in ("-", "?", "---", "...") or YAML_PLAIN_UNSAFE.search(value):
        return "'" + value.replace("'", "''") + "'"
    return value


def yaml_list(key, values):
    return f"{key}: []\n" if not values else f"{key}:\n" + "".join(f"- {yaml_string(v)}\n" for v in values)


def new_time(value, path):
    if not isinstance(value, str) or not TIME.fullmatch(value):
        raise Unreadable(f"{path}: `{value}` is not an RFC 3339 time")
    # As chrono writes it: no fraction, or 3, 6 or 9 digits; UTC as Z.
    m = TIME.fullmatch(value)
    nanos = (m.group(1) or ".")[1:].ljust(9, "0")[:9]
    digits = 0 if nanos == "0" * 9 else 3 if nanos.endswith("0" * 6) else 6 if nanos.endswith("000") else 9
    fraction = "." + nanos[:digits] if digits else ""
    offset = "Z" if m.group(2) in ("Z", "+00:00") else m.group(2)
    return value[:19] + fraction + offset


def front_matter(fields):
    out = ["---\n"]
    for key, value in fields:
        if isinstance(value, list):
            out.append(yaml_list(key, value))
        elif isinstance(value, dict):
            out.append(f"{key}:\n" + "".join(f"  {k}: {v}\n" for k, v in value.items()))
        elif value is None:
            out.append(f"{key}: null\n")
        else:
            out.append(f"{key}: {value}\n")
    out.append("---\n")
    return "".join(out)


def toml_string(value):
    return json.dumps(value, ensure_ascii=False)


# ---------------------------------------------------------------- reading the old records


def read(root, rel):
    try:
        with open(os.path.join(root, rel), encoding="utf-8") as f:
            return f.read()
    except (OSError, UnicodeDecodeError) as e:
        raise Unreadable(f"{rel}: {e}") from None


def read_record(root, rel):
    text = read(root, rel)
    if not text.startswith("---\n") or "\n---\n" not in text[3:]:
        raise Unreadable(f"{rel}: no front matter between `---` lines")
    head, body = text[4:].split("\n---\n", 1)
    try:
        fm = parse_yaml(head)
    except Unreadable as e:
        raise Unreadable(f"{rel}: {e}") from None
    expected_id = posixpath.basename(rel)[:-3]
    if not isinstance(fm, dict) or fm.get("id") != expected_id:
        raise Unreadable(f"{rel}: its id is not {expected_id}")
    rel_block = fm.get("relationships") or {}
    problems = [
        fm.get("schema") != "atelier.issue" and "schema",
        fm.get("status") not in OPEN + CLOSED and "status",
        fm.get("issue_type") not in TYPES and "issue_type",
        fm.get("priority") not in PRIORITY and "priority",
        not isinstance(fm.get("title"), str) and "title",
        not (isinstance(fm.get("labels") or [], list) and all(isinstance(x, str) for x in fm.get("labels") or []))
        and "labels",
        not isinstance(rel_block, dict) and "relationships",
    ]
    if any(problems):
        raise Unreadable(f"{rel}: cannot read {', '.join(p for p in problems if p)}")
    for key in ("blocks", "children", "attachments", "relates"):
        entries = rel_block.get(key) or []
        if not isinstance(entries, list) or not all(isinstance(e, dict) and e.get("id") for e in entries):
            raise Unreadable(f"{rel}: cannot read relationships.{key}")
    strategy = fm.get("strategy")
    if strategy is not None and not (isinstance(strategy, dict) and isinstance(strategy.get("path"), str)):
        raise Unreadable(f"{rel}: cannot read strategy")
    fm["created_at"] = new_time(fm.get("created_at"), rel)
    fm["updated_at"] = new_time(fm.get("updated_at"), rel)
    fm["body"] = body
    return fm


def read_config(root):
    try:
        old = tomllib.loads(read(root, ".atelier/config.toml"))
        workflow = parse_yaml(read(root, ".atelier/workflow.yaml"))
    except (tomllib.TOMLDecodeError, Unreadable) as e:
        raise Unreadable(f".atelier/config.toml or workflow.yaml: {e}") from None
    slug_ = old.get("project_slug")
    base = (workflow.get("branch_policy") or {}).get("base_branch")
    if not slug_ or not base:
        raise Unreadable(".atelier/config.toml or workflow.yaml: no project_slug or base_branch")
    lines = [f"schema_version = 3\nslug = {toml_string(slug_)}\nbase_branch = {toml_string(base)}\n"]
    for check in workflow.get("merge_checks") or []:
        if check == "ci.passed":
            continue  # the forge's ci_workflow, below
        if isinstance(check, dict) and isinstance((check.get("shell.check") or {}).get("command"), str):
            lines.append(f"[[merge_check]]\nrun = {toml_string(check['shell.check']['command'])}\n")
        else:
            raise Unreadable(f".atelier/workflow.yaml: cannot read the merge check {check!r}")
    forge = workflow.get("forge")
    if forge:
        if forge.get("provider") != "github" or not forge.get("remote"):
            raise Unreadable(".atelier/workflow.yaml: cannot read forge")
        text = f'[forge]\nprovider = "github"\nremote = {toml_string(forge["remote"])}\n'
        name = forge.get("ci_workflow")
        if name and os.path.isfile(os.path.join(root, ".github/workflows", name)):
            text += f"ci_workflow = {toml_string(name)}\n"
        lines.append(text)
    return "\n".join(lines)


# ---------------------------------------------------------------- links


class Links:
    """Moves a text from one repository path to another without breaking a
    relative link that resolved, and replaces each mention of a moved
    strategy file's old path with its mission's directory."""

    def __init__(self, tracked, moved, renamed):
        self.tracked = set(tracked)
        self.dirs = {posixpath.dirname(p) for p in tracked}
        for d in list(self.dirs):
            while d:
                d = posixpath.dirname(d)
                self.dirs.add(d)
        self.moved = moved  # old path -> (mission dir, {slug: file name})
        self.renamed = renamed  # an open record's old file -> its new one

    def target(self, old_path, fragment):
        new_dir, where = self.moved[old_path]
        if fragment and fragment in where:
            return f"{new_dir}/{where[fragment]}"
        return new_dir

    def rewrite(self, text, old_file, new_file, own=None):
        def link(m):
            prefix, target = destination(m)
            path, hash_, fragment = target.partition("#")
            if not path:
                if own and fragment in own[1] and own[1][fragment] != own[0]:
                    return f"{prefix}{own[1][fragment]}#{fragment}"
                return m.group(0)
            if re.match(r"[A-Za-z][\w+.-]*:", path) or path.startswith("/"):
                return m.group(0)
            resolved = posixpath.normpath(posixpath.join(posixpath.dirname(old_file), path))
            if resolved in self.renamed:
                resolved = self.renamed[resolved]
            elif resolved in self.moved:
                resolved = self.target(resolved, fragment)
            elif resolved not in self.tracked and resolved not in self.dirs:
                return m.group(0)
            new = posixpath.relpath(resolved, posixpath.dirname(new_file))
            return f"{prefix}{new}{hash_}{fragment}"

        text = LINK.sub(link, text)
        for old in sorted(self.moved, key=len, reverse=True):
            text = text.replace(old, self.moved[old][0] + "/")
        return text


def files_naming(root, moved):
    """Tracked files outside .atelier/ that name a moved file's old path, or
    link to it relatively: {file: [old paths]}."""
    def grep(*args):
        out = subprocess.run(["git", "-C", root, "grep", "-l", "-F", *args, ":!.atelier"],
                             capture_output=True, text=True)
        if out.returncode not in (0, 1):
            raise Unreadable(f"git grep failed: {out.stderr.strip()}")
        return [f for f in out.stdout.split("\n") if f and f not in moved]

    found = {}
    for old in sorted(moved):
        names = set(grep("-e", old, "--", "."))
        for f in grep("-e", posixpath.basename(old), "--", "*.md"):
            targets = [destination(m)[1].partition("#")[0] for m in LINK.finditer(read(root, f))]
            if any(posixpath.normpath(posixpath.join(posixpath.dirname(f), t)) == old for t in targets):
                names.add(f)
        for f in names:
            found.setdefault(f, []).append(old)
    return found


# ---------------------------------------------------------------- conversion


def read_records(root):
    issue_dir = os.path.join(root, ".atelier/issues")
    if not os.path.isdir(issue_dir):
        raise Unreadable(".atelier/issues/ is missing")
    names = sorted(os.listdir(issue_dir))
    for name in names:
        if not name.endswith(".md"):
            raise Unreadable(f".atelier/issues/{name}: not a record")
    return {n[:-3]: read_record(root, f".atelier/issues/{n}") for n in names}


class Graph:
    """The old relationships. Either end of an advances edge, and either end
    of a blocks/blocked_by edge, may hold it."""

    def __init__(self, records):
        self.records = records
        self.parents, self.blocked_by, self.advances = {}, {}, {}
        for rid, r in records.items():
            for child in r["relationships"].get("children") or []:
                self.parents.setdefault(child["id"], []).append(rid)
            for target in r["relationships"].get("blocks") or []:
                self.blocked_by.setdefault(target["id"], set()).add(rid)
            for rel in r["relationships"].get("relates") or []:
                if rel.get("type") == "blocked_by":
                    self.blocked_by.setdefault(rid, set()).add(rel["id"])
                if rel.get("type") == "advances":
                    self.advances.setdefault(rid, set()).add(rel["id"])
                    self.advances.setdefault(rel["id"], set()).add(rid)
        # An item that advances no mission takes its nearest ancestor's.
        self.mission_of, self.through = {}, {}
        for rid in records:
            if self.is_open(rid, mission=False):
                found = None if self.advanced(rid) else self.mission_through_parent(rid)
                self.mission_of[rid] = found[0] if found else self.advanced(rid)
                if found:
                    self.through[rid] = found[1]

    def is_open(self, rid, mission=None):
        r = self.records.get(rid)
        return bool(r) and r["status"] in OPEN and mission in (None, r["issue_type"] == "mission")

    def advanced(self, rid):
        """The first open mission by ID that `rid` advances."""
        return min((m for m in self.advances.get(rid, ()) if self.is_open(m, mission=True)), default=None)

    def blockers(self, rid):
        """The open items blocking `rid`, which become its blocked_by."""
        return sorted(b for b in self.blocked_by.get(rid, ()) if self.is_open(b, mission=False))

    def as_text(self, rid):
        """(relation, ID) for each of `rid`'s relationships that no field of
        its new record holds."""
        mission, blockers = self.mission_of.get(rid), self.blockers(rid)
        found = [("parent", p) for p in sorted(self.parents.get(rid, []))]
        for t in self.records[rid]["relationships"].get("relates") or []:
            kind, other = t.get("type"), t["id"]
            if kind == "advances" and (other == mission or self.mission_of.get(other) == rid):
                continue
            if kind == "blocked_by" and (other in blockers or not self.is_open(other)):
                continue
            found.append((kind if t.get("kind", "issue") == "issue" else f"{kind} {t.get('kind')}", other))
        for b in sorted(self.blocked_by.get(rid, ())):
            if self.is_open(b) and b not in blockers and ("blocked_by", b) not in found:
                found.append(("blocked_by", b))
        return found

    def mission_through_parent(self, rid):
        """(mission, ancestor) for the nearest ancestor of `rid` that advances
        an open mission; the first parent by ID at each step."""
        ancestor, seen = min(self.parents.get(rid, []), default=None), set()
        while ancestor and ancestor not in seen:
            seen.add(ancestor)
            if self.advanced(ancestor):
                return self.advanced(ancestor), ancestor
            ancestor = min(self.parents.get(ancestor, []), default=None)
        return None


def mission_section(title):
    return next((s for s in MISSION_SECTIONS if s.lower() == title.lower()), None)


def split_strategy(text):
    """(the mission sections, demoted to `##`, the rest of the file, and
    which of mission.md and design.md each heading's anchor lands in)."""
    preamble, sections = split_sections(text)
    level = len(sections[0][1]) - len(sections[0][1].lstrip("#")) if sections else 2
    taken, rest = {}, preamble
    for title, head, content in sections:
        name = mission_section(title)
        if name and name not in taken:
            taken[name] = shift_headings(content, 2 - level)
        else:
            rest += head + content
    # A reference definition goes with each part that uses it.
    definitions = {m.group(1).lower(): m.group(0) for m in REFERENCE.finditer(text)}

    def missing(part):
        defined = {m.group(1).lower() for m in REFERENCE.finditer(part)}
        used = [label for label in sorted(definitions) if f"[{label}]" in part.lower() and label not in defined]
        return "".join(f"\n{definitions[label]}\n" for label in used)

    if taken:
        last = [name for name in MISSION_SECTIONS if name in taken][-1]
        taken[last] = taken[last].rstrip("\n") + "\n" + missing("".join(taken.values()))
    rest += missing(rest)
    where = {a: "design.md" for a in anchors(rest)}
    where.update({a: "mission.md" for a in anchors("".join(f"# {t}\n{c}" for t, c in taken.items()))})
    return taken, rest, where


def mission_files(rid, r, sections, preamble, strategy, links, front):
    """mission.md, and design.md and notes.md where they have text."""
    new_dir = f".atelier/missions/{rid}"
    path, taken, rest, where = strategy or (None, {}, "", {})
    own, notes = {}, []
    for title, content in sections:
        name = mission_section(title)
        if name and name not in taken and name not in own:
            own[name] = content
        else:
            notes.append((title, content))

    def fix(text, src, name, anchors_=None):
        return links.rewrite(text, src, f"{new_dir}/{name}", anchors_)

    old_file = f".atelier/issues/{rid}.md"
    body = []
    for name in MISSION_SECTIONS:
        if name in taken:
            body.append((name, fix(taken[name], path, "mission.md", ("mission.md", where))))
        elif name in own:
            body.append((name, fix(own[name], old_file, "mission.md")))
    if "Outcome" not in taken and "Outcome" not in own:
        body.insert(1 if body and body[0][0] == "Why" else 0, ("Outcome", UNSPECIFIED))
    files = {f"{new_dir}/mission.md": front_matter([
        ("schema", "atelier.mission"), ("schema_version", 3), ("id", rid), ("title", yaml_string(r["title"])),
        ("status", front["status"]), ("labels", front["labels"]), ("lead", None), ("ready", front["ready"]),
        ("created_at", r["created_at"])]) + render_body("", body)}
    if rest.strip():
        files[f"{new_dir}/design.md"] = fix(rest, path, "design.md", ("design.md", where))
    if notes or preamble.strip():
        notes = [(t, fix(c, old_file, "notes.md")) for t, c in notes]
        files[f"{new_dir}/notes.md"] = (f"# Notes of {rid}\n\nWhat the old mission record held beyond its mission "
                                        "sections.\n" + render_body(fix(preamble, old_file, "notes.md"), notes))
    return files


def item_file(rid, r, sections, preamble, graph, links, front):
    if r["issue_type"] == "decision":
        sections = [("Goal" if t == "Question" else t, c) for t, c in sections]
    first, others = {}, []
    for title, content in sections:
        if title in ("Goal", "Done when", "Scope") and title not in first:
            first[title] = content
        else:
            others.append((title, content))
    if r["issue_type"] == "decision":
        first.setdefault("Done when", DECISION_DONE_WHEN)
    ordered = [(t, first.get(t, UNSPECIFIED)) for t in ("Goal", "Done when")]
    ordered += [("Scope", first["Scope"])] if "Scope" in first else []
    new_file = f".atelier/items/{rid}/item.md"
    text = front_matter([
        ("schema", "atelier.item"), ("schema_version", 3), ("id", rid), ("title", yaml_string(r["title"])),
        ("status", front["status"]), ("mission", graph.mission_of.get(rid)), ("labels", front["labels"]),
        ("priority", PRIORITY[r["priority"]]), ("blocked_by", graph.blockers(rid)), ("implementers", []),
        ("ready", front["ready"]), ("trivial", None), ("caused_by", None), ("created_at", r["created_at"])])
    return {new_file: text + links.rewrite(render_body(preamble, ordered + others), f".atelier/issues/{rid}.md", new_file)}


def convert(root):
    """({path: text} to write, [paths] to remove, report lines) for the old
    records under `root`. Raises Unreadable."""
    tracked = subprocess.run(["git", "-C", root, "ls-files", "-z"], capture_output=True, text=True,
                             check=True).stdout.split("\0")
    tracked = sorted(p for p in tracked if p)
    config = read_config(root)
    records = read_records(root)
    graph = Graph(records)

    strategies, moved = {}, {}
    for rid, r in records.items():
        if r["issue_type"] == "mission" and r["status"] in OPEN and r.get("strategy"):
            path = posixpath.normpath(r["strategy"]["path"])
            if path not in tracked:
                raise Unreadable(f".atelier/issues/{rid}.md: its strategy {path} is not a tracked file")
            taken, rest, where = split_strategy(read(root, path))
            strategies[rid] = (path, taken, rest, where)
            moved[path] = (f".atelier/missions/{rid}", where)
    renamed = {f".atelier/issues/{rid}.md": f".atelier/missions/{rid}/mission.md" if r["issue_type"] == "mission"
               else f".atelier/items/{rid}/item.md" for rid, r in records.items() if r["status"] in OPEN}
    links = Links(tracked, moved, renamed)

    writes = {".atelier/config.toml": config}
    count = {"converted": {}, "left": {}}
    report = {"in_progress": [], "as_text": []}
    closed_blockers = 0
    for rid, r in sorted(records.items()):
        kind, status = r["issue_type"], r["status"]
        if status not in OPEN:
            count["left"][f"{kind} {status}"] = count["left"].get(f"{kind} {status}", 0) + 1
            continue
        if status == "in_progress":
            report["in_progress"].append(rid)
        closed_blockers += sum(1 for b in graph.blocked_by.get(rid, ()) if not graph.is_open(b))
        preamble, sections = split_sections(r["body"])
        sections = [(t, c) for t, _, c in sections]
        as_text = graph.as_text(rid)
        if as_text:
            lines = [f"- {label}: {other}" + (f" ({records[other]['title']})" if other in records else "")
                     for label, other in as_text]
            sections.append(("Links", "Relationships of the old record that the new format has no field for.\n\n"
                             + "\n".join(lines)))
            report["as_text"].append(f"{rid}: " + ", ".join(f"{label} {other}" for label, other in as_text))
        status = NEW_STATUS[status]
        front = {"status": status,
                 "ready": {"by": "conversion", "at": r["updated_at"]} if status == "ready" else None,
                 "labels": list(dict.fromkeys((r.get("labels") or [])
                                              + ([kind] if kind not in ("task", "mission") else [])
                                              + (["needs-human"] if kind == "decision" else [])))}
        if kind == "mission":
            writes.update(mission_files(rid, r, sections, preamble, strategies.get(rid), links, front))
            name = "mission"
        else:
            writes.update(item_file(rid, r, sections, preamble, graph, links, front))
            name = "item from decision" if kind == "decision" else "item"
        count["converted"][f"{name} {status}"] = count["converted"].get(f"{name} {status}", 0) + 1
    return writes, sorted(moved), summary(root, count, report, closed_blockers, moved, graph)


def summary(root, count, report, closed_blockers, moved, graph):
    lines = ["Records converted, by kind and new status:"]
    lines += [f"  {k}: {v}" for k, v in sorted(count["converted"].items())]
    lines += ["Records left behind, by kind and status (Git keeps them):"]
    lines += [f"  {k}: {v}" for k, v in sorted(count["left"].items())]
    lines += [f"  {d} files: {sum(len(fs) for _, _, fs in os.walk(os.path.join(root, '.atelier', d)))}"
              for d in ("reviews", "evidence", "runtime")]
    lines += [f"  blocked_by entries naming a closed or missing record: {closed_blockers}"]
    lines += ["Made ready from in_progress; each branch's work is not converted:"]
    lines += [f"  {rid}" for rid in report["in_progress"]] or ["  (none)"]
    lines += ["Relationships with no field in the new format, written in the record's Links section:"]
    lines += [f"  {line}" for line in report["as_text"]] or ["  (none)"]
    lines += ["Items that advance no mission, given their nearest ancestor's (the parent is in Links):"]
    lines += [f"  {rid}: {graph.mission_of[rid]}, through {a}" for rid, a in sorted(graph.through.items())] or ["  (none)"]
    lines += ["Strategy files moved:"]
    lines += [f"  {old} -> {moved[old][0]}/" for old in sorted(moved)] or ["  (none)"]
    naming = files_naming(root, moved)
    lines += ["Tracked files that still name a moved file's old path, left unchanged:"]
    lines += [f"  {f}: {', '.join(olds)}" for f, olds in sorted(naming.items())] or ["  (none)"]
    return lines


# ---------------------------------------------------------------- commands


def git_root():
    out = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True)
    if out.returncode:
        sys.exit("convert_records.py runs inside a Git repository")
    return out.stdout.strip()


def apply(root):
    writes, removes, lines = convert(root)
    shutil.rmtree(os.path.join(root, ".atelier"))
    for rel, text in sorted(writes.items()):
        os.makedirs(os.path.dirname(os.path.join(root, rel)), exist_ok=True)
        with open(os.path.join(root, rel), "w", encoding="utf-8", newline="") as f:
            f.write(text)
    for rel in removes:
        os.remove(os.path.join(root, rel))
    print("Replaced .atelier/ with the new layout. Nothing was committed.")
    print("\n".join(lines))


def verify(root, commit):
    sha = subprocess.run(["git", "-C", root, "rev-parse", "--verify", f"{commit}^{{commit}}"],
                         capture_output=True, text=True)
    if sha.returncode:
        sys.exit(f"`{commit}` names no commit")
    with tempfile.TemporaryDirectory() as tmp:
        fresh = os.path.join(tmp, "old")
        for cmd in (["git", "clone", "-q", "--no-checkout", "--shared", root, fresh],
                    ["git", "-C", fresh, "checkout", "-q", "--detach", sha.stdout.strip()]):
            subprocess.run(cmd, check=True, capture_output=True)
        writes, removes, _ = convert(fresh)
    # Every file under .atelier/, ignored by Git or not, since none of the old layout may remain.
    present = sorted(os.path.relpath(os.path.join(d, f), root) for d, _, fs in os.walk(os.path.join(root, ".atelier"))
                     for f in fs)
    problems = []
    for rel in sorted(set(present) | set(writes)):
        if rel not in writes:
            problems.append(f"  not in a fresh conversion: {rel}")
        elif rel not in present:
            problems.append(f"  missing: {rel}")
        else:
            with open(os.path.join(root, rel), "rb") as f:
                if f.read() != writes[rel].encode("utf-8"):
                    problems.append(f"  differs: {rel}")
    problems += [f"  still present, though moved: {rel}" for rel in removes if os.path.exists(os.path.join(root, rel))]
    if problems:
        print(f"This tree does not match a fresh conversion of {commit}:")
        print("\n".join(problems))
        sys.exit(1)
    print(f"This tree's .atelier/ and moved files match a fresh conversion of {commit}: {len(writes)} files.")


def main(argv):
    if argv and (len(argv) != 2 or argv[0] != "--verify"):
        sys.exit(__doc__)
    root = git_root()
    try:
        verify(root, argv[1]) if argv else apply(root)
    except Unreadable as e:
        sys.exit(f"Cannot read an old record, so nothing was written: {e}")


if __name__ == "__main__":
    main(sys.argv[1:])
