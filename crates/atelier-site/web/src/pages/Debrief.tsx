import type { ReactNode } from "react";
import { useParams } from "react-router";
import { useApi } from "../api";
import type { Debrief } from "../api/Debrief";
import type { DebriefResponse } from "../api/DebriefResponse";
import type { OpenEntry } from "../api/OpenEntry";
import type { TimedEvent } from "../api/TimedEvent";
import { Markdown } from "../components/Markdown";
import { Facts, ItemLine, Loading, Page, Section } from "../components/Page";
import {
  askLine,
  awaitedBy,
  changeText,
  changedCourse,
  commitsSince,
  criterionWord,
  decisionsOn,
  deliveredFacts,
  findingCounts,
  itemText,
  judged,
  lastDelivery,
  origin,
  proofCue,
  reportFacts,
  statusWord,
  verdict,
} from "../debrief";
import { countNoun, flat, localTime, shortCommit } from "../text";
import { MissionNav, missionCrumbs } from "./Mission";

/** `/<repo>/mission/<id>/debrief`: what `atelier debrief` prints. */
export function DebriefPage() {
  const { repo = "", id = "" } = useParams();
  const loaded = useApi<DebriefResponse>(`/api/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}/debrief`);
  return (
    <Loading loaded={loaded} crumbs={missionCrumbs(repo, id)}>
      {({ debrief }) => (
        <Page crumbs={missionCrumbs(repo, id, "Debrief")} title={`Debrief: ${debrief.mission.title}`}>
          <MissionNav repo={repo} id={debrief.mission.id} strategy={debrief.mission.strategy !== null} current="debrief" />
          <DebriefView debrief={debrief} />
        </Page>
      )}
    </Loading>
  );
}

/** An item by id, as the debrief names it. */
function Item({ view, id, cues = [] }: { view: Debrief; id: string; cues?: ReactNode[] }) {
  const issue = view.refs[id];
  if (!issue) return <ItemLine id={id} cues={cues} />;
  return <ItemLine id={id} issueType={issue.issue_type} status={issue.status} title={issue.title} cues={cues} />;
}

/** Labelled lines under an item. */
function Sub({ facts }: { facts: [string, string][] }) {
  if (facts.length === 0) return null;
  return (
    <ul className="sub">
      {facts.map(([label, text], at) => (
        <li key={at}>
          <span className="label">{label}:</span> {flat(text)}
        </li>
      ))}
    </ul>
  );
}

