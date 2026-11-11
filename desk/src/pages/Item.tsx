import { CircleCheck, Square } from "lucide-react";
import { Link, useParams } from "react-router";
import type { ItemPage } from "../api/ItemPage";
import type { MissionPage } from "../api/MissionPage";
import { AnswerForm } from "../components/Acts";
import { AgentsCard, Identity } from "../components/Agents";
import { Crumbs } from "../components/Crumbs";
import { PendingMark, questionKey } from "../components/Questions";
import { itemFacts } from "../components/ItemLine";
import { ReviewList } from "../components/Reviews";
import { Card, Empty, ErrorBanner, Markdown, Priority, Skeleton, StatusGlyph } from "../components/ui";
import { withPending } from "../lib/acts";
import { api, to, useData } from "../lib/api";
import { recordPath } from "../lib/links";
import { dateTime, shortCommit, shortDate, statusName, statusTitle } from "../lib/words";


const pillTone: Record<string, string> = { building: "run", in_review: "review", changes_requested: "bad", approved: "ok", done: "ok" };

function ItemDetail({ repo, page }: { repo: string; page: ItemPage }) {
  const { row } = page;
  const from = recordPath.item(row.id);
  // A Markdown file the item links to opens in its mission's Documents tab.
  const docHref = page.mission ? (path: string, hash: string) => `${to.doc(repo, page.mission!, path)}${hash ? `#${hash}` : ""}` : undefined;
  const md = (text: string) => <Markdown text={text} from={from} docHref={docHref} />;
  const facts = itemFacts(row);
  const ticked = page.done_when.filter((d) => d.ticked).length;
  const decisions = withPending(page.decisions, page.pending_answers);
  // Only the oldest open question can be answered; one answered from the Desk is no longer open.
  const answerable = decisions.findIndex((d) => d.question.answer === null && d.pending === null);
  return (
    <div className="issue">
      <article className="issue-main">
        <div className="ids">
          <span className={`status-pill ${pillTone[row.status] ?? "plain"}`} data-testid="status">
            <StatusGlyph status={row.status} size={13} quiet /> {statusTitle(row.status)}
          </span>
          <span className="mono muted">{row.id}</span>
          <span className="muted read-from">{page.read_from === "branch" ? "Read from its branch" : "Read from the base"}</span>
        </div>
        <h1>{row.title}</h1>
        {facts.length > 0 ? (
          <ul className="facts-line big">
            {facts.map((f) => (
              <li key={f}>{f}</li>
            ))}
          </ul>
        ) : null}

        {page.goal ? (
          <section className="prose-sect">
            <h2>Goal</h2>
            {md(page.goal)}
          </section>
        ) : null}
        <section className="prose-sect">
          <h2>
            Done when{" "}
            <span className="tally">
              {ticked} of {page.done_when.length}
            </span>
          </h2>
          {page.done_when.length === 0 ? (
            <p className="muted">No Done when lines are written.</p>
          ) : (
            <ul className="done-when">
              {page.done_when.map((d, at) => (
                <li key={at} data-ticked={d.ticked}>
                  <span className={`glyph ${d.ticked ? "ok" : "dim"}`} title={d.ticked ? "Ticked" : "Not ticked"}>
                    {d.ticked ? <CircleCheck size={15} aria-hidden /> : <Square size={14} aria-hidden />}
                    <span className="sr">{d.ticked ? "Ticked: " : "Not ticked: "}</span>
                  </span>
                  <Markdown text={d.text} small from={from} docHref={docHref} />
                </li>
              ))}
            </ul>
          )}
        </section>
        {page.scope ? (
          <section className="prose-sect">
            <h2>Scope</h2>
            {md(page.scope)}
          </section>
        ) : null}
        <section className="prose-sect">
          <h2>Report</h2>
          {page.report ? md(page.report) : <p className="muted">Not written yet.</p>}
        </section>
        {page.decisions.length > 0 ? (
          <section className="prose-sect">
            <h2>
              Questions and decisions <span className="tally">{page.decisions.length}</span>
            </h2>
            <ol className="decisions">
              {decisions.map(({ question: q, pending }, at) => {
                const answer = q.answer ?? pending?.answer ?? null;
                const outcome = q.answer ? q.outcome : (pending?.outcome ?? null);
                return (
                  <li key={at} data-testid="decision">
                    <div className="q">{q.question}</div>
                    {answer ? (
                      <div className="a">
                        {answer.text}
                        {outcome ? <span className="tag">{outcome === "defer" ? "deferred" : "no action"}</span> : null}
                        {pending ? <PendingMark /> : null}
                      </div>
                    ) : (
                      <div className="a open">Not answered yet.</div>
                    )}
                    <div className="sub">
                      {q.finding ? <>About finding {q.finding}. </> : null}
                      {answer ? (
                        <>
                          Answered by <Identity id={answer.by} />
                          {pending ? ". It is written in the item's checkout, and its implementer's next submit commits it." : null}
                        </>
                      ) : null}
                    </div>
                    {at === answerable ? (
                      page.can_decide ? (
                        <AnswerForm key={questionKey(q)} repo={repo} item={row.id} finding={q.finding} />
                      ) : (
                        <p className="sub">
                          {page.decide_at === null
                            ? "It can be answered here once a checkout has the item's branch out, since the answer is written there."
                            : "You cannot answer it from here now."}
                        </p>
                      )
                    ) : null}
                  </li>
                );
              })}
            </ol>
          </section>
        ) : null}
        {page.responses.length > 0 ? (
          <section className="prose-sect">
            <h2>
              Responses to findings <span className="tally">{page.responses.length}</span>
            </h2>
            <ul className="responses">
              {page.responses.map((r, at) => (
                <li key={at}>
                  <span className="mono">{r.finding}</span>{" "}
                  {r.kind === "fixed" ? (
                    <>
                      fixed in <span className="mono">{shortCommit(r.commit)}</span>
                    </>
                  ) : (
                    <>rebutted: {r.fact}</>
                  )}
                </li>
              ))}
            </ul>
          </section>
        ) : null}
        <section className="prose-sect">
          <h2>
            Reviews <span className="tally">{page.reviews.length}</span>
          </h2>
          {page.reviews.length === 0 ? <p className="muted">No review yet.</p> : <ReviewList reviews={page.reviews} />}
          {page.unsettled.length > 0 ? (
            <p className="sub">
              Deferred and not settled: <span className="mono">{page.unsettled.join(", ")}</span>
            </p>
          ) : null}
        </section>
        <details className="whole">
          <summary>The whole record</summary>
          {md(page.body)}
        </details>
      </article>

      <aside className="issue-side" aria-label="Item details">
        <AgentsCard repo={repo} agents={page.threads} t3={page.t3} none="No thread has started this item." />
        <Card title="Details">
          <dl className="kv">
            <dt>Status</dt>
            <dd>{statusTitle(row.status)}</dd>
            <dt>Mission</dt>
            <dd>{page.mission ? <MissionName repo={repo} id={page.mission} /> : "None"}</dd>
            <dt>Priority</dt>
            <dd className="row" style={{ gap: 6 }}>
              <Priority priority={row.priority} /> {row.priority}
            </dd>
            {row.labels.length > 0 ? (
              <>
                <dt>Labels</dt>
                <dd className="pills">
                  {row.labels.map((l) => (
                    <span key={l} className={`tag${l === "needs-human" ? " you" : ""}`}>
                      {l}
                    </span>
                  ))}
                </dd>
              </>
            ) : null}
            <dt>Holder</dt>
            <dd>{row.holder ? <Identity id={row.holder} /> : "Nobody"}</dd>
            {page.implementers.length > 1 ? (
              <>
                <dt>Earlier</dt>
                <dd>
                  {page.implementers.slice(0, -1).map((i) => (
                    <div key={i}>
                      <Identity id={i} />
                    </div>
                  ))}
                </dd>
              </>
            ) : null}
            {row.reviewer ? (
              <>
                <dt>Reviewer</dt>
                <dd>
                  <Identity id={row.reviewer} />
                </dd>
              </>
            ) : null}
            <dt>Ready</dt>
            <dd>{page.ready ? `${page.ready.by}, ${dateTime(page.ready.at)}` : "Not yet"}</dd>
            {row.branch ? (
              <>
                <dt>Branch</dt>
                <dd>{row.branch.merged ? "Merged" : "Not merged"}</dd>
              </>
            ) : null}
            {row.approved_at ? (
              <>
                <dt>Approved</dt>
                <dd>{dateTime(row.approved_at)}</dd>
              </>
            ) : null}
            {page.trivial ? (
              <>
                <dt>Trivial</dt>
                <dd>{page.trivial}</dd>
              </>
            ) : null}
            {page.caused_by ? (
              <>
                <dt>Caused by</dt>
                <dd>
                  <Link to={to.item(repo, page.caused_by)} className="mono">
                    {page.caused_by}
                  </Link>
                </dd>
              </>
            ) : null}
            <dt>Created</dt>
            <dd>{shortDate(page.created_at)}</dd>
            <dt>Read at</dt>
            <dd className="mono">{shortCommit(page.commit)}</dd>
          </dl>
        </Card>
        <Card title="Blocked by" tally={row.blockers.length} flush>
          {row.blockers.length === 0 ? <p className="card-empty">Nothing blocks it.</p> : null}
          {row.blockers.map((b) => (
            <Link key={b.id} to={to.item(repo, b.id)} className="doc-link">
              <StatusGlyph status={b.status ?? "draft"} size={13} quiet />
              <span className="truncate">
                <span className="mono">{b.id}</span> <span className="muted">{b.status ? statusName(b.status) : "not found"}</span>
              </span>
            </Link>
          ))}
        </Card>
      </aside>
    </div>
  );
}

/** A mission's title where it is known, its ID otherwise. */
function MissionName({ repo, id }: { repo: string; id: string }) {
  const loaded = useData<MissionPage>(api.mission(repo, id), 120_000);
  return <Link to={to.mission(repo, id)}>{loaded.data?.mission.title ?? id}</Link>;
}

/** `/r/:repo/item/:id`: one item in full. */
export function ItemPageView() {
  const { repo = "", id = "" } = useParams();
  const loaded = useData<ItemPage>(api.item(repo, id));
  const page = loaded.data && loaded.data.row.id === id ? loaded.data : undefined;
  return (
    <div className="issue-wrap">
      <Crumbs
        trail={[
          { label: repo, to: to.repo(repo) },
          ...(page?.mission ? [{ label: page.mission, to: to.mission(repo, page.mission) }] : []),
          { label: page?.row.title ?? id },
        ]}
      />
      <ErrorBanner loaded={loaded} />
      {/* Keyed by the item, so a form or fold opened on one item never carries to the next one shown here. */}
      {page ? <ItemDetail key={JSON.stringify([repo, id])} repo={repo} page={page} /> : loaded.loading ? <Skeleton rows={8} /> : !loaded.error ? <Empty title="No such item." /> : null}
    </div>
  );
}
