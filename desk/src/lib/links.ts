/** A link in a Markdown file, as the Desk can follow it. */
export type Target =
  | { kind: "external"; href: string }
  | { kind: "anchor"; hash: string }
  | { kind: "doc"; path: string; hash: string }
  | { kind: "file"; path: string }
  | { kind: "outside" };

/**
 * Resolves `href`, written in the repository file at `from`, against that
 * file's directory. A Markdown file becomes a document the Desk can show;
 * another file is named but not opened; a path that climbs out of the
 * repository is refused.
 */
export function resolveLink(from: string, href: string): Target {
  if (/^[a-z][a-z0-9+.-]*:/i.test(href) || href.startsWith("//")) return { kind: "external", href };
  if (href.startsWith("#")) return { kind: "anchor", hash: href.slice(1) };
  const at = href.indexOf("#");
  const bare = at === -1 ? href : href.slice(0, at);
  const hash = at === -1 ? "" : href.slice(at + 1);
  const parts = bare.startsWith("/") ? [] : from.split("/").slice(0, -1);
  for (const part of bare.split("/")) {
    if (part === "..") {
      if (parts.length === 0) return { kind: "outside" };
      parts.pop();
    } else if (part && part !== ".") {
      parts.push(decodePart(part));
    }
  }
  const path = parts.join("/");
  if (!path) return { kind: "outside" };
  return /\.md$/i.test(path) ? { kind: "doc", path, hash } : { kind: "file", path };
}

function decodePart(part: string): string {
  try {
    return decodeURIComponent(part);
  } catch {
    return part;
  }
}

/** `design.md` from `.atelier/missions/x/design.md`. */
export function fileName(path: string): string {
  return path.split("/").pop() ?? path;
}

/** Where an item's and a mission's records live, for resolving links in their text. */
export const recordPath = {
  mission: (id: string) => `.atelier/missions/${id}/mission.md`,
  item: (id: string) => `.atelier/items/${id}/item.md`,
};

/**
 * A Markdown file's YAML front matter, as top-level `key: value` lines,
 * and the text after it. A file without front matter is all body.
 */
export function splitFrontMatter(markdown: string): { fields: [string, string][]; body: string } {
  const match = /^---\r?\n([\s\S]*?)\r?\n---\r?\n?/.exec(markdown);
  if (!match) return { fields: [], body: markdown };
  const fields: [string, string][] = [];
  for (const line of match[1].split(/\r?\n/)) {
    const field = /^([A-Za-z_][\w-]*):\s*(.*)$/.exec(line);
    if (!field) continue;
    let value = field[2].trim();
    if (/^(["']).*\1$/.test(value)) value = value.slice(1, -1).replaceAll('\\"', '"');
    fields.push([field[1], value]);
  }
  return { fields, body: markdown.slice(match[0].length) };
}

/** GitHub's heading anchors: lower case, punctuation dropped, spaces as hyphens. */
export function slug(text: string): string {
  return text
    .trim()
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\s_-]/gu, "")
    .replace(/\s/g, "-");
}
