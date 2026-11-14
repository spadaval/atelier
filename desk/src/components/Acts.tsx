import { CircleCheck, MessagesSquare, OctagonX, TriangleAlert } from "lucide-react";
import { useId, useState, type ReactNode } from "react";
import type { ActResponse } from "../api/ActResponse";
import type { MissionPage } from "../api/MissionPage";
import type { SessionResponse } from "../api/SessionResponse";
import { act, sessionRequest, startSession, stopRequest, succeeded, type Sent } from "../lib/acts";
import { reloadAll } from "../lib/api";
import { shortCommit } from "../lib/words";
import { ThreadTitle } from "./Agents";

/** A refusal as the server gave it: its text, the rules it names and its next step. */
function Refusal({ title, text, rules = [], next = null }: { title: string; text: string; rules?: string[]; next?: string | null }) {
  return (
    <div className="act-result bad" role="alert" data-testid="refusal">
      <TriangleAlert size={15} aria-hidden />
      <div className="grow">
        <b>{title}</b>
        <p className="act-text">{text}</p>
        {rules.length > 0 ? (
          <ul className="act-rules">
            {rules.map((rule, at) => (
              <li key={at}>{rule}</li>
            ))}
          </ul>
        ) : null}
        {next ? (
          <p className="act-next">
            <span className="muted">Next: </span>
            {next}
          </p>
        ) : null}
      </div>
    </div>
  );
}

function Failed({ error }: { error: string }) {
  return (
    <div className="act-result bad" role="alert" data-testid="failure">
      <TriangleAlert size={15} aria-hidden />
      <p className="grow">It did not run. {error}</p>
    </div>
  );
}

/** What an act answered: done, refused, or no answer at all. */
function ActOutcome({ sent, refusedTitle, done }: { sent: Sent<ActResponse>; refusedTitle: string; done: (text: string, commit: string | null) => ReactNode }) {
  if (sent.state === "failed") return <Failed error={sent.error} />;
  if (sent.state !== "answered") return null;
  const r = sent.response;
  if (r.result === "refused") return <Refusal title={refusedTitle} text={r.text} rules={r.rules} next={r.next} />;
  return (
    <div className="act-result ok" role="status" data-testid="done">
      <CircleCheck size={15} aria-hidden />
      <div className="grow">{done(r.text, r.commit)}</div>
    </div>
  );
}

/** Asks once before an act runs. */
function Confirm({ label, children, yes, onYes, onNo }: { label: string; children: ReactNode; yes: string; onYes: () => void; onNo: () => void }) {
  return (
    <div className="confirm" role="group" aria-label={label}>
      <p>{children}</p>
      <div className="row">
        <button type="button" className="btn primary" autoFocus onClick={onYes}>
          {yes}
        </button>
        <button type="button" className="btn ghost" onClick={onNo}>
          Not yet
        </button>
      </div>
    </div>
  );
}

/**
 * Stop a mission. A proposed plan is stopped here: the Desk writes the
 * drop with his reason, commits it, pushes the plan's branch and closes its
 * pull request, and every page is read again. An open mission's stop is
 * published by a record-only branch that merges, which the Desk does not
 * make yet (atelier-yclc), so its page gives the commands; a stop someone
 * has written shows as pending until that branch merges.
 */
