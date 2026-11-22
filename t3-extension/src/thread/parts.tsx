// The pieces This thread and the project at a glance share: section headings, notes, status dots
// and the row that opens a record.
import { Target } from "lucide-react";
import type { ReactNode } from "react";
import * as m from "./model";

export function Note({ children }: { children: ReactNode }) {
  return (
    <p className="max-w-[75ch] text-destructive-foreground text-xs" data-note="">
      {children}
    </p>
  );
}

/** A section heading: a label, a rule and a count, as T3's section headers. */
export function Heading({ children, count, warn }: { children: ReactNode; count?: ReactNode; warn?: boolean }) {
  return (
    <h2 className={`mt-3 flex h-8 items-center gap-2 font-medium text-xs ${warn ? "text-warning-foreground" : "text-muted-foreground"}`}>
      <span className="shrink-0">{children}</span>
      <span aria-hidden className="h-px min-w-2 flex-1 bg-border" />
      {count !== undefined ? <span className="shrink-0 font-normal tabular-nums">{count}</span> : null}
    </h2>
  );
}

export const dotClass: Record<string, string> = {
  open: "bg-muted-foreground",
  building: "bg-info",
  in_review: "bg-violet-500",
  changes_requested: "bg-warning",
  approved: "bg-success",
  done: "bg-success/50",
  dropped: "border border-muted-foreground/40",
};

/** A record's status as a dot, named for a screen reader. */
export function StatusDot({ status }: { status: string }) {
  return <span className={`size-2 shrink-0 justify-self-center rounded-full ${dotClass[status] ?? "bg-muted-foreground/50"}`} role="img" aria-label={m.statusTitle(status)} title={m.statusTitle(status)} />;
}

export function MissionGlyph({ tone }: { tone: "you" | "run" | "done" | "plain" }) {
  const color = { you: "text-warning-foreground", run: "text-info-foreground", done: "text-success-foreground", plain: "text-muted-foreground" }[tone];
  return <Target className={`size-3.5 shrink-0 ${color}`} aria-hidden />;
}

/** One record as a row that opens it: glyph, ID, title, and a short note on the right. */
export function Row({ id, row, title, glyph, note, onClick, dim }: { id: string; row: string; title: string; glyph: ReactNode; note?: ReactNode; onClick(): void; dim?: boolean }) {
  return (
    <button
      type="button"
      data-row={row}
      onClick={onClick}
      title={`${id}: ${title}`}
      className={`grid h-7 w-full cursor-pointer grid-cols-[14px_auto_minmax(0,1fr)_auto] items-center gap-2 rounded-md px-1.5 text-left outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset ${dim ? "opacity-70" : ""}`}
    >
      {glyph}
      <span className="font-mono text-muted-foreground text-xs">{id}</span>
      <span className="truncate">{title}</span>
      <span className="flex items-center gap-1.5 text-muted-foreground text-xs">{note}</span>
    </button>
  );
}
