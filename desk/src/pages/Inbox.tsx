import { CircleCheck, Target } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router";
import type { InboxRepo } from "../api/InboxRepo";
import type { InboxQuestion } from "../api/InboxQuestion";
import type { InboxResponse } from "../api/InboxResponse";
import type { ItemPage } from "../api/ItemPage";
import { LiveThreadRow, T3Notice } from "../components/Agents";
import { Crumbs } from "../components/Crumbs";
import { ItemLine } from "../components/ItemLine";
import { PendingMark, QuestionLine, questionKey } from "../components/Questions";
import { inboxCount } from "../components/Shell";
import { Empty, ErrorBanner, ScopeBar, Skeleton } from "../components/ui";
import { api, to, useData } from "../lib/api";
import { countsLine, plural } from "../lib/words";

/** A question answered from this inbox: the question, the answer given, and the act's text. */
type Answered = InboxQuestion & { answer: string; text: string };

/**
 * A question answered from this inbox, shown while its item's page lists the
 * answer in `pending_answers`, which the inbox does not carry. Once a commit
 * holds the answer it is no longer pending, and the line goes.
 */
function AnsweredLine({ repo, answered: a }: { repo: string; answered: Answered }) {
  const item = useData<ItemPage>(api.item(repo, a.item));
  const pending = item.data?.pending_answers.some((p) => p.question === a.question && p.finding === a.finding && p.answer?.text === a.answer);
  if (item.data && !pending) return null;
  return (
    <div className="qline" data-testid="answered">
      <span className="glyph ok">
        <CircleCheck size={15} aria-hidden />
      </span>
      <span className="grow">
        <Link to={to.item(repo, a.item)} className="q">
          {a.question}
        </Link>
        <span className="sub">
          <span className="mono">{a.item}</span> <PendingMark /> Its item's page shows the answer until its implementer commits it.
        </span>
        <p className="act-text">{a.text}</p>
      </span>
    </div>
  );
}

function RepoInbox({ repo, many }: { repo: InboxRepo; many: boolean }) {
  const name = repo.repo;
  const count = inboxCount(repo);
  // Answers given here this visit. The server leaves an answered question out of the inbox until a commit holds it.
  const [answered, setAnswered] = useState<Answered[]>([]);
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
      ) : count === 0 && answered.length === 0 ? (
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

      {repo.questions.length + answered.length > 0 ? (
        <div className="sect">
          <div className="sect-head">
            <h3>Open questions</h3>
            <span className="tally">{repo.questions.length}</span>
          </div>
          <div className="list">
            {repo.questions.map((q) => (
              <QuestionLine
                key={questionKey(q)}
                repo={name}
                question={q}
                answerable
                onAnswered={(text, answer) => setAnswered((was) => [...was, { ...q, answer, text }])}
              />
            ))}
            {answered.map((a, at) => (
              <AnsweredLine key={at} repo={name} answered={a} />
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
            your go, threads and questions for an answer, and items labelled needs-human for work only you can do.
          </p>
        </div>
      </div>
      <ErrorBanner loaded={loaded} />
      {loaded.data ? <T3Notice t3={loaded.data.t3} /> : null}
      {!loaded.data && loaded.loading ? <Skeleton rows={8} /> : null}
      {repos.map((r) => (
        <RepoInbox key={r.repo} repo={r} many={repos.length > 1} />
      ))}
    </div>
  );
}
