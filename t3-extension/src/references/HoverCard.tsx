// The card a record link shows: what the ID names and where it stands, read from the Desk, in
// T3's PreviewCard, placed at the ID in the conversation (references.tsx).
import { useEffect, useMemo, useState } from "react";
import { messageOf } from "../host/api";
import { Button } from "../t3/ui/button";
import { PreviewCard, PreviewCardPopup } from "../t3/ui/preview-card";
import type { RecordDetails, Reference, ResolvedRecord } from "./references";

export type HoverCardProps = {
  reference: Reference;
  resolve(ref: Reference): Promise<ResolvedRecord>;
  readDetails(record: ResolvedRecord): Promise<RecordDetails>;
  /** The record's normal address on its Desk; read again once it is resolved, which may name another Desk or kind. */
  externalUrl(ref: Reference): string;
  /** Opens the record in the Desk. */
  onOpen(ref: Reference): void;
  /** Forgets what was read of it and shows the card again. */
  onReload(ref: Reference): void;
  onClose(): void;
  /** Whether the pointer is over the card, which keeps it open. */
  onPointer(over: boolean): void;
  popupRef(node: HTMLElement | null): void;
};

type State<T> = { value: T } | { error: string } | null;

/** The card's further lines, from the record's own page. */
function detailLines(record: ResolvedRecord, details: RecordDetails): { status: string; lines: { text: string; muted: boolean }[] } {
  const lines: { text: string; muted: boolean }[] = [];
  if (details.kind === "mission") {
    // A mission's page has no priority, holder, checks, blockers or goal.
    return { status: [record.kind, record.status].join(" · "), lines: [{ text: "Read from Desk · cached up to 30 seconds", muted: true }] };
  }
  const { page } = details;
  const row = page.row;
  const status = [record.kind, record.status, row.priority, row.holder && `Held by ${row.holder}`].filter(Boolean).join(" · ");
  if (page.done_when.length) lines.push({ text: `${page.done_when.filter((line) => line.ticked).length}/${page.done_when.length} completion checks`, muted: true });
  if (row.waiting_on.length || page.blocked_by.length) lines.push({ text: `Blocked by ${(row.waiting_on.length ? row.waiting_on : page.blocked_by).join(", ")}`, muted: false });
  if (page.goal) lines.push({ text: page.goal.slice(0, 240), muted: true });
  lines.push({ text: `Read from ${page.read_from ?? "Desk"} · cached up to 30 seconds`, muted: true });
  return { status, lines };
}

export function HoverCard({ reference, resolve, readDetails, externalUrl, onOpen, onReload, onClose, onPointer, popupRef }: HoverCardProps) {
  const [record, setRecord] = useState<State<ResolvedRecord>>(null);
  const [details, setDetails] = useState<State<RecordDetails>>(null);
  useEffect(() => {
    // A card shown for another reference, or again, drops what this one read.
    let live = true;
    void (async () => {
      let resolved: ResolvedRecord;
      try {
        resolved = await resolve(reference);
      } catch (error) {
        if (live) setRecord({ error: messageOf(error) });
        return;
      }
      if (!live) return;
      setRecord({ value: resolved });
      reference.anchor.setAttribute("aria-label", `${reference.id}: ${resolved.title}`);
      try {
        const read = await readDetails(resolved);
        if (live) setDetails({ value: read });
      } catch (error) {
        if (live) setDetails({ error: messageOf(error) });
      }
    })();
    return () => {
      live = false;
    };
  }, [reference, resolve, readDetails]);
  // The ID's place in the conversation; Base UI follows it as the conversation scrolls.
  const anchor = useMemo(() => ({ getBoundingClientRect: () => reference.range.getBoundingClientRect(), contextElement: reference.node.parentElement ?? undefined }), [reference]);

  // Resolving fills in the reference's Desk and kind, so the address is read after it.
  const external = externalUrl(reference);
  let body;
  if (!record) {
    body = (
      <>
        <strong>{reference.id}</strong>
        <p className="text-muted-foreground">Loading from Desk…</p>
      </>
    );
  } else if ("error" in record) {
    body = (
      <>
        <strong>{reference.id}</strong>
        <p className="whitespace-pre-wrap text-destructive-foreground [overflow-wrap:anywhere]">{record.error}</p>
        <div>
          <Button size="xs" variant="outline" onClick={() => onReload(reference)}>
            Retry
          </Button>
        </div>
      </>
    );
  } else {
    const found = record.value;
    const more = details && "value" in details ? detailLines(found, details.value) : null;
    body = (
      <>
        <div className="flex items-center gap-2">
          <strong className="min-w-0 flex-1 font-mono text-xs">{reference.id}</strong>
          <span className="text-xs text-muted-foreground">{found.repo}</span>
        </div>
        <h3 className="text-sm font-medium text-foreground">{found.title}</h3>
        <p className="text-xs">{more ? more.status : `${found.kind} · ${found.status}`}</p>
        <div className="flex flex-col gap-1 text-xs">
          {!details ? (
            <p className="text-muted-foreground">Reading details…</p>
          ) : "error" in details ? (
            <p className="whitespace-pre-wrap text-destructive-foreground [overflow-wrap:anywhere]">{details.error}</p>
          ) : (
            more?.lines.map((line, index) => (
              <p key={index} className={line.muted ? "text-muted-foreground" : undefined}>
                {line.text}
              </p>
            ))
          )}
        </div>
        <div className="flex flex-wrap items-center gap-2 pt-1">
          <Button size="xs" onClick={() => onOpen(reference)}>
            Open in Desk
          </Button>
          <Button size="xs" variant="outline" onClick={() => onReload(reference)}>
            Refresh
          </Button>
          <Button size="xs" variant="link" render={<a href={external} target="_blank" rel="noopener noreferrer" />}>
            Open externally ↗
          </Button>
        </div>
      </>
    );
  }

  return (
    <PreviewCard
      open
      onOpenChange={(open) => {
        if (!open) onClose();
      }}
    >
      <PreviewCardPopup
        ref={popupRef}
        id="atelier-hovercard"
        role="dialog"
        aria-label="Atelier reference details"
        side="bottom"
        align="start"
        sideOffset={8}
        anchor={anchor}
        className="flex max-h-[70vh] w-[360px] max-w-[calc(100vw-24px)] flex-col gap-2 overflow-auto p-3.5 text-[13px] leading-normal"
        onPointerEnter={() => onPointer(true)}
        onPointerLeave={() => onPointer(false)}
      >
        {body}
      </PreviewCardPopup>
    </PreviewCard>
  );
}
