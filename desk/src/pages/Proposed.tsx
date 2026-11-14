import { GitBranch, Target } from "lucide-react";
import { Link } from "react-router";
import type { PrMark } from "../api/PrMark";
import type { ProposedRepo } from "../api/ProposedRepo";
import type { ProposedResponse } from "../api/ProposedResponse";
import type { RoundMark } from "../api/RoundMark";
import { Crumbs } from "../components/Crumbs";
import { Empty, ErrorBanner, FetchedNote, Skeleton } from "../components/ui";
import { api, to, useData } from "../lib/api";
import { plural } from "../lib/words";

export function PullRequestLink({ pr }: { pr: PrMark | null }) {
  if (!pr) return <span className="muted">no pull request</span>;
  return (
    <a href={pr.url} target="_blank" rel="noreferrer" className="mono" data-testid="pull-request">
      #{pr.number} {pr.state.toLowerCase()}
    </a>
  );
}

/** A plan review round's verdicts in a few words. */
export function roundWords(round: RoundMark | null): string {
  if (!round) return "not reviewed yet";
  const verdicts = round.reviews.map((r) => (r.verdict === "approve" ? "approved" : r.verdict === "changes" ? "changes asked" : "not finished"));
  return `round ${round.round}: ${verdicts.join(", ")}`;
}

const changeWords = { added: "added", changed: "changed", dropped: "dropped", removed: "removed" } as const;

function RepoProposed({ repo, many }: { repo: ProposedRepo; many: boolean }) {
  const name = repo.repo;
  const none = repo.plans.length === 0 && repo.changes.length === 0;
  return (
    <section className="inbox-repo" aria-label={name}>
      {many ? (
        <h2 className="repo-title">
          <Link to={to.repo(name)}>{name}</Link> <span className="tally">{repo.plans.length + repo.changes.length}</span>
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
          <Empty title="Nothing is proposed here." />
        </div>
      ) : null}
      {repo.plans.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Plans</h3>
            <span className="tally">{repo.plans.length}</span>
          </div>
          <div className="list">
            {repo.plans.map((p) => (
              <div key={p.mission.id} className="qline" data-testid="plan">
                <span className="glyph you">
                  <Target size={15} aria-hidden />
                </span>
                <span className="grow">
                  <Link to={to.mission(name, p.mission.id)} className="q">
                    {p.mission.title}
                  </Link>
                  <span className="sub">
                    <span className="mono">{p.mission.id}</span> · <span className="mono">{p.mission.branch ?? "on the base"}</span> · <PullRequestLink pr={p.pull_request} /> · {roundWords(p.round)}
                  </span>
                </span>
              </div>
            ))}
          </div>
        </div>
      ) : null}
      {repo.changes.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Record changes on branches</h3>
            <span className="tally">{repo.changes.length}</span>
          </div>
          <div className="list">
            {repo.changes.map((c) => (
              <div key={c.branch} className="qline" data-testid="change">
                <span className="glyph dim">
                  <GitBranch size={15} aria-hidden />
                </span>
                <span className="grow">
                  <span className="q mono">{c.branch}</span>
                  <span className="sub">
                    <PullRequestLink pr={c.pull_request} /> · {plural(c.records.length, "record")}
                  </span>
                  <ul className="changes">
                    {c.records.map((r) => (
                      <li key={`${r.kind}${r.id}`}>
                        <Link to={r.kind === "item" ? to.item(name, r.id) : to.mission(name, r.id)} className="mono">
                          {r.id}
                        </Link>{" "}
                        {changeWords[r.change]}: {r.title}
                      </li>
                    ))}
                  </ul>
                </span>
              </div>
            ))}
          </div>
        </div>
      ) : null}
      <FetchedNote fetched={repo.fetched} />
    </section>
  );
}

/** `/proposed`: plans and record changes on branches not yet merged, in every repository. */
export function ProposedPage() {
  const loaded = useData<ProposedResponse>(api.proposed());
  const repos = loaded.data?.repos ?? [];
  return (
    <div className="page narrow">
      <Crumbs trail={[{ label: "Proposed" }]} />
      <div className="page-head">
        <div>
          <h1>Proposed</h1>
          <p className="lede">
            What waits to merge: each mission's plan on its branch, with its pull request and its last round of plan review, and record changes on other branches. A plan merges once two reviewers approve it; stop one from its page.
          </p>
        </div>
      </div>
      <ErrorBanner loaded={loaded} />
      {!loaded.data && loaded.loading ? <Skeleton rows={8} /> : null}
      {repos.map((r) => (
        <RepoProposed key={r.repo} repo={r} many={repos.length > 1} />
      ))}
    </div>
  );
}
