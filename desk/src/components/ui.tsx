import { Circle, CircleCheck, CircleDashed, CircleDot, CircleX, Eye, RotateCcw, ShieldCheck, TriangleAlert } from "lucide-react";
import { Children, isValidElement, type ReactNode } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Link, useSearchParams } from "react-router";
import type { Priority as PriorityLevel } from "../api/Priority";
import type { StatusCount } from "../api/StatusCount";
import type { Loaded } from "../lib/api";
import { resolveLink, slug } from "../lib/links";
import { countsLine, statusName, statusTitle } from "../lib/words";

const GLYPHS: Record<string, [typeof Circle, string]> = {
  draft: [CircleDashed, "dim"],
  ready: [Circle, "plain"],
  building: [CircleDot, "run"],
  in_review: [Eye, "review"],
  changes_requested: [RotateCcw, "bad"],
  approved: [ShieldCheck, "ok"],
  done: [CircleCheck, "ok"],
  dropped: [CircleX, "dim"],
};

/**
 * Where work stands, as one glyph, the same for items and missions.
 * `building` on a mission means work in progress. `quiet` when the status
 * is also written beside it, so a screen reader hears it once.
 */
export function StatusGlyph({ status, size = 15, label, quiet }: { status: string; size?: number; label?: string; quiet?: boolean }) {
  const [Icon, tone] = GLYPHS[status] ?? [CircleDashed, "dim"];
  const words = label ?? statusTitle(status as never);
  if (quiet) {
    return (
      <span className={`glyph ${tone}`} aria-hidden>
        <Icon size={size} strokeWidth={2} />
      </span>
    );
  }
  return (
    <span className={`glyph ${tone}`} title={words}>
      <Icon size={size} strokeWidth={2} aria-hidden />
      <span className="sr">{words}</span>
    </span>
  );
}

const LEVELS: Record<PriorityLevel, number> = { low: 1, medium: 2, high: 3, critical: 3 };

export function Priority({ priority }: { priority: PriorityLevel }) {
  const level = LEVELS[priority];
  return (
    <span className={`prio${priority === "critical" ? " hot" : ""}`} title={`${priority} priority`}>
      {[1, 2, 3].map((n) => (
        <i key={n} className={n <= level ? "on" : ""} style={{ height: 4 + n * 2.6 }} />
      ))}
      <span className="sr">{priority} priority</span>
    </span>
  );
}

/** A mission's items as one bar, a segment per status sized by its count. */
export function ScopeBar({ counts, tall }: { counts: StatusCount[]; tall?: boolean }) {
  if (counts.length === 0) {
    return (
      <span className={`seg none${tall ? " tall" : ""}`} role="img" aria-label="No items">
        <i />
      </span>
    );
  }
  const label = countsLine(counts);
  return (
    <span className={`seg${tall ? " tall" : ""}`} role="img" aria-label={label} title={label}>
      {counts.map((c) => (
        <i key={c.status} className={c.status} style={{ flex: c.count }} />
      ))}
    </span>
  );
}

/** The counts per status, spelled out, as `atelier show` lists them. */
export function Counts({ counts }: { counts: StatusCount[] }) {
  if (counts.length === 0) return <span className="legend muted">No items yet</span>;
  return (
    <span className="legend" data-testid="counts">
      {counts.map((c) => (
        <span key={c.status} data-status={c.status}>
          <span className="seg" style={{ width: 8, minWidth: 8, height: 8 }} aria-hidden>
            <i className={c.status} style={{ flex: 1 }} />
          </span>
          <span className="num">{c.count}</span> {statusName(c.status)}
        </span>
      ))}
    </span>
  );
}

function textOf(node: ReactNode): string {
  return Children.toArray(node)
    .map((child) => (typeof child === "string" || typeof child === "number" ? String(child) : isValidElement<{ children?: ReactNode }>(child) ? textOf(child.props.children) : ""))
    .join("");
}

function heading(Tag: "h1" | "h2" | "h3" | "h4") {
  return ({ children }: { children?: ReactNode }) => <Tag id={slug(textOf(children))}>{children}</Tag>;
}

/**
 * Markdown from the repository, safe to show: no raw HTML. A relative link
 * is resolved against `from`, the file the text is in; `docHref` turns a
 * Markdown file into a page of the Desk, and other files show as text.
 */
export function Markdown({ text, small, from, docHref }: { text: string; small?: boolean; from?: string; docHref?: (path: string, hash: string) => string | null }) {
  return (
    <div className={`md${small ? " small" : ""}`}>
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        skipHtml
        urlTransform={(url) => url}
        components={{
          h1: heading("h1"),
          h2: heading("h2"),
          h3: heading("h3"),
          h4: heading("h4"),
          a: ({ href, children }) => {
            if (!href) return <span>{children}</span>;
            const target = resolveLink(from ?? "", href);
            if (target.kind === "external") {
              return /^(https?|mailto):/i.test(target.href) ? (
                <a href={target.href} target="_blank" rel="noreferrer">
                  {children}
                </a>
              ) : (
                <span>{children}</span>
              );
            }
            if (target.kind === "anchor") return <a href={`#${target.hash}`}>{children}</a>;
            if (target.kind === "doc" && from !== undefined) {
              const page = docHref?.(target.path, target.hash);
              if (page) return <Link to={page}>{children}</Link>;
            }
            const path = target.kind === "doc" || target.kind === "file" ? target.path : href;
            return (
              <span className="file-ref" title={`${path} is not shown on the Desk`}>
                {children}
              </span>
            );
          },
        }}
      >
        {text}
      </ReactMarkdown>
    </div>
  );
}

export function ErrorBanner({ loaded }: { loaded: Loaded<unknown> }) {
  if (!loaded.error) return null;
  return (
    <div className="banner" role="alert">
      <TriangleAlert size={15} aria-hidden />
      <span>{loaded.error}</span>
    </div>
  );
}

export function Skeleton({ rows = 4 }: { rows?: number }) {
  return (
    <div className="list" aria-busy="true">
      {Array.from({ length: rows }, (_, at) => (
        <div key={at} className="skel" />
      ))}
    </div>
  );
}

export function Empty({ title, children }: { title: string; children?: ReactNode }) {
  return (
    <div className="empty">
      <b>{title}</b>
      {children}
    </div>
  );
}

export function Card({ title, tally, children, flush }: { title: string; tally?: ReactNode; children: ReactNode; flush?: boolean }) {
  return (
    <section className="card">
      <header>
        <h3>{title}</h3>
        {tally !== undefined ? <span className="tally">{tally}</span> : null}
      </header>
      <div className={flush ? "card-flush" : "card-body"}>{children}</div>
    </section>
  );
}

/** A tab that lives in the query string, so a view can be linked. */
export function useTab<T extends string>(name: string, tabs: readonly T[], fallback: T): [T, (tab: T) => void] {
  const [params, setParams] = useSearchParams();
  const asked = params.get(name) as T | null;
  const tab = asked && tabs.includes(asked) ? asked : fallback;
  return [
    tab,
    (next) => {
      const p = new URLSearchParams(params);
      if (next === fallback) p.delete(name);
      else p.set(name, next);
      setParams(p, { replace: true });
    },
  ];
}

export function Tabs<T extends string>({ tabs, value, onChange }: { tabs: [T, ReactNode][]; value: T; onChange: (tab: T) => void }) {
  return (
    <div className="tabs" role="tablist">
      {tabs.map(([key, label]) => (
        <button key={key} type="button" role="tab" className="tab" aria-selected={key === value} onClick={() => onChange(key)}>
          {label}
        </button>
      ))}
    </div>
  );
}
