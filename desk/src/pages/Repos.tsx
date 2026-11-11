import { FolderGit2 } from "lucide-react";
import { Link } from "react-router";
import type { MissionsResponse } from "../api/MissionsResponse";
import type { RepoSummary } from "../api/RepoSummary";
import type { ReposResponse } from "../api/ReposResponse";
import { Crumbs } from "../components/Crumbs";
import { Empty, ErrorBanner, ScopeBar, Skeleton, StatusGlyph } from "../components/ui";
import { api, to, useData } from "../lib/api";
import { countsLine, finished, missionState, plural, shortCommit, statusTitle, total } from "../lib/words";

function RepoMissions({ name }: { name: string }) {
  const loaded = useData<MissionsResponse>(api.missions(name), 60_000);
  if (!loaded.data) return loaded.error ? <ErrorBanner loaded={loaded} /> : <Skeleton rows={3} />;
  const missions = [...loaded.data.missions].sort((a, b) => Number(b.in_progress) - Number(a.in_progress) || Number(a.status === "draft") - Number(b.status === "draft"));
  if (missions.length === 0) return <Empty title="No open missions." />;
  return (
    <>
      {missions.map((m) => (
        <Link key={m.id} to={to.mission(name, m.id)} className="mission-row">
          <span className="t">
            <StatusGlyph status={missionState(m)} label={m.in_progress ? "Work in progress" : statusTitle(m.status)} />
            <span className="truncate">{m.title}</span>
            {m.status === "draft" ? <span className="tag you">draft</span> : null}
            {m.waiting.length > 0 ? <span className="tag you">{m.waiting.length} for you</span> : null}
          </span>
          <span title={countsLine(m.counts)}>
            <ScopeBar counts={m.counts} />
          </span>
          <span className="n">
            {finished(m.counts)}/{total(m.counts)}
          </span>
        </Link>
      ))}
    </>
  );
}

function Repo({ repo }: { repo: RepoSummary }) {
  return (
    <section className="sect repo" aria-label={repo.name}>
      <div className="sect-head">
        <FolderGit2 size={15} aria-hidden className="muted" />
        <h2>
          <Link to={to.repo(repo.name)}>{repo.name}</Link>
        </h2>
        {repo.base ? (
          <span className="tally">
            {repo.base} at {shortCommit(repo.base_tip)}
          </span>
        ) : null}
      </div>
      {repo.error ? (
        <div className="banner" role="alert">
          <span>{repo.error}</span>
        </div>
      ) : (
        <>
          <p className="repo-counts" data-testid="repo-counts">
            <span>
              <b className="num">{repo.in_progress}</b> in progress
            </span>
            <span className={repo.drafts > 0 ? "you" : ""}>
              <b className="num">{repo.drafts}</b> {repo.drafts === 1 ? "draft" : "drafts"} waiting for your go
            </span>
            <span className={repo.waiting > 0 ? "you" : ""}>
              <b className="num">{repo.waiting}</b> {repo.waiting === 1 ? "item needs" : "items need"} you
            </span>
          </p>
          <div className="list">
            <RepoMissions name={repo.name} />
          </div>
        </>
      )}
    </section>
  );
}

/** `/`: every repository, each as its missions, with what waits on the human. */
export function ReposPage() {
  const loaded = useData<ReposResponse>(api.repos());
  const repos = loaded.data?.repos ?? [];
  const going = repos.reduce((n, r) => n + r.in_progress, 0);
  const drafts = repos.reduce((n, r) => n + r.drafts, 0);
  const waiting = repos.reduce((n, r) => n + r.waiting, 0);
  return (
    <div className="page narrow">
      <Crumbs trail={[{ label: "Repositories" }]} />
      <div className="page-head">
        <div>
          <h1>{loaded.data ? `${plural(going, "mission", "missions")} in progress.` : "Reading every repository…"}</h1>
          {loaded.data ? (
            <p className="lede">
              {drafts + waiting === 0 ? (
                "Nothing waits on you."
              ) : (
                <Link to="/inbox" className="you-link">
                  {plural(drafts, "draft waits", "drafts wait")} for your go and {plural(waiting, "item needs", "items need")} you.
                </Link>
              )}{" "}
              Across {plural(repos.length, "repository", "repositories")}.
            </p>
          ) : null}
        </div>
      </div>
      <ErrorBanner loaded={loaded} />
      {!loaded.data && loaded.loading ? <Skeleton rows={6} /> : null}
      {loaded.data && repos.length === 0 ? <Empty title="No repository is served.">The Desk's configuration names the folders whose checkouts it shows.</Empty> : null}
      {repos.map((r) => (
        <Repo key={r.name} repo={r} />
      ))}
    </div>
  );
}
