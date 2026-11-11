import { ChevronDown, ChevronRight, FileText, MessageCircleQuestion } from "lucide-react";
import { useEffect, useState } from "react";
import { Link, useLocation, useParams, useSearchParams } from "react-router";
import type { DocResponse } from "../api/DocResponse";
import type { ItemPage } from "../api/ItemPage";
import type { ItemRow } from "../api/ItemRow";
import type { MissionPage } from "../api/MissionPage";
import type { MissionRow } from "../api/MissionRow";
import type { MissionsResponse } from "../api/MissionsResponse";
import type { Question } from "../api/Question";
import { Crumbs } from "../components/Crumbs";
import { ItemLine } from "../components/ItemLine";
import { Findings, ReviewList } from "../components/Reviews";
import { Card, Counts, Empty, ErrorBanner, Markdown, ScopeBar, Skeleton, StatusGlyph, Tabs, useTab } from "../components/ui";
import { api, to, useData } from "../lib/api";
import { fixedInReview, groupByStatus, waitingOnYou } from "../lib/group";
import { fileName, recordPath, splitFrontMatter } from "../lib/links";
import { countsLine, dateTime, finished, missionState, plural, shortCommit, shortDate, statusTitle, total, who } from "../lib/words";

/** Which mission a repository opens on: one with work in progress, else a ready one, else the first. */
export function currentMission(missions: MissionRow[]): string | undefined {
  return (missions.find((m) => m.in_progress) ?? missions.find((m) => m.status === "ready") ?? missions[0])?.id;
}

function MissionCard({ repo, mission, current }: { repo: string; mission: MissionRow; current: boolean }) {
  const all = total(mission.counts);
  return (
    <Link to={to.mission(repo, mission.id)} className="mcard" aria-current={current ? "page" : undefined}>
      <span className="mcard-t">
        <StatusGlyph status={missionState(mission)} size={14} label={mission.in_progress ? "Work in progress" : statusTitle(mission.status)} />
        <span className="mcard-title">{mission.title}</span>
      </span>
      <span className="mcard-b">
        <ScopeBar counts={mission.counts} />
        <span className="mono" title={`${finished(mission.counts)} of ${all} items done or dropped`}>
          {finished(mission.counts)}/{all}
        </span>
        {mission.waiting.length > 0 ? <span className="tag you">{mission.waiting.length} for you</span> : null}
      </span>
    </Link>
  );
}

function Rail({ repo, body, selected }: { repo: string; body: MissionsResponse | undefined; selected?: string }) {
  const missions = body?.missions ?? [];
  const going = missions.filter((m) => m.in_progress);
  const ready = missions.filter((m) => !m.in_progress && m.status === "ready");
  const drafts = missions.filter((m) => m.status === "draft");
  const [showDrafts, setShowDrafts] = useState(true);
  return (
    <nav className="pane rail-pane" aria-label={`Missions of ${repo}`}>
      <div className="pane-head">
        <b>{repo}</b>
        {body ? <span className="muted mono">{shortCommit(body.commit)}</span> : null}
      </div>
      {!body ? <Skeleton rows={5} /> : null}
      {going.length > 0 ? <div className="pane-label">In progress</div> : null}
      {going.map((m) => (
        <MissionCard key={m.id} repo={repo} mission={m} current={m.id === selected} />
      ))}
      {ready.length > 0 ? <div className="pane-label">Ready, nothing started</div> : null}
      {ready.map((m) => (
        <MissionCard key={m.id} repo={repo} mission={m} current={m.id === selected} />
      ))}
      {drafts.length > 0 ? (
        <button type="button" className="pane-label" aria-expanded={showDrafts} onClick={() => setShowDrafts(!showDrafts)}>
          {showDrafts ? <ChevronDown size={12} /> : <ChevronRight size={12} />} Drafts, waiting for your go <span className="tally">{drafts.length}</span>
        </button>
      ) : null}
      {showDrafts ? drafts.map((m) => <MissionCard key={m.id} repo={repo} mission={m} current={m.id === selected} />) : null}
      {body && missions.length === 0 ? <Empty title="No open missions.">A mission is a batch of work agents run once you say go.</Empty> : null}
      {body ? (
        <p className="loose" data-testid="loose">
          {body.loose === 0 ? "No open item is outside a mission." : `${plural(body.loose, "open item is", "open items are")} in no mission.`}
        </p>
      ) : null}
    </nav>
  );
}

const HEADINGS = ["Why", "Success criteria", "Scope", "Authority"] as const;

