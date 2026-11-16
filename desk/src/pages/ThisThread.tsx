import { Check, ChevronRight, Clock, ExternalLink, GitPullRequest, Square, Target } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router";
import type { Agent } from "../api/Agent";
import type { InboxResponse } from "../api/InboxResponse";
import type { ItemPage } from "../api/ItemPage";
import type { MissionPage } from "../api/MissionPage";
import type { RecordName } from "../api/RecordName";
import type { ThreadInfo } from "../api/ThreadInfo";
import type { Tie } from "../api/Tie";
import type { TiesResponse } from "../api/TiesResponse";
import { LiveThreadRow } from "../components/Agents";
import { Crumbs } from "../components/Crumbs";
import { ErrorBanner, ScopeBar, Skeleton, StatusGlyph } from "../components/ui";
import { api, to, useAll, useData, useThreads } from "../lib/api";
import { openThread, useHostContext, type HostContext } from "../lib/host";
import { countsLine, finished, roleWords, statusName, statusTitle, total } from "../lib/words";
import { Block, ProjectGlance, Row, since, who } from "../components/Glance";

/** T3 Code's statuses for a thread whose work has ended. */
const ENDED = new Set(["completed", "cancelled", "failed", "interrupted", "rolled_back"]);
const STALE_MS = 30 * 60 * 1000;
const CLOSED = new Set(["done", "dropped"]);

/** What a thread is to a record, as a verb. */
const VERBS: Record<Tie["role"], [string, string]> = {
  implementer: ["implements", "implemented"],
  reviewer: ["reviews", "reviewed"],
  planner: ["plans", "planned"],
  lead: ["leads", "led"],
  decider: ["answers on", "answered on"],
};

function tieLine(ties: Tie[]): string | null {
  const current = ties.filter((t) => t.current);
  const shown = current.length > 0 ? current : ties.slice(-2);
  if (shown.length === 0) return null;
  const seen = new Set<string>();
  return shown
    .filter((t) => (seen.has(`${t.role}${t.id}`) ? false : (seen.add(`${t.role}${t.id}`), true)))
    .map((t) => `${VERBS[t.role][t.current ? 0 : 1]} ${t.id}${t.current ? "" : ` (${statusName(t.status)})`}`)
    .join(", ");
}

/** The live thread that is `identity` among a page's agents, as a short name. */
function agentName(agents: Agent[], identity: string | null): string | null {
  if (!identity) return null;
  return who(agents.find((a) => a.identity === identity) ?? null, identity);
}

function ItemHold({ tie, page }: { tie: Tie; page: ItemPage | null }) {
  const row = page?.row;
  const ticked = page?.done_when.filter((d) => d.ticked).length ?? 0;
  const reviewer = row ? agentName(page.threads, row.reviewer) : null;
  return (
    <section className="hold-card">
      <header>
        <StatusGlyph status={row?.status ?? tie.status} />
        <Link to={to.item(tie.repo, tie.id)} className="hold-title">
          <span className="id">{tie.id}</span> {row?.title ?? tie.title}
        </Link>
        <span className="tag">{statusTitle(row?.status ?? tie.status)}</span>
      </header>
      <p className="hold-role">This thread {VERBS[tie.role][0]} this item.</p>
      {page && page.done_when.length > 0 ? (
        <>
          <div className="hold-sub">
            Done when <span className="tally">{ticked}/{page.done_when.length}</span>
          </div>
          <ul className="ticks">
            {page.done_when.map((d, at) => (
              <li key={at} className={d.ticked ? "on" : undefined}>
                {d.ticked ? <Check size={13} aria-label="Ticked" /> : <Square size={13} aria-label="Not ticked" />}
                <span>{d.text}</span>
              </li>
            ))}
          </ul>
        </>
      ) : null}
      <div className="hold-foot">
        {page?.pull_request ? (
          <a href={page.pull_request.url} target="_blank" rel="noreferrer">
            <GitPullRequest size={13} aria-hidden /> #{page.pull_request.number} {page.pull_request.state.toLowerCase()} <ExternalLink size={11} aria-hidden />
          </a>
        ) : row?.branch ? (
          <span>No pull request yet</span>
        ) : null}
        {reviewer ? <span>Reviewed by {reviewer}</span> : row && row.status === "in_review" ? <span>No reviewer holds it yet</span> : null}
        <Link to={to.item(tie.repo, tie.id)} className="more">
          Open item <ChevronRight size={13} aria-hidden />
        </Link>
      </div>
    </section>
  );
}

