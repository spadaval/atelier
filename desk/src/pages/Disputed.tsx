import { Link } from "react-router";
import type { DisputedRepo } from "../api/DisputedRepo";
import type { DisputedResponse } from "../api/DisputedResponse";
import type { Side } from "../api/Side";
import { Identity, ThreadTitle } from "../components/Agents";
import { Crumbs } from "../components/Crumbs";
import { Empty, ErrorBanner, FetchedNote, Skeleton } from "../components/ui";
import { api, to, useData } from "../lib/api";

/** One side of a disagreement: its thread and the model behind it, where known. */
function SideMark({ side, role }: { side: Side | null; role: string }) {
  if (!side) return <span className="muted">{role} unknown</span>;
  return (
    <span className="side" data-testid="side">
      {role} {side.link ? <ThreadTitle thread={{ title: side.identity, link: side.link }} /> : <Identity id={side.identity} />}
      {side.model ? <span className="mono"> ({side.model})</span> : <span className="muted"> (model unknown)</span>}
    </span>
  );
}

function RepoDisputed({ repo, many }: { repo: DisputedRepo; many: boolean }) {
  const name = repo.repo;
  const none = repo.rebuttals.length === 0 && repo.plans.length === 0;
  return (
    <section className="inbox-repo" aria-label={name}>
      {many ? (
        <h2 className="repo-title">
          <Link to={to.repo(name)}>{name}</Link> <span className="tally">{repo.rebuttals.length + repo.plans.length}</span>
        </h2>
      ) : null}
      {repo.error ? (
        <div className="banner" role="alert">
          <span>
            {name} cannot be read: {repo.error}
          </span>
        </div>
      ) : none ? (
        <div className="list">
          <Empty title="Nothing is disputed here." />
        </div>
      ) : null}
      {repo.rebuttals.map((r) => (
        <div key={`${r.item}/${r.finding}`} className="qline" data-testid="rebuttal">
          <span className="grow">
            <Link to={to.item(name, r.item)} className="q">
              <span className="mono">{r.item}</span>/{r.finding}: {r.title}
            </Link>
            <span className="sub">
              Rebutted: {r.fact}
            </span>
            <span className="sub">
              <SideMark side={r.raised_by} role="Raised by" /> · <SideMark side={r.implementer} role="rebutted by" />
            </span>
            <span className="sub">
              {r.judged.length === 0
                ? "Not yet reviewed."
                : r.judged.map((j, at) => (
                    <span key={j.finding}>
                      {at > 0 ? "; " : null}
                      {j.judged[0].toUpperCase() + j.judged.slice(1)} in {j.finding}, <SideMark side={j.reviewer} role="by" />
                    </span>
                  ))}
            </span>
          </span>
        </div>
      ))}
      {repo.plans.map((p) => (
        <div key={`${p.mission}/${p.round}`} className="qline" data-testid="split-round">
          <span className="grow">
            <Link to={to.mission(name, p.mission)} className="q">
              Plan of <span className="mono">{p.mission}</span>, round {p.round}
            </Link>
            <span className="sub">
              {p.reviews.map((r, at) => (
                <span key={r.name}>
                  {at > 0 ? "; " : null}
                  {r.verdict === "approve" ? "Approved" : "Changes asked"}, <SideMark side={r.reviewer} role="by" />
                </span>
              ))}
            </span>
          </span>
        </div>
      ))}
      {repo.error ? null : <FetchedNote fetched={repo.fetched} />}
    </section>
  );
}

/** `/disputed`: what `atelier list --disputed` gives, with each side's model where it is known. */
export function DisputedPage() {
  const loaded = useData<DisputedResponse>(api.disputed());
  const repos = loaded.data?.repos ?? [];
  return (
    <div className="page narrow">
      <Crumbs trail={[{ label: "Disputed" }]} />
      <div className="page-head">
        <div>
          <h1>Disputed</h1>
          <p className="lede">
            Each finding an implementer rebutted, with what the next round's reviewer made of it, and each plan-review round whose reviewers disagreed. Nothing here waits on you; it shows where two models
            saw a change differently.
          </p>
        </div>
      </div>
      <ErrorBanner loaded={loaded} />
      {!loaded.data && loaded.loading ? <Skeleton rows={8} /> : null}
      {repos.map((r) => (
        <RepoDisputed key={r.repo} repo={r} many={repos.length > 1} />
      ))}
    </div>
  );
}