function QuestionLine({ repo, question }: { repo: string; question: Question }) {
  return (
    <Link to={to.item(repo, question.item)} className="qline">
      <span className="glyph you">
        <MessageCircleQuestion size={15} aria-hidden />
      </span>
      <span className="grow">
        <span className="q">{question.question}</span>
        <span className="sub">
          <span className="mono">{question.item}</span>
          {question.finding ? <> · about finding {question.finding}</> : null}
        </span>
      </span>
    </Link>
  );
}

function Decided({ repo, questions }: { repo: string; questions: Question[] }) {
  const answered = questions.filter((q) => q.answer !== null);
  if (answered.length === 0) return null;
  return (
    <section className="prose-sect">
      <h2>
        Decided on the way <span className="tally">{answered.length}</span>
      </h2>
      <ol className="decisions">
        {answered.map((q, at) => (
          <li key={`${q.item}${at}`}>
            <div className="q">{q.question}</div>
            <div className="a">
              {q.answer!.text}
              {q.outcome ? <span className="tag">{q.outcome === "defer" ? "deferred" : "no action"}</span> : null}
            </div>
            <div className="sub">
              <Link to={to.item(repo, q.item)} className="mono">
                {q.item}
              </Link>{" "}
              · answered by <span title={q.answer!.by}>{who(q.answer!.by)}</span>
            </div>
          </li>
        ))}
      </ol>
    </section>
  );
}

function Overview({ repo, page }: { repo: string; page: MissionPage }) {
  const section = (heading: string) => page.sections.find((s) => s.heading.toLowerCase() === heading.toLowerCase());
  const from = recordPath.mission(page.mission.id);
  const docHref = (path: string, hash: string) => `${to.doc(repo, page.mission.id, path)}${hash ? `#${hash}` : ""}`;
  const waiting = waitingOnYou(page.items, page.questions);
  const result = section("Result");
  return (
    <>
      {page.dropped ? (
        <div className="banner quiet">
          <b>Dropped.</b> {page.dropped}
        </div>
      ) : null}
      {waiting.questions.length + waiting.items.length > 0 ? (
        <section className="prose-sect">
          <h2>
            Waiting on you <span className="tally">{waiting.questions.length + waiting.items.length}</span>
          </h2>
          <div className="list">
            {waiting.questions.map((q, at) => (
              <QuestionLine key={`${q.item}${at}`} repo={repo} question={q} />
            ))}
            {waiting.items.map((row) => (
              <ItemLine key={row.id} repo={repo} row={row} />
            ))}
          </div>
        </section>
      ) : null}
      {result ? (
        <section className="prose-sect">
          <h2>Result</h2>
          <Markdown text={result.markdown} from={from} docHref={docHref} />
        </section>
      ) : null}
      {HEADINGS.map((heading) => {
        const s = section(heading);
        return s ? (
          <section key={heading} className="prose-sect">
            <h2>{heading}</h2>
            <Markdown text={s.markdown} from={from} docHref={docHref} />
          </section>
        ) : null;
      })}
      {!result && page.mission.status !== "draft" ? (
        <section className="prose-sect">
          <h2>Result</h2>
          <p className="muted">Not written yet. A mission's Result is written when it closes.</p>
        </section>
      ) : null}
      <section className="prose-sect">
        <h2>
          Plan reviews <span className="tally">{page.plan_reviews.length}</span>
        </h2>
        {page.plan_reviews.length === 0 ? <p className="muted">No one has reviewed this plan.</p> : <ReviewList reviews={page.plan_reviews} plan />}
      </section>
      <Decided repo={repo} questions={page.questions} />
    </>
  );
}

/** A done item's Report and the findings fixed in review, read from its own page when opened. */
function DoneDetail({ repo, id }: { repo: string; id: string }) {
  const loaded = useData<ItemPage>(api.item(repo, id), 120_000);
  if (!loaded.data) return <div className="report">{loaded.error ? <ErrorBanner loaded={loaded} /> : <Skeleton rows={2} />}</div>;
  const fixed = fixedInReview(loaded.data.reviews);
  return (
    <div className="report">
      {loaded.data.report ? <Markdown text={loaded.data.report} small from={recordPath.item(id)} /> : <p className="muted">No Report was written.</p>}
      {fixed.length > 0 ? (
        <>
          <h4>Fixed in review</h4>
          <Findings findings={fixed.map((f) => f.finding)} />
        </>
      ) : null}
    </div>
  );
}

function DoneLine({ repo, row }: { repo: string; row: ItemRow }) {
  const [open, setOpen] = useState(false);
  return (
    <ItemLine
      repo={repo}
      row={row}
      after={
        <>
          <button type="button" className="disclose" aria-expanded={open} onClick={() => setOpen(!open)}>
            {open ? <ChevronDown size={13} /> : <ChevronRight size={13} />} {open ? "Hide the Report" : "Report and fixes"}
          </button>
          {open ? <DoneDetail repo={repo} id={row.id} /> : null}
        </>
      }
    />
  );
}