function MissionHold({ tie, page }: { tie: Tie; page: MissionPage | null }) {
  const open = page?.items.filter((i) => !CLOSED.has(i.status)) ?? [];
  const counts = page?.mission.counts ?? [];
  return (
    <section className="hold-card">
      <header>
        <span className="glyph run">
          <Target size={15} aria-hidden />
        </span>
        <Link to={to.mission(tie.repo, tie.id)} className="hold-title">
          <span className="id">{tie.id}</span> {page?.mission.title ?? tie.title}
        </Link>
        <span className="tally" title={countsLine(counts)}>
          {finished(counts)}/{total(counts)} done
        </span>
      </header>
      <p className="hold-role">This thread {VERBS[tie.role][0]} this mission.</p>
      {page ? <ScopeBar counts={counts} tall /> : null}
      {open.length > 0 ? (
        <div className="list hold-items">
          {open.map((i) => (
            <Row
              key={i.id}
              repo={tie.repo}
              id={i.id}
              title={i.title}
              status={i.status}
              note={
                <>
                  <span className="stage">{statusName(i.status)}</span>
                  {i.holder ? <span className="who">{agentName(page?.threads ?? [], i.holder)}</span> : null}
                </>
              }
            />
          ))}
        </div>
      ) : null}
      <div className="hold-foot">
        <Link to={to.mission(tie.repo, tie.id)} className="more">
          Open mission <ChevronRight size={13} aria-hidden />
        </Link>
      </div>
    </section>
  );
}

function ChildRow({ id, info, ties, now }: { id: string; info: ThreadInfo | null; ties: Tie[]; now: number }) {
  const ended = info ? ENDED.has(info.status) : false;
  const changed = info?.updated_at ? Date.parse(info.updated_at) : NaN;
  const stale = !ended && info !== null && !Number.isNaN(changed) && now - changed > STALE_MS;
  const line = tieLine(ties);
  const age = since(info?.updated_at, now);
  return (
    <button type="button" className={`child-row${stale ? " stale" : ""}`} onClick={() => openThread(id)} title="Open this thread in T3 Code">
      <span className={`pulse ${info?.live ? info.status : ended ? "none" : (info?.status ?? "none")}`} aria-hidden />
      <span className="grow">
        <span className="t">{info ? info.title : <span className="mono">{who(null, id)}</span>}</span>
        <span className="s">
          <span className={info?.status === "waiting" ? "you-text" : undefined}>{info ? statusTitle(info.status as never) : "T3 Code does not know it"}</span>
          {info?.model ? <span className="mono">{info.model}</span> : null}
          {line ? <span>{line}</span> : null}
        </span>
      </span>
      <span className="note">
        {stale ? (
          <span className="tag bad">
            <Clock size={11} aria-hidden /> Quiet {age}
          </span>
        ) : age ? (
          <span className="age">{age}</span>
        ) : null}
      </span>
    </button>
  );
}

