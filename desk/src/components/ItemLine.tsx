import { GitBranch, MessageCircleQuestion } from "lucide-react";
import type { ReactNode } from "react";
import { Link } from "react-router";
import type { ItemRow } from "../api/ItemRow";
import { to } from "../lib/api";
import { statusName } from "../lib/words";
import { Priority, StatusGlyph } from "./ui";

/** What an item needs next, in a few words: who holds it, what blocks it, what it lacks. */
export function itemFacts(row: ItemRow): string[] {
  const facts: string[] = [];
  if (row.dropped) facts.push(`Dropped: ${row.dropped}`);
  if (row.holder) facts.push(`Held by ${row.holder}`);
  if (row.reviewer) facts.push(`Reviewed by ${row.reviewer}`);
  // A ready item's unmet conditions already name its blockers.
  const waiting = row.status === "ready" ? [] : row.blockers.filter((b) => row.waiting_on.includes(b.id));
  if (waiting.length > 0) facts.push(`Waits on ${waiting.map((b) => `${b.id} (${b.status ? statusName(b.status) : "not found"})`).join(", ")}`);
  facts.push(...row.cannot_start, ...row.needs);
  if (row.status === "ready" && row.startable) facts.push("Can start now");
  return facts;
}

/** One item: status, ID and title, then what it needs. Links to the item's page. */
export function ItemLine({ repo, row, extra, after }: { repo: string; row: ItemRow; extra?: ReactNode; after?: ReactNode }) {
  const facts = itemFacts(row);
  return (
    <div className="iline">
      <Link to={to.item(repo, row.id)} className="item">
        <StatusGlyph status={row.status} />
        <span className="id">{row.id}</span>
        <span className="title">{row.title}</span>
        <span className="meta">
          {row.question ? (
            <span className="tag you" title={row.question}>
              <MessageCircleQuestion size={11} aria-hidden /> Question
            </span>
          ) : null}
          {row.needs_human ? <span className="tag you">Needs you</span> : null}
          {row.branch ? (
            <span className="tag opt" title={row.branch.merged ? "Its branch is merged" : "Its branch is not merged yet"}>
              <GitBranch size={11} aria-hidden /> {row.branch.merged ? "merged" : "branch"}
            </span>
          ) : null}
          {extra}
          <Priority priority={row.priority} />
        </span>
      </Link>
      {facts.length > 0 ? (
        <ul className="facts-line">
          {facts.map((fact) => (
            <li key={fact}>{fact}</li>
          ))}
        </ul>
      ) : null}
      {after}
    </div>
  );
}
