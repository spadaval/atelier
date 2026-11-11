import { MessageCircleQuestion, Target } from "lucide-react";
import { Link } from "react-router";
import type { InboxRepo } from "../api/InboxRepo";
import type { InboxResponse } from "../api/InboxResponse";
import { Crumbs } from "../components/Crumbs";
import { ItemLine } from "../components/ItemLine";
import { inboxCount } from "../components/Shell";
import { Empty, ErrorBanner, ScopeBar, Skeleton } from "../components/ui";
import { api, to, useData } from "../lib/api";
import { countsLine, plural } from "../lib/words";

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

      {repo.drafts.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Draft missions, waiting for your go</h3>
            <span className="tally">{repo.drafts.length}</span>
          </div>
          <div className="list">
            {repo.drafts.map((m) => (
              <Link key={m.id} to={to.mission(name, m.id)} className="mission-row wait">
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

      {repo.questions.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Open questions</h3>
            <span className="tally">{repo.questions.length}</span>
          </div>
          <div className="list">
            {repo.questions.map((q) => (
              <Link key={q.item} to={to.item(name, q.item)} className="qline">
                <span className="glyph you">
                  <MessageCircleQuestion size={15} aria-hidden />
                </span>
                <span className="grow">
                  <span className="q">{q.question}</span>
                  <span className="sub">
                    <span className="mono">{q.item}</span>
                    {q.finding ? <> · about finding {q.finding}</> : null}
                  </span>
                </span>
              </Link>
            ))}
          </div>
        </div>
      ) : null}

      {repo.needs_human.map((group) => (
        <div key={group.mission ?? ""} className="sect">
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
    </section>
  );
}

/** `/inbox`: what waits on the human, in every repository: missions to say go on, questions, and work only he can do. */
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
            {loaded.data ? (count === 0 ? "Nothing waits on you." : `${plural(count, "thing waits", "things wait")} on you.`) : "Reading every repository…"} Draft missions wait for
            your go, questions for an answer, and items labelled needs-human for work only you can do.
          </p>
        </div>
      </div>
      <ErrorBanner loaded={loaded} />
      {!loaded.data && loaded.loading ? <Skeleton rows={8} /> : null}
      {repos.map((r) => (
        <RepoInbox key={r.repo} repo={r} many={repos.length > 1} />
      ))}
    </div>
  );
}