/** One thread on an item, as a chip that opens it in T3 Code. */
function ThreadChip({ agent, info }: { agent: Agent; info: ThreadInfo | undefined }) {
  const status = agent.thread?.status ?? info?.status ?? "unknown";
  const title = agent.thread?.title ?? info?.title ?? who(null, agent.identity);
  const model = agent.thread?.model ?? info?.model ?? null;
  return (
    <button type="button" className="chip" onClick={() => openThread(agent.identity)} title={`${title}\n${roleWords[agent.role]}, ${status}${model ? `, ${model}` : ""}`}>
      <span className={`pulse ${ENDED.has(status) ? "none" : status}`} aria-hidden />
      <span className="role">{agent.role === "implementer" ? "impl" : agent.role === "reviewer" ? "review" : agent.role}</span>
      <span className="st">{status.replaceAll("_", " ")}</span>
      {model ? <span className="mono">{model.replace(/^claude-|^gpt-/, "")}</span> : null}
    </button>
  );
}

/** A mission this thread planned or leads, with its items and the threads on each. */
function MissionWork({ tie, page, infos }: { tie: Tie; page: MissionPage | null; infos: Map<string, ThreadInfo> }) {
  const counts = page?.mission.counts ?? [];
  const items = page?.items ?? [];
  return (
    <section className="hold-card work-card">
      <header>
        <span className={`glyph ${page?.mission.status === "done" ? "ok" : page?.mission.in_progress ? "run" : "plain"}`}>
          <Target size={15} aria-hidden />
        </span>
        <Link to={to.control(tie.repo, tie.id)} className="hold-title" title="Open mission control">
          <span className="id">{tie.id}</span> {page?.mission.title ?? tie.title}
        </Link>
        <span className="tally" title={countsLine(counts)}>
          {finished(counts)}/{total(counts)} done
        </span>
      </header>
      <p className="hold-role">
        This thread {VERBS[tie.role][tie.current ? 0 : 1]} this mission{page?.mission.status === "done" ? ", which is done" : ""}.
      </p>
      {page ? <ScopeBar counts={counts} tall /> : <Skeleton rows={2} />}
      {items.length > 0 ? (
        <div className="list hold-items">
          {items.map((i) => {
            const agents = (page?.threads ?? []).filter((a) => a.item === i.id && (a.role === "implementer" || a.role === "reviewer"));
            return (
              <div key={i.id} className="work-item">
                <Link to={to.item(tie.repo, i.id)} className="glance-row">
                  <StatusGlyph status={i.status} />
                  <span className="id">{i.id}</span>
                  <span className="title">{i.title}</span>
                  <span className="note">
                    <span className="stage">{statusName(i.status)}</span>
                  </span>
                </Link>
                {agents.length > 0 ? (
                  <div className="chips">
                    {agents.map((a, at) => (
                      <ThreadChip key={`${a.identity}${at}`} agent={a} info={infos.get(a.identity)} />
                    ))}
                  </div>
                ) : null}
              </div>
            );
          })}
        </div>
      ) : null}
      <div className="hold-foot">
        <Link to={to.control(tie.repo, tie.id)} className="more">
          Mission control <ChevronRight size={13} aria-hidden />
        </Link>
      </div>
    </section>
  );
}

function MentionRow({ record }: { record: RecordName }) {
  const href = record.kind === "item" ? to.item(record.repo, record.id) : to.mission(record.repo, record.id);
  return (
    <Link to={href} className="glance-row">
      {record.kind === "mission" ? (
        <span className={`glyph ${record.status === "done" ? "ok" : "run"}`}>
          <Target size={15} aria-hidden />
        </span>
      ) : (
        <StatusGlyph status={record.status} />
      )}
      <span className="id">{record.id}</span>
      <span className="title">{record.title}</span>
      <span className="note">
        <span className="kind">{record.kind}</span>
        <span className="stage">{statusName(record.status)}</span>
      </span>
    </Link>
  );
}

