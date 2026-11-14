import { Link } from "react-router";
import type { Agent } from "../api/Agent";
import type { MotionRepo } from "../api/MotionRepo";
import type { MotionResponse } from "../api/MotionResponse";
import type { T3State } from "../api/T3State";
import { AgentRow, T3Notice } from "../components/Agents";
import { Crumbs } from "../components/Crumbs";
import { Empty, ErrorBanner, FetchedNote, Skeleton, StatusGlyph } from "../components/ui";
import { api, to, useData } from "../lib/api";
import { statusTitle, t3Note } from "../lib/words";

/** Whether T3 Code says `agent`'s thread is alive: `null` when T3 Code has no list to judge by. */
export function alive(agent: Agent, t3: T3State): boolean | null {
  if (agent.thread) return true;
  const note = t3Note(t3);
  return note === null || note.known ? false : null;
}

function Who({ repo, agent, t3 }: { repo: string; agent: Agent | null; t3: T3State }) {
  if (!agent) return <p className="sub muted">No thread holds it.</p>;
  const note = t3Note(t3);
  return <AgentRow repo={repo} agent={{ ...agent, item: null }} known={note === null || note.known} />;
}

function RepoMotion({ repo, many, t3 }: { repo: MotionRepo; many: boolean; t3: T3State }) {
  const name = repo.repo;
  return (
    <section className="inbox-repo" aria-label={name}>
      {many ? (
        <h2 className="repo-title">
          <Link to={to.repo(name)}>{name}</Link> <span className="tally">{repo.items.length}</span>
        </h2>
      ) : null}
      {repo.error ? (
        <div className="banner" role="alert">
          <span>
            {name} cannot be read: {repo.error}
          </span>
        </div>
      ) : repo.items.length === 0 && repo.missions.length === 0 ? (
        <div className="list">
          <Empty title="Nothing is in motion here." />
        </div>
      ) : null}
      {repo.missions.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Missions in progress</h3>
            <span className="tally">{repo.missions.length}</span>
          </div>
          <div className="list">
            {repo.missions.map((m) => (
              <div key={m.mission.id} className="motion" data-testid="motion-mission">
                <Link to={to.mission(name, m.mission.id)} className="q">
                  {m.mission.title}
                </Link>
                <Who repo={name} agent={m.lead} t3={t3} />
              </div>
            ))}
          </div>
        </div>
      ) : null}
      {repo.items.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Started items</h3>
            <span className="tally">{repo.items.length}</span>
          </div>
          <div className="list">
            {repo.items.map((i) => (
              <div key={i.id} className="motion" data-testid="motion-item">
                <Link to={to.item(name, i.id)} className="item">
                  <StatusGlyph status={i.status} />
                  <span className="id">{i.id}</span>
                  <span className="title">{i.title}</span>
                  <span className="meta">
                    <span className="tag">{statusTitle(i.status)}</span>
                    {i.branch ? <span className="tag opt mono">{i.branch}</span> : null}
                  </span>
                </Link>
                <Who repo={name} agent={i.holder} t3={t3} />
                {i.reviewer ? <Who repo={name} agent={i.reviewer} t3={t3} /> : null}
              </div>
            ))}
          </div>
        </div>
      ) : null}
      <FetchedNote fetched={repo.fetched} />
    </section>
  );
}

/** `/motion`: each started item and the thread on it, and each mission in progress with its lead, in every repository. */
export function MotionPage() {
  const loaded = useData<MotionResponse>(api.motion());
  const repos = loaded.data?.repos ?? [];
  return (
    <div className="page narrow">
      <Crumbs trail={[{ label: "In motion" }]} />
      <div className="page-head">
        <div>
          <h1>In motion</h1>
          <p className="lede">Each started item, read from its branch, and the thread that holds it, with whether T3 Code lists that thread as running. Each mission in progress, with its lead.</p>
        </div>
      </div>
      <ErrorBanner loaded={loaded} />
      {loaded.data ? <T3Notice t3={loaded.data.t3} /> : null}
      {!loaded.data && loaded.loading ? <Skeleton rows={8} /> : null}
      {loaded.data ? repos.map((r) => <RepoMotion key={r.repo} repo={r} many={repos.length > 1} t3={loaded.data!.t3} />) : null}
    </div>
  );
}