/** The sections of `atelier debrief`, in its order. */
export function DebriefView({ debrief: view }: { debrief: Debrief }) {
  const closed = view.timeline.closed !== null;
  const facts: [string, ReactNode][] = [
    ["Mission", <ItemLine id={view.mission.id} status={view.mission.status} cues={[view.mission.priority]} />],
    ["Status", [statusWord(view).toLowerCase(), ...verdict(view)].join(" | ")],
  ];
  if (view.timeline.started) facts.push(["Started", localTime(view.timeline.started.time)]);
  if (view.timeline.closed) facts.push(["Closed", localTime(view.timeline.closed.time)]);
  else {
    const last = lastDelivery(view);
    if (last) facts.push(["Last delivery", localTime(last)]);
  }
  if (view.strategy) facts.push(["Strategy", <code>{view.strategy}</code>]);
  if (view.generated_from) facts.push(["Generated from", <code>{shortCommit(view.generated_from)}</code>]);

  const checks = view.validation;
  const delivered = checks.approved + checks.trivial + checks.not_approved.length + checks.unreviewed.length;
  const changed = view.work.filter(changedCourse).map((entry) => entry.id);
  const hasCriteria = view.criteria.proofs.length + view.criteria.criteria.parse.problems.length > 0;
  const missionDecisions = decisionsOn(view, view.mission.id);
  const hasValidation = delivered > 0 || checks.runs.length > 0 || checks.awaiting.length > 0;
  const hasPlan =
    view.planned_total > 0 || changed.length > 0 || view.added.length > 0 || view.dropped.length > 0 || view.found.length > 0;

  return (
    <>
      <Facts facts={facts} />
      <Section title="The Problem" show={!!view.why}>
        <Markdown text={view.why ?? ""} />
      </Section>
      <Section title="What We Set Out To Do" show={!!view.outcome || hasCriteria || missionDecisions.length > 0}>
        {view.outcome ? <Markdown text={view.outcome} /> : null}
        {hasCriteria ? (
          <ul className="rows">
            {view.criteria.proofs.map((proof) => (
              <li key={proof.criterion.slug}>
                <code>{proof.criterion.slug}</code> {flat(proof.criterion.text)}
              </li>
            ))}
            {view.criteria.criteria.parse.problems.map((problem, at) => (
              <li key={`p${at}`} className="warn">
                ⚠ {problem}
              </li>
            ))}
          </ul>
        ) : null}
        <Sub facts={missionDecisions} />
      </Section>
      <Section title="Planned Work" show={view.planned.length > 0}>
        <ul className="rows">
          {view.planned.map(([id, parent]) => (
            <li key={id} className={parent ? "child" : undefined}>
              <Item view={view} id={id} />
            </li>
          ))}
        </ul>
      </Section>
      <Section title="What We Did" show={view.events.length > 0}>
        <ol className="rows timeline">
          {view.events.map((timed, at) => (
            <EventRow key={at} view={view} timed={timed} />
          ))}
        </ol>
      </Section>
      <Section title="Results" show={view.criteria.proofs.length > 0}>
        <ul className="rows">
          {view.criteria.proofs.map((proof) => {
            const slug = proof.criterion.slug;
            const cues: string[] = [];
            const by = view.delivered_by[slug] ?? [];
            if (by.length > 0) cues.push(`by ${by.join(", ")}`);
            const cue = proofCue(proof, closed);
            const awaits = awaitedBy(view, slug);
            if (cue) cues.push(cue);
            else if (awaits) cues.push(`awaits ${awaits}`);
            return (
              <li key={slug} className={`mark-${proof.mark}`}>
                <code>{slug}</code> <strong>{criterionWord(proof.mark)}</strong>
                {cues.map((cue) => (
                  <span className="cue" key={cue}>
                    {cue}
                  </span>
                ))}
              </li>
            );
          })}
        </ul>
      </Section>
      <Section title="Validation" show={hasValidation}>
        <ul className="rows">
          {delivered > 0 ? (
            <li>
              Item review: {checks.approved} of {delivered} delivered items approved
              {checks.trivial > 0 ? `, ${checks.trivial} waived as trivial` : ""}
              {findingCounts(checks.findings) ? ` | ${findingCounts(checks.findings)}` : ""}
            </li>
          ) : null}
          {delivered > 0 && checks.not_approved.length > 0 ? <li>Not approved: {checks.not_approved.join(", ")}</li> : null}
          {delivered > 0 && checks.unreviewed.length > 0 ? <li>No review recorded: {checks.unreviewed.join(", ")}</li> : null}
          {checks.runs.map((run, at) => (
            <li key={`run${at}`}>
              {localTime(run.at)} {run.identity} at <code>{shortCommit(run.commit)}</code> |{" "}
              {countNoun(run.receipts.length, "receipt")}
              {run.code_commits_since ? ` | ${commitsSince(run.code_commits_since, closed)}` : ""}
              <ul className="sub">
                {run.receipts.map((receipt) => (
                  <li key={receipt.id}>
                    {receipt.criteria.map((criterion) => `${criterion.slug} ${criterion.result}`).join(", ")}{" "}
                    <code>{receipt.id}</code> | {flat(receipt.summary)}
                  </li>
                ))}
                {run.later_work.length > 0 ? <li>Delivered since: {run.later_work.join(", ")}</li> : null}
              </ul>
            </li>
          ))}
          {checks.awaiting.map(([slug, mark, by]) => (
            <li key={`await-${slug}`}>
              Awaiting: <code>{slug}</code> {criterionWord(mark)}
              {by ? ` | ${itemText(view, by)}` : ""}
            </li>
          ))}
        </ul>
      </Section>
      <Section title="Plan vs Actual" show={hasPlan}>
        <ul className="rows">
          {view.planned_total > 0 ? (
            <li>
              Planned: {view.planned_done} of {countNoun(view.planned_total, "item")} done
            </li>
          ) : null}
          {changed.length > 0 ? <li>Changed course: {changed.join(", ")}</li> : null}
          {view.added.map((id) => (
            <li key={`added-${id}`}>
              Added: <Item view={view} id={id} />
            </li>
          ))}
          {view.dropped.map((entry) => (
            <li key={`dropped-${entry.id}`}>
              Dropped:{" "}
              <Item
                view={view}
                id={entry.id}
                cues={[flat(entry.reason ?? ""), ...entry.changes.map(changeText)].filter(Boolean)}
              />
              <Sub facts={decisionsOn(view, entry.id)} />
            </li>
          ))}
          {view.found.length > 0 ? <li>Found: {countNoun(view.found.length, "item")} still open</li> : null}
        </ul>
      </Section>
      <Section title="Needs Your Review" show={view.asks.length > 0}>
        <ul className="rows">
          {view.asks.map((ask, at) => {
            const [title, detail] = askLine(view, ask);
            const id = ask.kind === "decision" || ask.kind === "needs_human" ? ask.value : null;
            return (
              <li key={at} className="warn">
                ⚠ {id ? <code>{id}</code> : null} {title}
                {detail ? <span className="cue">{detail}</span> : null}
                {id ? <Sub facts={decisionsOn(view, id)} /> : null}
              </li>
            );
          })}
        </ul>
      </Section>
      <Section title="Left To Do" show={view.left.length > 0}>
        <OpenRows view={view} entries={view.left} found={false} />
      </Section>
      <Section title="Found Along The Way" show={view.found.length > 0}>
        <OpenRows view={view} entries={view.found} found />
      </Section>
    </>
  );
}

