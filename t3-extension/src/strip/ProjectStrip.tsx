// The project strip's cards, drawn with T3's components: Waiting on you, In motion and Next up,
// from GET /api/launch?repo=<repo>. Fields the Desk does not send count as empty. A Next up row
// puts a prompt in the composer; it is never sent.
import type { ReactNode } from "react";
import type { LaunchResponse } from "../../../desk/src/api/LaunchResponse";
import type { ThreadEntry } from "../host/adapter";
import { useDeskRead } from "../react/desk";
import type { Repository } from "../repositories";
import { Badge } from "../t3/ui/badge";
import { Button, InlineButton } from "../t3/ui/button";

export type ProjectStripProps = {
  repo: Repository;
  /** Below 700px of composer width the In motion card goes; the composer-filling Next up card stays. */
  narrow: boolean;
  /** Why T3's composer cannot be filled, or null when it can. */
  composerMissing: string | null;
  /** T3's threads, by ID, as they are when the launch data is drawn. */
  threads(): Map<string, ThreadEntry>;
  openInbox(): void;
  openItem(id: string): void;
  openHome(): void;
  showThread(thread: ThreadEntry): void;
  /** Puts `text` in the composer of the draft the strip is drawn for. */
  fill(text: string): void;
};

// An item ID as Atelier writes them: lowercase letters and digits in dash-separated parts.
const itemIdPattern = /^[a-z0-9]+(?:-[a-z0-9]+)+$/;
const age = (seconds: number) => (seconds < 3600 ? `${Math.max(1, Math.round(seconds / 60))}m` : seconds < 86400 ? `${Math.round(seconds / 3600)}h` : `${Math.round(seconds / 86400)}d`);

export function ProjectStrip({ repo, narrow, composerMissing, threads, openInbox, openItem, openHome, showThread, fill }: ProjectStripProps) {
  const path = `/api/launch?repo=${encodeURIComponent(repo.repo)}`;
  const { data, error } = useDeskRead<LaunchResponse>(repo.desk, path, 60000);
  if (!data) return <Note error={Boolean(error)}>{error ? `GET ${repo.desk}${path}: ${error.message}` : `Reading ${repo.repo} from the Desk…`}</Note>;

  const ready = data.ready ?? [];
  const waitingItems = data.waiting_items ?? [];
  const inMotion = data.in_motion ?? [];
  const waiting = data.waiting ?? waitingItems.length;
  const t3 = threads();
  const shown = waitingItems.slice(0, 2);
  const more = waiting - shown.length;
  const waitingRows = shown.map((item) => (
    <Row key={item.id} title={`${item.id}: ${item.title}`} onClick={openInbox}>
      <span className="truncate">{item.title}</span>
    </Row>
  ));
  if (more > 0)
    waitingRows.push(
      <Row key="more" title="Open the inbox" onClick={openInbox}>
        <Faint>{shown.length ? `${more} more` : `${waiting} waiting · open the inbox`}</Faint>
      </Row>,
    );
  const motionRows = inMotion.slice(0, 3).map((item) => {
    const thread = item.holder ? t3.get(item.holder.identity) : undefined;
    const holder = item.holder && (item.holder.title ?? item.holder.identity);
    const meta = [item.stage, item.age_seconds != null && age(Number(item.age_seconds))].filter(Boolean).join(" · ");
    return (
      <Row key={item.id} title={`${item.id}: ${item.title}${holder ? `, held by ${holder}` : ""}${thread ? "" : " (opens in the Desk)"}`} onClick={() => (thread ? showThread(thread) : openItem(item.id))}>
        <Line id={item.id} title={item.title} meta={meta} />
        {holder ? <Faint>{holder}</Faint> : null}
      </Row>
    );
  });
  const nextRows = ready.slice(0, 3).map((item) => {
    // The prompt is built only from an ID of Atelier's form; any other ID is shown but cannot be used.
    const valid = itemIdPattern.test(item.id);
    const usable = valid && composerMissing === null;
    const title = usable ? `${item.id}: ${item.title}${item.priority ? ` (${item.priority})` : ""}. Puts a start prompt in the composer` : `${item.id}: ${valid ? `T3's composer is not available (${composerMissing})` : "not an Atelier item ID, so no prompt is made from it"}`;
    return (
      <Row key={item.id} title={title} onClick={usable ? () => fill(`Start ${item.id}: run \`atelier start ${item.id}\` in a fresh checkout and follow what it prints.`) : undefined}>
        <Line id={item.id} title={item.title} />
      </Row>
    );
  });
  return (
    <div className="flex h-full flex-col gap-2 px-0.5 text-foreground text-sm">
      <div className="flex h-6 shrink-0 items-center justify-between gap-2 px-1">
        <InlineButton tone="muted" className="text-xs underline decoration-current/35 underline-offset-3" data-t3-strip-project="" title={`Open the ${repo.repo} project home`} onClick={openHome}>
          {repo.repo}
        </InlineButton>
        {error ? (
          <span className="min-w-0 flex-1 truncate text-destructive-foreground text-xs" data-t3-strip-note="" data-error="" title={error.message}>
            {`Not read again: ${error.message}`}
          </span>
        ) : null}
        <Button size="xs" variant="outline" data-t3-strip-plan="" title="Put a planning prompt in the composer" disabled={composerMissing !== null} onClick={() => fill("Let's plan a mission for ")}>
          Plan a mission
        </Button>
      </div>
      <div className={`grid min-h-0 flex-1 gap-2 ${narrow ? "grid-cols-2" : "grid-cols-3"}`}>
        <Card name="waiting" title="Waiting on you" count={waiting} empty="Nothing waiting on you" onHeader={openInbox} rows={waitingRows} />
        <Card name="motion" title="In motion" count={inMotion.length} empty="Nothing in motion" hidden={narrow} rows={motionRows} />
        <Card name="next" title="Next up" count={ready.length} empty="Nothing ready" rows={nextRows} />
      </div>
    </div>
  );
}

