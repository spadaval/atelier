import type { ReactNode } from "react";
import { useParams } from "react-router";
import { useApi } from "../api";
import type { Brief } from "../api/Brief";
import type { BriefResponse } from "../api/BriefResponse";
import type { MissionEvent } from "../api/MissionEvent";
import { Markdown } from "../components/Markdown";
import { Facts, ItemLine, Loading, Page, Section } from "../components/Page";
import { localDate, shortCommit } from "../text";
import { MissionNav, missionCrumbs } from "./Mission";

/** `/<repo>/mission/<id>/brief`: what `atelier brief` prints. */
export function BriefPage() {
  const { repo = "", id = "" } = useParams();
  const loaded = useApi<BriefResponse>(`/api/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}/brief`);
  return (
    <Loading loaded={loaded} crumbs={missionCrumbs(repo, id)}>
      {({ brief }) => (
        <Page crumbs={missionCrumbs(repo, id, "Brief")} title={`Brief: ${brief.mission.title}`}>
          <MissionNav repo={repo} id={brief.mission.id} strategy={brief.mission.strategy !== null} current="brief" />
          <BriefView brief={brief} />
        </Page>
      )}
    </Loading>
  );
}

/** One review event on the mission, as the brief lists it. */
export function eventLine(event: MissionEvent): string {
  return `${event.event} ${event.id} ${localDate(event.created_at)} by ${event.identity} at ${shortCommit(event.commit)} | ${event.summary}`;
}

/** The sections of `atelier brief`, in its order. */
export function BriefView({ brief }: { brief: Brief }) {
  const mission = brief.mission;
  const plan: [string, string | null][] = [
    ["Sizing", brief.sizing],
    ["Derisking", brief.derisking],
  ];
  const facts: [string, ReactNode][] = [
    ["Mission", <ItemLine id={mission.id} status={mission.status} cues={[mission.priority]} />],
  ];
  for (const [title, text] of plan) if (text === null) facts.push([title, "not written"]);
  if (brief.strategy) facts.push(["Strategy", <code>{brief.strategy}</code>]);
  if (brief.generated_from) facts.push(["Generated from", <code>{shortCommit(brief.generated_from)}</code>]);
  const blocked = brief.items.filter((item) => item.open_blockers.length > 0);
  const unknowns = brief.items.filter((item) => item.unknowns !== null);
  const approvals = brief.mission_events.filter((event) => event.event === "approval").length;
  return (
    <>
      <Facts facts={facts} />
      <Section title="Outcome">
        <Markdown text={brief.outcome} />
      </Section>
      {plan.map(([title, text]) =>
        text === null ? null : (
          <Section title={title} key={title}>
            <Markdown text={text} />
          </Section>
        ),
      )}
      <Section
        title="Success Criteria"
        show={brief.criteria.parse.criteria.length + brief.criteria.parse.problems.length > 0}
      >
        <ul className="rows">
          {brief.criteria.parse.criteria.map((criterion) => (
            <li key={criterion.slug}>
              <code>{criterion.slug}</code> {criterion.text}
            </li>
          ))}
          {brief.criteria.parse.problems.map((problem, at) => (
            <li key={`p${at}`} className="warn">
              ⚠ {problem}
            </li>
          ))}
        </ul>
      </Section>
      <Section title="Scope">
        {brief.items.length === 0 ? (
          <p className="muted">No items in scope.</p>
        ) : (
          <ul className="rows">
            {brief.items.map((item) => {
              const cues: string[] = [];
              if (item.size) cues.push(`size ${item.size}`);
              if (item.open_blockers.length > 0) cues.push(`blockers: ${item.open_blockers.join(", ")}`);
              if (item.needs_human) cues.push("needs a human");
              return (
                <li key={item.issue.id} className={item.parent ? "child" : undefined}>
                  <ItemLine
                    id={item.issue.id}
                    issueType={item.issue.issue_type}
                    status={item.issue.status}
                    title={item.issue.title}
                    cues={cues}
                  />
                </li>
              );
            })}
          </ul>
        )}
      </Section>
      <Section title="Blockers" show={blocked.length > 0}>
        <ul className="rows">
          {blocked.map((item) => (
            <li key={item.issue.id}>
              <code>{item.issue.id}</code> blocked by {item.open_blockers.join(", ")}
            </li>
          ))}
        </ul>
      </Section>
      <Section title="Unknowns" show={unknowns.length > 0}>
        <ul className="rows">
          {unknowns.map((item) => (
            <li key={item.issue.id}>
              <code>{item.issue.id}</code>
              <Markdown text={item.unknowns ?? ""} />
            </li>
          ))}
        </ul>
      </Section>
      <Section title="Open Human Decisions" show={brief.human_decisions.length > 0}>
        <ul className="rows">
          {brief.human_decisions.map((issue) => (
            <li key={issue.id}>
              <ItemLine id={issue.id} issueType={issue.issue_type} status={issue.status} title={issue.title} />
            </li>
          ))}
        </ul>
      </Section>
      <Section title="Risks" show={brief.risks !== null}>
        <Markdown text={brief.risks ?? ""} />
      </Section>
      <Section title="Mission Reviews">
        <ul className="rows">
          {brief.mission_events.map((event) => (
            <li key={event.id}>{eventLine(event)}</li>
          ))}
          {approvals === 0 ? <li className="muted">No plan approval recorded.</li> : null}
        </ul>
      </Section>
    </>
  );
}