function ThreadView({ context }: { context: HostContext }) {
  const self = useData<ThreadInfo>(api.thread(context.thread));
  const ties = useData<TiesResponse>(api.ties(context.thread));
  const inbox = useData<InboxResponse>(api.inbox());
  const all = ties.data?.ties ?? [];
  const current = all.filter((t) => t.current);
  const unique = (list: Tie[]) => list.filter((t, at) => list.findIndex((u) => u.repo === t.repo && u.id === t.id) === at);
  const itemTies = unique(current.filter((t) => t.kind === "item"));
  const missionTies = unique(current.filter((t) => t.kind === "mission"));
  const items = useAll<ItemPage>(itemTies.map((t) => api.item(t.repo, t.id)));
  const missions = useAll<MissionPage>(missionTies.map((t) => api.mission(t.repo, t.id)));
  // Missions this thread planned or leads, done or not.
  const workTies = unique(all.filter((t) => t.kind === "mission" && (t.role === "planner" || t.role === "lead")));
  const work = useAll<MissionPage>(workTies.map((t) => api.mission(t.repo, t.id)));
  const workAgents = work.data.flatMap((p) => (p?.threads ?? []).filter((a) => a.item !== null).map((a) => a.identity));
  const workInfos = useThreads(workAgents);
  const childInfo = useAll<ThreadInfo>(context.children.map((id) => api.thread(id)));
  const childTies = useAll<TiesResponse>(context.children.map(api.ties));
  const named = useAll<RecordName>(context.mentioned.map(api.record));
  const now = Date.now();
  const [showDone, setShowDone] = useState(false);

  // A read that fails leaves its part out, and the page still draws.
  const settled = (read: { data?: unknown; error?: string }) => read.data !== undefined || read.error !== undefined;
  if (!settled(ties) || !settled(inbox) || childTies.loading || named.loading) return <Skeleton rows={6} />;
  const repos = inbox.data?.repos ?? [];

  // A record this thread holds is shown under Holds, not again here.
  const held = new Set(current.map((t) => t.id));
  const mentioned = named.data.filter((r): r is RecordName => r !== null && !held.has(r.id));
  const openMentioned = mentioned.filter((r) => !CLOSED.has(r.status));
  const doneMentioned = mentioned.filter((r) => CLOSED.has(r.status));

  // The records this page is about: what this thread and its children hold,
  // the items of held missions, and what the conversation names.
  const records = new Set<string>([...current.map((t) => t.id), ...mentioned.map((r) => r.id)]);
  for (const t of childTies.data) for (const tie of t?.ties ?? []) if (tie.current) records.add(tie.id);
  for (const m of missions.data) for (const i of m?.items ?? []) records.add(i.id);
  for (const m of work.data) if (m && m.mission.status !== "done") for (const i of m.items) records.add(i.id);
  const threads = new Set([context.thread, ...context.children]);
  const waitingItems = repos.flatMap((r) =>
    r.needs_human.flatMap((g) => g.items.filter((i) => records.has(i.id) || (g.mission !== null && records.has(g.mission))).map((row) => ({ repo: r.repo, row }))),
  );
  const waitingThreads = repos.flatMap((r) => r.threads.filter((t) => threads.has(t.id) || t.holds.some((h) => records.has(h.id))));
  const waitingPlans = repos.flatMap((r) => r.proposed.filter((m) => records.has(m.id)).map((m) => ({ repo: r.repo, m })));
  const waits = waitingItems.length + waitingThreads.length + waitingPlans.length;

  const holds = itemTies.length + missionTies.length;
  // A child that no record names: the work above does not show it.
  const others = context.children.map((id, at) => ({ id, at })).filter(({ at }) => (childTies.data[at]?.ties ?? []).length === 0);
  // Without its ties the page cannot say the thread holds nothing.
  const empty = ties.data !== undefined && holds === 0 && workTies.length === 0 && others.length === 0 && mentioned.length === 0 && waits === 0;
  const earlier = tieLine(all.filter((t) => !t.current));

  return (
    <>
      <ErrorBanner loaded={ties} />
      <div className="thread-head">
        <b>{self.data?.title ?? "This thread"}</b>
        {earlier && holds === 0 ? <span>Earlier it {earlier}.</span> : null}
      </div>
      {empty ? (
        context.repository ? (
          <>
            <p className="thread-part">This thread holds nothing, started nothing and names no record. Here is {context.repository}.</p>
            <ProjectGlance repo={context.repository} />
          </>
        ) : (
          <p className="glance-empty">This thread holds nothing, started nothing and names no record.</p>
        )
      ) : (
        <div className="glance one">
          {waits > 0 ? (
            <Block title="Waiting on you" tally={waits}>
              {waitingPlans.map(({ repo, m }) => (
                <Link key={m.id} to={to.mission(repo, m.id)} className="glance-row hot">
                  <span className="glyph you">
                    <Target size={15} aria-hidden />
                  </span>
                  <span className="id">{m.id}</span>
                  <span className="title">{m.title}</span>
                  <span className="note">
                    <span className="tag you">Plan proposed</span>
                  </span>
                </Link>
              ))}
              {waitingThreads.map((t) => (
                <LiveThreadRow key={t.id} thread={t} holds={t.holds} />
              ))}
              {waitingItems.map(({ repo, row }) => (
                <Row key={row.id} repo={repo} id={row.id} title={row.title} status={row.status} note={<span className="tag you">Needs you</span>} />
              ))}
            </Block>
          ) : null}

          {holds > 0 ? (
            <section className="glance-block">
              <div className="glance-head">
                <h3>Holds</h3>
                <span className="tally">{holds}</span>
              </div>
              {itemTies.map((t, at) => (
                <ItemHold key={`${t.repo}/${t.id}`} tie={t} page={items.data[at] ?? null} />
              ))}
              {missionTies.map((t, at) => (
                <MissionHold key={`${t.repo}/${t.id}`} tie={t} page={missions.data[at] ?? null} />
              ))}
            </section>
          ) : null}

          {workTies.length > 0 ? (
            <section className="glance-block">
              <div className="glance-head">
                <h3>Work under this thread</h3>
                <span className="tally">{workTies.length}</span>
              </div>
              {workTies.map((t, at) => (
                <MissionWork key={`${t.repo}/${t.id}`} tie={t} page={work.data[at] ?? null} infos={workInfos} />
              ))}
            </section>
          ) : null}

          {others.length > 0 ? (
            <Block title="Other threads it started" tally={others.length}>
              {others.map(({ id, at }) => (
                <ChildRow key={id} id={id} info={childInfo.data[at] ?? null} ties={[]} now={now} />
              ))}
            </Block>
          ) : null}

          {mentioned.length > 0 ? (
            <Block title="Mentioned" tally={mentioned.length}>
              {openMentioned.map((r) => (
                <MentionRow key={r.id} record={r} />
              ))}
              {doneMentioned.length > 0 ? (
                <>
                  <div className="done-divider">
                    Done or dropped: {doneMentioned.length} of {mentioned.length}
                  </div>
                  {(showDone ? doneMentioned : doneMentioned.slice(0, 4)).map((r) => (
                    <div key={r.id} className="done-row">
                      <MentionRow record={r} />
                    </div>
                  ))}
                  {doneMentioned.length > 4 && !showDone ? (
                    <button type="button" className="fold" onClick={() => setShowDone(true)}>
                      Show {doneMentioned.length - 4} more done
                    </button>
                  ) : null}
                </>
              ) : null}
            </Block>
          ) : null}
        </div>
      )}
    </>
  );
}

/** `/t`: what the host's thread holds, started and names, and what of it waits on the human. */
export function ThisThreadPage() {
  const context = useHostContext();
  return (
    <div className="page glance-page">
      <Crumbs trail={[{ label: "This thread" }]} />
      {context ? <ThreadView key={context.thread} context={context} /> : <p className="glance-empty">Waiting for T3 Code to say which thread this is.</p>}
    </div>
  );
}
