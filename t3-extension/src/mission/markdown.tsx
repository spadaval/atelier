// A mission's sections as written (its Result, Outcome and Success criteria), drawn as React
// elements with no raw HTML. Records hold paragraphs, bullet and numbered lists, headings, inline
// code, emphasis and links, and that is all this reads; a link is shown as its text, since the page
// opens nothing but threads, items and pull requests. The Desk uses react-markdown, which is not a
// dependency the extension may add without asking.
import type { ReactNode } from "react";

type Block = { kind: "p" | "ul" | "ol" | "h"; lines: string[] };

function blocks(text: string): Block[] {
  const out: Block[] = [];
  let open = null as Block | null;
  for (const raw of text.split("\n")) {
    const line = raw.trimEnd();
    const bullet = /^\s*[-*+]\s+(.*)$/.exec(line);
    const numbered = /^\s*\d+[.)]\s+(.*)$/.exec(line);
    const heading = /^#{1,6}\s+(.*)$/.exec(line);
    if (!line.trim()) {
      open = null;
    } else if (heading) {
      out.push({ kind: "h", lines: [heading[1]] });
      open = null;
    } else if (bullet || numbered) {
      const kind = bullet ? "ul" : "ol";
      if (open?.kind !== kind) out.push((open = { kind, lines: [] }));
      open.lines.push((bullet ?? numbered)![1]);
    } else if (open && (open.kind === "ul" || open.kind === "ol") && /^\s+/.test(line)) {
      // A list item's continuation line.
      open.lines[open.lines.length - 1] += ` ${line.trim()}`;
    } else if (open?.kind === "p") {
      open.lines.push(line.trim());
    } else {
      out.push((open = { kind: "p", lines: [line.trim()] }));
    }
  }
  return out;
}

/** Inline code, bold, italics and links (as their text). */
function inline(text: string): ReactNode[] {
  const parts: ReactNode[] = [];
  const pattern = /`([^`]+)`|\*\*([^*]+)\*\*|\*([^*]+)\*|(?<![\w])_([^_]+)_(?![\w])|\[([^\]]+)\]\([^)]*\)/g;
  let at = 0;
  for (const match of text.matchAll(pattern)) {
    if (match.index > at) parts.push(text.slice(at, match.index));
    const key = parts.length;
    if (match[1] !== undefined)
      parts.push(
        <code key={key} className="font-mono text-[0.92em]">
          {match[1]}
        </code>,
      );
    else if (match[2] !== undefined)
      parts.push(
        <strong key={key} className="font-semibold">
          {match[2]}
        </strong>,
      );
    else if (match[3] !== undefined || match[4] !== undefined) parts.push(<em key={key}>{match[3] ?? match[4]}</em>);
    else parts.push(match[5]);
    at = match.index + match[0].length;
  }
  if (at < text.length) parts.push(text.slice(at));
  return parts;
}

function List({ numbered, lines }: { numbered: boolean; lines: string[] }) {
  const Tag = numbered ? "ol" : "ul";
  return (
    <Tag className={numbered ? "list-decimal ps-5" : "list-disc ps-5"}>
      {lines.map((line, n) => (
        <li key={n} className="py-0.5">
          {inline(line)}
        </li>
      ))}
    </Tag>
  );
}

/** `text` as paragraphs and lists, each at most 75 characters wide. */
export function Markdown({ text }: { text: string }) {
  return (
    <div className="flex max-w-[75ch] flex-col gap-2 text-sm leading-relaxed" data-markdown="">
      {blocks(text).map((block, at) =>
        block.kind === "p" ? (
          <p key={at}>{inline(block.lines.join(" "))}</p>
        ) : block.kind === "h" ? (
          <p key={at} className="font-medium">
            {inline(block.lines[0])}
          </p>
        ) : (
          <List key={at} numbered={block.kind === "ol"} lines={block.lines} />
        ),
      )}
    </div>
  );
}
