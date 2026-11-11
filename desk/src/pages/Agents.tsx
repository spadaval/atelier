import { Link, useParams } from "react-router";
import type { LiveThread } from "../api/LiveThread";
import type { ThreadsResponse } from "../api/ThreadsResponse";
import { LiveThreadRow, T3Notice } from "../components/Agents";
import { Crumbs } from "../components/Crumbs";
import { Empty, ErrorBanner, Skeleton } from "../components/ui";
import { api, to, useData } from "../lib/api";
import { threadsFor } from "../lib/threads";
import { plural } from "../lib/words";

function headline(threads: LiveThread[]): string {
  const waiting = threads.filter((t) => t.status === "waiting").length;
  const working = threads.length - waiting;
  const first = working === 0 ? "No thread is working" : `${plural(working, "thread is", "threads are")} working`;
  return waiting === 0 ? `${first}.` : `${first}, and ${plural(waiting, "waits", "wait")} on a person.`;
}

/**
 * `/agents` and `/r/:repo/agents`: T3 Code's live threads, each with what
 * it holds in the records and a link to it in T3 Code.
 */
export function AgentsPage() {
  const { repo } = useParams();
  const loaded = useData<ThreadsResponse>(api.threads(), 10_000);
  const all = loaded.data?.threads ?? [];
  const { holding, free } = threadsFor(all, repo);
  const repos = [...new Set(all.flatMap((t) => t.holds.map((h) => h.repo)))].sort();
  return (
    <div className="page narrow">
      <Crumbs trail={repo === undefined ? [{ label: "Agents" }] : [{ label: repo, to: to.repo(repo) }, { label: "Agents" }]} />
      <div className="page-head">
        <div>
          <h1>{loaded.data ? headline([...holding, ...free]) : "Agents"}</h1>
          <p className="lede">
            T3 Code's live threads{repo === undefined ? "" : ` on ${repo}`}, each with the missions and items its records name it on. A thread's title opens it in T3 Code.
          </p>
        </div>
      </div>
      {repos.length > 0 ? (
        <nav className="pills toolbar" aria-label="Repositories">
          <Link to={to.agents()} className="pill" aria-current={repo === undefined ? "page" : undefined}>
            Every repository
          </Link>
          {repos.map((r) => (
            <Link key={r} to={to.agents(r)} className="pill" aria-current={repo === r ? "page" : undefined}>
              {r}
            </Link>
          ))}
        </nav>
      ) : null}
      <ErrorBanner loaded={loaded} />
      {loaded.data ? <T3Notice t3={loaded.data.t3} /> : null}
      {!loaded.data && loaded.loading ? <Skeleton rows={6} /> : null}
      {loaded.data && holding.length + free.length === 0 ? (
        <div className="list">
          <Empty title={repo === undefined ? "No thread is live in T3 Code." : `No live thread works on ${repo}.`}>A thread shows here once its records name it, as a holder, a reviewer, a lead or a decider.</Empty>
        </div>
      ) : null}
      {holding.length > 0 ? (
        <section className="sect" aria-label="Working on missions and items">
          <div className="sect-head">
            <h2>On missions and items</h2>
            <span className="tally">{holding.length}</span>
          </div>
          <div className="list">
            {holding.map((t) => (
              <LiveThreadRow key={t.id} thread={t} holds={t.holds} showRepo={repo === undefined} />
            ))}
          </div>
        </section>
      ) : null}
      {free.length > 0 ? (
        <section className="sect" aria-label="Holding nothing in the records">
          <div className="sect-head">
            <h2>Holding nothing in the records</h2>
            <span className="tally">{free.length}</span>
          </div>
          <div className="list">
            {free.map((t) => (
              <LiveThreadRow key={t.id} thread={t} holds={[]} />
            ))}
          </div>
        </section>
      ) : null}
    </div>
  );
}