function Work({ repo, items }: { repo: string; items: ItemRow[] }) {
  const groups = groupByStatus(items);
  if (groups.length === 0) {
    return (
      <div className="list">
        <Empty title="No items yet.">The mission's items show here, grouped by where they stand.</Empty>
      </div>
    );
  }
  return (
    <>
      {groups.map((group) => (
        <section key={group.status} className="work-group" aria-label={statusTitle(group.status)}>
          <h2 className="group-title">
            <StatusGlyph status={group.status} size={13} quiet />
            {statusTitle(group.status)} <span className="tally">{group.items.length}</span>
          </h2>
          <div className="list">
            {group.items.map((row) => (row.status === "done" ? <DoneLine key={row.id} repo={repo} row={row} /> : <ItemLine key={row.id} repo={repo} row={row} />))}
          </div>
        </section>
      ))}
    </>
  );
}

function DocReader({ repo, mission, docs, chosen }: { repo: string; mission: string; docs: MissionPage["documents"]; chosen: string | null }) {
  const path = chosen ?? docs[0]?.path ?? null;
  const loaded = useData<DocResponse>(path ? api.doc(repo, path) : null, 120_000);
  const { hash } = useLocation();
  useEffect(() => {
    if (!loaded.data || !hash) return;
    document.getElementById(decodeURIComponent(hash.slice(1)))?.scrollIntoView({ block: "start" });
  }, [loaded.data, hash]);
  const listed = docs.some((d) => d.path === path);
  if (!path) {
    return (
      <div className="list">
        <Empty title="This mission has no documents.">Files beside its record, such as its design, show here.</Empty>
      </div>
    );
  }
  return (
    <div className="docs">
      <ul className="doc-list">
        {docs.map((doc) => (
          <li key={doc.path}>
            <Link to={to.doc(repo, mission, doc.path)} replace aria-current={doc.path === path ? "true" : undefined}>
              <span className="glyph dim">
                <FileText size={15} aria-hidden />
              </span>
              <span>
                <span className="doc-title">{fileName(doc.path)}</span>
                <span className="doc-path">{doc.title}</span>
              </span>
            </Link>
          </li>
        ))}
        {!listed ? (
          <li>
            <span className="doc-open" aria-current="true">
              <span className="glyph dim">
                <FileText size={15} aria-hidden />
              </span>
              <span>
                <span className="doc-title">{fileName(path)}</span>
                <span className="doc-path">{path}</span>
              </span>
            </span>
          </li>
        ) : null}
      </ul>
      <article className="doc-body" aria-label={path}>
        <ErrorBanner loaded={loaded} />
        {!loaded.data && loaded.loading ? <Skeleton rows={6} /> : null}
        {loaded.data && loaded.data.path === path ? (
          <>
            <p className="doc-where mono">{loaded.data.path}</p>
            <FrontMatter fields={splitFrontMatter(loaded.data.markdown).fields} />
            <Markdown text={splitFrontMatter(loaded.data.markdown).body} from={loaded.data.path} docHref={(p, h) => `${to.doc(repo, mission, p)}${h ? `#${h}` : ""}`} />
          </>
        ) : null}
      </article>
    </div>
  );
}

function FrontMatter({ fields }: { fields: [string, string][] }) {
  if (fields.length === 0) return null;
  return (
    <dl className="kv front">
      {fields.map(([key, value]) => (
        <div key={key} style={{ display: "contents" }}>
          <dt>{key.replaceAll("_", " ")}</dt>
          <dd>{value}</dd>
        </div>
      ))}
    </dl>
  );
}

type MissionTab = "overview" | "work" | "docs";
const TABS = ["overview", "work", "docs"] as const;

function MissionView({ repo, id }: { repo: string; id: string }) {
  const loaded = useData<MissionPage>(api.mission(repo, id));
  const [tab, setTab] = useTab<MissionTab>("tab", TABS, "overview");
  const [params] = useSearchParams();
  const page = loaded.data;
  if (!page || page.mission.id !== id) {
    return (
      <div className="pane center-pane">
        <Crumbs trail={[{ label: repo, to: to.repo(repo) }, { label: id }]} />
        <article className="paper">
          <div className="paper-body">
            <ErrorBanner loaded={loaded} />
            {loaded.loading ? <Skeleton rows={6} /> : null}
          </div>
        </article>
      </div>
    );
  }
  const m = page.mission;
  const outcome = page.sections.find((s) => s.heading.toLowerCase() === "outcome");
  const from = recordPath.mission(m.id);
  const docHref = (path: string, hash: string) => `${to.doc(repo, m.id, path)}${hash ? `#${hash}` : ""}`;
  return (
    <>
      <div className="pane center-pane">
        <Crumbs trail={[{ label: repo, to: to.repo(repo) }, { label: m.title }]} />
        <ErrorBanner loaded={loaded} />
        <article className="paper">
          <header className="paper-head">
            <div className="ids">
              <span className={`status-pill ${m.status === "draft" ? "you" : m.status === "dropped" ? "" : "plain"}`} data-testid="status">
                <StatusGlyph status={m.status} size={13} quiet /> {statusTitle(m.status)}
              </span>
              {m.in_progress ? <span className="tag run">Work in progress</span> : null}
              <span className="mono muted">{m.id}</span>
            </div>
            <h1>{m.title}</h1>
            <div className="mcounts">
              <Counts counts={m.counts} />
            </div>
            {outcome ? (
              <div className="outcome">
                <h2 className="sr">Outcome</h2>
                <Markdown text={outcome.markdown} from={from} docHref={docHref} />
              </div>
            ) : null}
            {m.status === "draft" ? (
              <p className="go-note">
                <b>This mission is a draft.</b> Agents start its items only after you say go.
              </p>
            ) : null}
          </header>
          <div className="paper-tabs">
            <Tabs
              value={tab}
              onChange={setTab}
              tabs={[
                ["overview", "Overview"],
                ["work", <>Work <span className="tally">{total(m.counts)}</span></>],
                ["docs", <>Documents <span className="tally">{page.documents.length}</span></>],
              ]}
            />
          </div>
          <div className="paper-body">
            {tab === "overview" ? <Overview repo={repo} page={page} /> : null}
            {tab === "work" ? <Work repo={repo} items={page.items} /> : null}
            {tab === "docs" ? <DocReader repo={repo} mission={m.id} docs={page.documents} chosen={params.get("doc")} /> : null}
          </div>
        </article>
      </div>

      <aside className="pane side-pane" aria-label="Mission details">
        <Card title="Progress">
          <ScopeBar counts={m.counts} tall />
          <p className="progress-line">
            {finished(m.counts)} of {plural(total(m.counts), "item")} done or dropped. {countsLine(m.counts)}.
          </p>
        </Card>
        <Card title="Details">
          <dl className="kv">
            <dt>Status</dt>
            <dd>{statusTitle(m.status)}</dd>
            <dt>Lead</dt>
            <dd title={m.lead ?? undefined}>{m.lead ? who(m.lead) : "Nobody"}</dd>
            <dt>Go</dt>
            <dd>{m.ready ? `${m.ready.by}, ${dateTime(m.ready.at)}` : "Not given yet"}</dd>
            <dt>Created</dt>
            <dd>{shortDate(m.created_at)}</dd>
            {m.labels.length > 0 ? (
              <>
                <dt>Labels</dt>
                <dd className="pills">
                  {m.labels.map((l) => (
                    <span key={l} className="tag">
                      {l}
                    </span>
                  ))}
                </dd>
              </>
            ) : null}
            <dt>Read at</dt>
            <dd className="mono">{shortCommit(page.commit)}</dd>
          </dl>
        </Card>
        <Card title="Documents" tally={page.documents.length} flush>
          {page.documents.length === 0 ? <p className="card-empty">No document beside the mission's record.</p> : null}
          {page.documents.map((doc) => (
            <Link key={doc.path} className="doc-link" to={to.doc(repo, m.id, doc.path)} title={doc.path}>
              <span className="glyph dim">
                <FileText size={15} aria-hidden />
              </span>
              <span className="truncate">{fileName(doc.path)}</span>
            </Link>
          ))}
        </Card>
      </aside>
    </>
  );
}

/** `/r/:repo` and `/r/:repo/mission/:id`: a repository is its missions. One is open; the rest wait in the rail. */
export function MissionPageView() {
  const { repo = "", id } = useParams();
  const loaded = useData<MissionsResponse>(api.missions(repo));
  const body = loaded.data;
  const selected = id ?? (body ? currentMission(body.missions) : undefined);
  return (
    <div className={`work${id ? " picked" : ""}${selected ? "" : " no-side"}`}>
      <Rail repo={repo} body={body} selected={selected} />
      {selected ? (
        <MissionView key={selected} repo={repo} id={selected} />
      ) : (
        <div className="pane center-pane">
          <Crumbs trail={[{ label: repo }]} />
          <ErrorBanner loaded={loaded} />
          {body ? (
            <article className="paper">
              <div className="paper-body">
                <Empty title="This repository has no open mission.">{`${plural(body.loose, "open item is", "open items are")} in no mission.`}</Empty>
              </div>
            </article>
          ) : loaded.loading ? (
            <Skeleton rows={8} />
          ) : null}
        </div>
      )}
    </div>
  );
}
