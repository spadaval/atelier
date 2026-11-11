import { CircleCheck, MessageSquareWarning, RotateCcw } from "lucide-react";
import type { FindingRow } from "../api/FindingRow";
import type { Review } from "../api/Review";
import { dateTime, plural, recommendationWords, shortCommit, who } from "../lib/words";
import { Markdown } from "./ui";

/** What became of a finding, in words: fixed, deferred and taken as an item, and so on. */
export function findingState(f: FindingRow): { words: string; tone: string } {
  if (f.open) return { words: "Open", tone: "bad" };
  if (f.settlement?.kind === "taken") return { words: `Deferred, taken as ${f.settlement.item}`, tone: "" };
  if (f.settlement?.kind === "dropped") return { words: `Deferred, then dropped: ${f.settlement.reason}`, tone: "" };
  if (f.decided === "defer" || f.deferred) return { words: "Deferred", tone: "review" };
  if (f.decided === "no_action") return { words: "No action, by decision", tone: "" };
  if (f.now === "withdrawn") return { words: "Withdrawn", tone: "" };
  if (f.now === "fixed" || f.recommendation === "fix_now") return { words: "Fixed", tone: "ok" };
  if (f.now === "kept") return { words: "Kept", tone: "" };
  return { words: recommendationWords[f.recommendation], tone: "" };
}

export function Findings({ findings }: { findings: FindingRow[] }) {
  return (
    <ul className="findings">
      {findings.map((f) => {
        const state = findingState(f);
        return (
          <li key={f.name}>
            <details>
              <summary>
                <span className={`glyph ${f.recommendation === "fix_now" ? "bad" : "review"}`}>
                  <MessageSquareWarning size={14} aria-hidden />
                </span>
                <span className="grow">
                  <span className="f-title">{f.title}</span>
                  <span className="sub">
                    <span className="mono">{f.name}</span> · {recommendationWords[f.recommendation]}
                    {f.was && f.was !== f.recommendation ? ` (was ${recommendationWords[f.was] ?? f.was})` : ""}
                  </span>
                </span>
                <span className={`tag ${state.tone}`}>{state.words}</span>
              </summary>
              {f.text ? <Markdown text={f.text} small /> : <p className="muted">The finding has no text beyond its title.</p>}
            </details>
          </li>
        );
      })}
    </ul>
  );
}

function ChangedSince({ paths }: { paths: string[] | null }) {
  if (paths === null) return <p className="sub">Whether the plan changed since cannot be told: this clone lacks the commit reviewed.</p>;
  if (paths.length === 0) return <p className="sub">Nothing in the plan changed since.</p>;
  return (
    <div className="sub changed">
      Changed since:{" "}
      {paths.map((p, at) => (
        <span key={p}>
          {at > 0 ? ", " : ""}
          <span className="mono">{p}</span>
        </span>
      ))}
    </div>
  );
}

/** Review rounds, oldest first, each with its verdict and findings; a plan review also says what changed since. */
export function ReviewList({ reviews, plan }: { reviews: Review[]; plan?: boolean }) {
  return (
    <ol className="timeline">
      {reviews.map((r) => (
        <li key={r.name}>
          <span className={`glyph ${r.verdict === "approve" ? "ok" : r.verdict === "changes" ? "bad" : "dim"}`}>
            {r.verdict === "approve" ? <CircleCheck size={15} /> : r.verdict === "changes" ? <RotateCcw size={15} /> : <MessageSquareWarning size={15} />}
          </span>
          <div className="grow">
            <div className="head">
              <b>
                Round {r.round}: {r.verdict === "approve" ? "approved" : r.verdict === "changes" ? "changes asked" : "no verdict yet"}
              </b>
              <span className="ink2">
                {r.reviewers.length > 0 ? r.reviewers.map((x) => who(x)).join(", ") : "no reviewer named"}
                {r.model ? ` on ${r.model}` : ""}
              </span>
              <time dateTime={r.created_at}>{dateTime(r.created_at)}</time>
              <span className="mono muted">at {shortCommit(r.commit)}</span>
            </div>
            {plan ? <ChangedSince paths={r.changed_since} /> : null}
            {r.findings.length > 0 ? (
              <>
                <p className="sub">{plural(r.findings.length, "finding")}</p>
                <Findings findings={r.findings} />
              </>
            ) : (
              <p className="sub">No findings.</p>
            )}
          </div>
        </li>
      ))}
    </ol>
  );
}
