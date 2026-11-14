import { GitMerge, GitPullRequestArrow, Target } from "lucide-react";
import { Link } from "react-router";
import type { InboxRepo } from "../api/InboxRepo";
import type { InboxResponse } from "../api/InboxResponse";
import type { NewsEvent } from "../api/NewsEvent";
import { LiveThreadRow, T3Notice } from "../components/Agents";
import { Crumbs } from "../components/Crumbs";
import { ItemLine } from "../components/ItemLine";
import { inboxCount } from "../components/Shell";
import { Empty, ErrorBanner, FetchedNote, ScopeBar, Skeleton } from "../components/ui";
import { api, to, useData } from "../lib/api";
import { countsLine, dateTime, plural } from "../lib/words";

function RepoInbox({ repo, many }: { repo: InboxRepo; many: boolean }) {
  const name = repo.repo;
  const count = inboxCount(repo);
  return (
    <section className="inbox-repo" aria-label={name}>
      {many ? (
        <h2 className="repo-title">
          <Link to={to.repo(name)}>{name}</Link> <span className="tally">{count}</span>
        </h2>
      ) : null}
      {repo.error ? (
        <div className="banner" role="alert">
          <span>
            {name} cannot be read: {repo.error}
          </span>
        </div>
      ) : count === 0 ? (
        <div className="list">
          <Empty title="Nothing waits on you here." />
        </div>
      ) : null}

      {repo.proposed.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Plans proposed, which you may stop</h3>
            <span className="tally">{repo.proposed.length}</span>
          </div>
          <div className="list">
            {repo.proposed.map((m) => (
              <Link key={m.id} to={to.mission(name, m.id)} className="mission-row wait" data-testid="proposed">
                <span className="t">
                  <span className="glyph you">
                    <Target size={15} aria-hidden />
                  </span>
                  <span className="truncate-2">{m.title}</span>
                </span>
                <span className="n" title={countsLine(m.counts)}>
                  <ScopeBar counts={m.counts} />
                </span>
              </Link>
            ))}
          </div>
        </div>
      ) : null}

      {repo.threads.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Threads waiting on a person</h3>
            <span className="tally">{repo.threads.length}</span>
          </div>
          <div className="list">
            {repo.threads.map((t) => (
              <LiveThreadRow key={t.id} thread={t} holds={t.holds} />
            ))}
          </div>
        </div>
      ) : null}

      {repo.needs_human.map((group) => (
        <div key={group.mission ?? ""} className="sect" data-testid="needs-human">
          <div className="sect-head">
            <h3>{group.mission ? <Link to={to.mission(name, group.mission)}>{group.mission_title ?? group.mission}</Link> : "In no mission"}</h3>
            <span className="tally">{group.items.length}</span>
          </div>
          <div className="list">
            {group.items.map((row) => (
              <ItemLine key={row.id} repo={name} row={row} />
            ))}
          </div>
        </div>
      ))}
      {repo.error ? null : <FetchedNote fetched={repo.fetched} />}
    </section>
  );
}

/** Plans proposed and merged lately, as the Desk saw them and told the human. */
function News({ news }: { news: NewsEvent[] }) {
  if (news.length === 0) return null;
  return (
    <section className="inbox-repo" aria-label="Lately">
      <h2 className="repo-title">
        Lately <span className="tally">{news.length}</span>
      </h2>
      <div className="list">
        {news.map((n, at) => (
          <Link key={at} to={to.mission(n.repo, n.mission)} className="mission-row" data-testid="news">
            <span className="t">
              <span className={`glyph ${n.kind === "proposed" ? "you" : "ok"}`}>
                {n.kind === "proposed" ? <GitPullRequestArrow size={15} aria-hidden /> : <GitMerge size={15} aria-hidden />}
              </span>
              <span className="truncate-2">
                {n.kind === "proposed" ? "Proposed" : "Merged"}: {n.title}
              </span>
            </span>
            <span className="n muted">
              {n.repo}, {dateTime(n.at)}
            </span>
          </Link>
        ))}
      </div>
    </section>
  );
}

/** `/inbox`: what waits on the human, in every repository, and the plans proposed and merged lately. */
export function InboxPage() {
  const loaded = useData<InboxResponse>(api.inbox());
  const repos = loaded.data?.repos ?? [];
  const count = repos.reduce((n, r) => n + inboxCount(r), 0);
  return (
    <div className="page narrow">
      <Crumbs trail={[{ label: "Inbox" }]} />
      <div className="page-head">
        <div>
          <h1>Inbox</h1>
          <p className="lede">
            {loaded.data ? (count === 0 ? "Nothing waits on you." : `${plural(count, "thing waits", "things wait")} on you.`) : "Reading every repository…"} Proposed plans
            go ahead unless you stop them, threads wait for an answer, and items labelled needs-human for work only you can do.
          </p>
        </div>
      </div>
      <ErrorBanner loaded={loaded} />
      {loaded.data ? <T3Notice t3={loaded.data.t3} /> : null}
      {!loaded.data && loaded.loading ? <Skeleton rows={8} /> : null}
      {repos.map((r) => (
        <RepoInbox key={r.repo} repo={r} many={repos.length > 1} />
      ))}
      {loaded.data ? <News news={loaded.data.news} /> : null}
    </div>
  );
}