function Card({ name, title, count, empty, onHeader, hidden, rows }: { name: string; title: string; count: number; empty: string; onHeader?: () => void; hidden?: boolean; rows: ReactNode[] }) {
  const head = (
    <>
      <span>{title}</span>
      <Badge variant="secondary" size="sm" className="tabular-nums" data-t3-card-count="">
        {count}
      </Badge>
    </>
  );
  const headClass = "h-6 shrink-0 justify-start gap-1.5 px-1.5 font-medium text-xs";
  return (
    <div data-t3-card={name} className={`${hidden ? "hidden" : "flex"} min-h-0 min-w-0 flex-col gap-px overflow-hidden rounded-xl border bg-card px-1.5 py-2`}>
      {onHeader ? (
        <Button variant="ghost-muted" size="xs" className={headClass} data-t3-card-head="" onClick={onHeader}>
          {head}
        </Button>
      ) : (
        <div className={`flex items-center text-muted-foreground ${headClass}`} data-t3-card-head="">
          {head}
        </div>
      )}
      {rows.length ? rows : <div className="px-1.5 py-1 text-muted-foreground text-xs">{empty}</div>}
    </div>
  );
}

/** A card's row: a button, disabled when it does nothing. */
function Row({ title, onClick, children }: { title: string; onClick?: () => void; children: ReactNode }) {
  return (
    <Button variant="ghost" size="sm-multiline" className="h-auto min-h-0 w-full min-w-0 shrink-0 flex-col items-stretch justify-center gap-0 px-1.5 py-1 text-start font-normal leading-tight sm:min-h-0" data-t3-card-row="" title={title} disabled={!onClick} onClick={onClick}>
      {children}
    </Button>
  );
}

function Line({ id, title, meta }: { id: string; title: string; meta?: string }) {
  return (
    <span className="flex min-w-0 items-baseline gap-1.5">
      <code className="shrink-0 text-[11px] text-muted-foreground">{id}</code>
      <span className="min-w-0 flex-1 truncate">{title}</span>
      {meta ? <span className="shrink-0 text-[11px] text-muted-foreground tabular-nums">{meta}</span> : null}
    </span>
  );
}

const Faint = ({ children }: { children: ReactNode }) => <span className="truncate text-[11px] text-muted-foreground">{children}</span>;

function Note({ error, children }: { error: boolean; children: ReactNode }) {
  return (
    <div className={`break-words p-1 text-xs ${error ? "text-destructive-foreground" : "text-muted-foreground"}`} data-t3-strip-note="" data-error={error ? "" : undefined}>
      {children}
    </div>
  );
}
