import type { ReactNode } from "react";
import { Link, useParams } from "react-router";
import { useApi } from "../api";
import type { BoardMission } from "../api/BoardMission";
import type { BoardResponse } from "../api/BoardResponse";
import type { BoardRow } from "../api/BoardRow";
import type { InProgress } from "../api/InProgress";
import type { ReviewState } from "../api/ReviewState";
import { Facts, ItemLine, Loading, Page, Section } from "../components/Page";
import { countNoun, localDate, shortCommit } from "../text";
import { InboxLink } from "./Inbox";

/** The crumbs of a repository's pages: the board, then `page`. */
export function repoCrumbs(repo: string, page?: string) {
  return page ? [{ label: repo, to: `/${repo}` }, { label: page }] : [{ label: repo }];
}

/** Review state in `atelier status`'s words, with `unreviewed` for no review at all. */
export const reviewWords: Record<ReviewState, string> = {
  approved: "✓ approved",
  approval_stale: "⚠ approval stale",
  not_approved: "⚠ not approved",
  unreviewed: "⚠ unreviewed",
};

/** One item, linked to its page. */
function Row({ repo, row, cues = [] }: { repo: string; row: BoardRow; cues?: ReactNode[] }) {
  const blockers = row.open_blockers.length > 0 ? [`blockers: ${row.open_blockers.join(", ")}`] : [];
  return (
    <li>
      <Link to={`/${repo}/item/${row.id}`}>
        <ItemLine
          id={row.id}
          issueType={row.issue_type}
          status={row.status}
          priority={row.priority}
          title={row.title}
          cues={[...cues, ...blockers]}
        />
      </Link>
    </li>
  );
}

function Rows({ repo, rows, empty }: { repo: string; rows: BoardRow[]; empty: string }) {
  if (rows.length === 0) return <p className="muted">{empty}</p>;
  return (
    <ul className="rows">
      {rows.map((row) => (
        <Row key={row.id} repo={repo} row={row} />
      ))}
    </ul>
  );
}

/** `3 of 8 items done, 1 of 2 criteria proven`. */
export function progress(mission: BoardMission): string {
  const items = `${mission.done} of ${countNoun(mission.planned, "item")} done`;
  if (mission.criteria_error !== null) return `${items}, criteria unreadable`;
  // The debrief's words: `1 of 1 criteria proven`.
  return `${items}, ${mission.proven} of ${mission.criteria} criteria proven`;
}

function inProgressCues(work: InProgress): ReactNode[] {
  const cues: ReactNode[] = [];
  if (work.branch !== null) cues.push(<code>{work.branch}</code>);
  const state = work.on_branch;
  if (state === null) return cues;
  cues.push(reviewWords[state.review]);
  if (state.open_findings.length > 0) cues.push(`⚠ ${countNoun(state.open_findings.length, "finding")} open`);
  cues.push(`behind ${state.behind}`);
  return cues;
}

/** `/<repo>`: where the repository's work stands, what is done and what is next. */
export function BoardPage() {
  const { repo = "" } = useParams();
  const loaded = useApi<BoardResponse>(`/api/${encodeURIComponent(repo)}/board`);
  return (
    <Loading loaded={loaded} crumbs={repoCrumbs(repo)}>
      {(body) => <BoardView body={body} />}
    </Loading>
  );
}

function BoardView({ body }: { body: BoardResponse }) {
  const { repo, board } = body;
  return (
    <Page crumbs={repoCrumbs(repo)} title={`Board: ${repo}`}>
      <InboxLink />
      <Facts
        facts={[
          ["Base", <code>{`${body.base_branch} at ${shortCommit(body.base_commit)}`}</code>],
          ["Today", board.today],
        ]}
      />
      <Section title="Missions">
        {board.missions.length === 0 ? (
          <p className="muted">No open missions.</p>
        ) : (
          <ul className="rows">
            {board.missions.map((mission) => (
              <li key={mission.id}>
                <Link to={`/${repo}/mission/${mission.id}`}>
                  <ItemLine
                    id={mission.id}
                    status={mission.status}
                    priority={mission.priority}
                    title={mission.title}
                    cues={[progress(mission)]}
                  />
                </Link>
              </li>
            ))}
          </ul>
        )}
      </Section>
      <Section title="In Progress">
        {body.in_progress.length === 0 ? (
          <p className="muted">No work in progress.</p>
        ) : (
          <ul className="rows">
            {body.in_progress.map((work) => (
              <Row key={`${work.item.id} ${work.branch ?? ""}`} repo={repo} row={work.item} cues={inProgressCues(work)} />
            ))}
          </ul>
        )}
        {body.unreadable_branches.map((branch) => (
          <p key={branch.branch} className="warn">
            ⚠ <code>{branch.branch}</code> cannot be read: {branch.error}
          </p>
        ))}
      </Section>
      <Section title="Ready Work">
        <Rows repo={repo} rows={board.ready} empty="No ready work." />
      </Section>
      <Section title="Needs a Human">
        <Rows repo={repo} rows={board.human} empty="Nothing waits on a human." />
      </Section>
      <Section title="Open Work">
        {board.groups.length === 0 ? <p className="muted">No open work.</p> : null}
        {board.groups.map((group) => (
          <div key={`${group.mission ?? ""}:${group.label ?? ""}`} className="group">
            <h3>
              {group.mission !== null ? (
                <Link to={`/${repo}/mission/${group.mission}`}>{group.title}</Link>
              ) : group.label !== null ? (
                <>
                  Label <code>{group.label}</code>
                </>
              ) : (
                "No label"
              )}
            </h3>
            {group.work.length === 0 ? (
              <p className="muted">No open work.</p>
            ) : (
              <ul className="rows">
                {group.work.map((row) => (
                  <Row key={row.id} repo={repo} row={row} cues={[row.state]} />
                ))}
              </ul>
            )}
          </div>
        ))}
      </Section>
      <Section title={`Done in the Last ${board.recent_days} Days`}>
        {board.recently_done.length === 0 ? (
          <p className="muted">Nothing done.</p>
        ) : (
          <ul className="rows">
            {board.recently_done.map((row) => (
              <Row key={row.id} repo={repo} row={row} cues={row.closed_at ? [localDate(row.closed_at)] : []} />
            ))}
          </ul>
        )}
      </Section>
    </Page>
  );
}