export function StopControl({ repo, page }: { repo: string; page: MissionPage }) {
  const [sent, setSent] = useState<Sent<ActResponse>>({ state: "idle" });
  const [reason, setReason] = useState("");
  const id = useId();
  const m = page.mission;
  const stopped = sent.state === "answered" && sent.response.result === "done";
  if (m.state === "stop_pending") {
    return (
      <div className="go-note" data-testid="stop">
        <p>
          <b>A stop is written and not published yet.</b>{" "}
          {page.stop_on
            ? `It is on ${page.stop_on}, and counts once that branch merges.`
            : page.pull_request
              ? `It is pushed; it counts once pull request #${page.pull_request.number} is closed.`
              : "It counts once it is pushed."}
        </p>
      </div>
    );
  }
  if (m.state === "open") {
    return (
      <details className="stop-how" data-testid="stop">
        <summary>How to stop this mission</summary>
        <p className="muted">It is agreed, so its stop reaches the base by a branch that merges. The Desk does not make that branch yet; in a checkout of {repo}, run:</p>
        <pre className="commands">{page.stop_commands.join("\n")}</pre>
        <p className="muted">From the push of that branch this page shows the stop as pending, and once it merges, as stopped.</p>
      </details>
    );
  }
  if (m.state !== "proposed" && !stopped) return null;
  const send = async () => {
    setSent({ state: "sending" });
    const result = await act(stopRequest(repo, m.id, reason.trim()));
    if (succeeded(result)) await reloadAll();
    setSent(result);
  };
  const sending = sent.state === "sending";
  return (
    <div className="go-note" data-testid="stop">
      {!stopped ? (
        <p>
          <b>This plan is proposed.</b> It merges once two reviewers approve it, and its items start after that. Stop it if it should not go ahead.
          {!page.can_stop ? " The Desk cannot publish a stop in this repository, so stop it from its planner's checkout with atelier drop." : null}
        </p>
      ) : null}
      {page.can_stop && (sent.state === "idle" || sent.state === "failed" || (sent.state === "answered" && !stopped)) ? (
        <div className="row act-bar">
          <button type="button" className="btn" onClick={() => setSent({ state: "confirming" })}>
            <OctagonX size={14} aria-hidden /> Stop this plan
          </button>
        </div>
      ) : null}
      {sent.state === "confirming" || sending ? (
        <form
          className="answer-form"
          aria-label="Stop this plan"
          onSubmit={(event) => {
            event.preventDefault();
            if (reason.trim() !== "" && !sending) void send();
          }}
        >
          <label htmlFor={`${id}-reason`}>Why, in one line. It is written into the mission's Result.</label>
          <input id={`${id}-reason`} className="field" type="text" autoFocus value={reason} disabled={sending} onChange={(event) => setReason(event.target.value)} />
          <p className="muted">The Desk drops the mission on its branch, pushes it and closes its pull request. Its planner and reviewers are refused from then on.</p>
          <div className="row">
            <button type="submit" className="btn primary" disabled={reason.trim() === "" || sending}>
              {sending ? "Stopping…" : "Yes, stop it"}
            </button>
            <button type="button" className="btn ghost" disabled={sending} onClick={() => setSent({ state: "idle" })}>
              Not yet
            </button>
          </div>
        </form>
      ) : null}
      <ActOutcome
        sent={sent}
        refusedTitle="The stop was refused."
        done={(text, commit) => (
          <>
            <b>You stopped this plan{commit ? `, committed as ${shortCommit(commit)}` : ""}.</b>
            <p className="act-text">{text}</p>
          </>
        )}
      />
    </div>
  );
}

/** Starts a planning session for a mission as a new T3 Code thread, then links to it. */
export function PlanningSession({ repo, mission }: { repo: string; mission: string }) {
  const [sent, setSent] = useState<Sent<SessionResponse>>({ state: "idle" });
  const start = async () => {
    setSent({ state: "sending" });
    setSent(await startSession(sessionRequest(repo, mission)));
  };
  const response = sent.state === "answered" ? sent.response : null;
  return (
    <div className="session" data-testid="session">
      {sent.state === "idle" || sent.state === "failed" || response?.result === "refused" ? (
        <button type="button" className="btn" onClick={() => setSent({ state: "confirming" })}>
          <MessagesSquare size={14} aria-hidden /> Start a planning session
        </button>
      ) : null}
      {sent.state === "confirming" ? (
        <Confirm label="Confirm starting a planning session" yes="Yes, start it" onYes={() => void start()} onNo={() => setSent({ state: "idle" })}>
          Start a new thread in T3 Code to plan this mission with you? It changes no record.
        </Confirm>
      ) : null}
      {sent.state === "sending" ? (
        <p className="muted" role="status">
          Starting a thread in T3 Code…
        </p>
      ) : null}
      {sent.state === "failed" ? <Failed error={sent.error} /> : null}
      {response?.result === "refused" ? <Refusal title="T3 Code did not start a session." text={response.text} /> : null}
      {response?.result === "started" ? (
        <div className="act-result ok" role="status" data-testid="started">
          <CircleCheck size={15} aria-hidden />
          <div className="grow">
            <b>A planning session started in T3 Code.</b>{" "}
            {response.link ? (
              <ThreadTitle thread={{ title: "Open the thread", link: response.link }} />
            ) : (
              <span>T3 Code gave no address for it; find the thread named for this mission in T3 Code.</span>
            )}
          </div>
        </div>
      ) : null}
    </div>
  );
}