function OpenRows({ view, entries, found }: { view: Debrief; entries: OpenEntry[]; found: boolean }) {
  return (
    <ul className="rows">
      {entries.map((entry) => {
        const cues: string[] = [];
        if (found) {
          const from = origin(entry);
          if (from) cues.push(`${from[1]} ${from[0] === view.mission.id ? "the mission" : from[0]}`);
          if (entry.filed_at) cues.push(`filed ${localTime(entry.filed_at)}`);
        } else if (entry.added_after_start) {
          cues.push("added after the start");
        }
        cues.push(...entry.changes.map(changeText));
        return (
          <li key={entry.id} className={entry.parent ? "child" : undefined}>
            <Item view={view} id={entry.id} cues={cues} />
            <Sub facts={decisionsOn(view, entry.id)} />
          </li>
        );
      })}
    </ul>
  );
}

/** One moment of What We Did. */
function EventRow({ view, timed }: { view: Debrief; timed: TimedEvent }) {
  const when = timed.at ? localTime(timed.at) : "undated";
  const event = timed.event;
  let body: ReactNode;
  switch (event.kind) {
    case "started":
      body = "started the mission";
      break;
    case "closed":
      body = "closed the mission";
      break;
    case "added":
      body = (
        <>
          added <Item view={view} id={event.value} />
        </>
      );
      break;
    case "filed": {
      const entry = view.found.find((entry) => entry.id === event.value);
      const from = entry ? origin(entry) : null;
      const cues = from ? [`${from[1]} ${from[0] === view.mission.id ? "the mission" : from[0]}`] : [];
      body = (
        <>
          filed <Item view={view} id={event.value} cues={cues} />
        </>
      );
      break;
    }
    case "dropped": {
      const reason = view.dropped.find((entry) => entry.id === event.value)?.reason ?? "";
      body = (
        <>
          dropped <Item view={view} id={event.value} cues={reason.trim() ? [flat(reason)] : []} />
        </>
      );
      break;
    }
    case "validated": {
      const run = view.validation.runs[event.value];
      body = run ? (
        <>
          validated by {run.identity} at <code>{shortCommit(run.commit)}</code> | {judged(run)}
        </>
      ) : (
        "validated"
      );
      break;
    }
    case "mission_review": {
      const review = view.mission_events[event.value];
      body = review ? (
        <>
          {review.event} <code>{review.id}</code> by {review.identity} at <code>{shortCommit(review.commit)}</code> |{" "}
          {flat(review.summary)}
        </>
      ) : (
        "mission review"
      );
      break;
    }
    case "base_exemption": {
      const exemption = view.base_exemptions[event.value];
      body = exemption ? (
        <>
          base commit <code>{shortCommit(exemption.commit)}</code> exempted as trivial ({exemption.id}) by{" "}
          {exemption.identity} | {flat(exemption.summary)}
        </>
      ) : (
        "base commit exempted as trivial"
      );
      break;
    }
    case "delivered": {
      const entry = view.work.find((entry) => entry.id === event.value);
      if (!entry) return null;
      const verb = view.refs[entry.id]?.status === "declined" ? "declined" : "delivered";
      const cues: string[] = [];
      if (entry.criteria.length > 0) cues.push(entry.criteria.join(", "));
      if (entry.parent) cues.push(`under ${entry.parent}`);
      if (entry.added_after_start) cues.push("added after the start");
      body = (
        <>
          {verb} <Item view={view} id={entry.id} cues={cues} />
          {entry.report?.outcome ? (
            <div className="outcome">
              <Markdown text={entry.report.outcome} />
            </div>
          ) : null}
          {entry.children.map((child) => (
            <div className="child-report" key={child.id}>
              └ <Item view={view} id={child.id} />
              <Sub facts={reportFacts(child.report, entry.follow_ups.length === 0)} />
            </div>
          ))}
          <Sub facts={deliveredFacts(view, entry)} />
        </>
      );
      break;
    }
  }
  return (
    <li>
      <time>{when}</time> {body}
    </li>
  );
}
