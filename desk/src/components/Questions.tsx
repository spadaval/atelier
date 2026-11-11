import { MessageCircleQuestion } from "lucide-react";
import { Link } from "react-router";
import type { Question } from "../api/Question";
import { to } from "../lib/api";
import { AnswerForm } from "./Acts";

/** An open question, linking to its item, with the form to answer it when it is its item's oldest. */
export function QuestionLine({ repo, question, answerable, onAnswered }: { repo: string; question: Pick<Question, "item" | "question" | "finding">; answerable: boolean; onAnswered?: (text: string) => void }) {
  return (
    <div className="qline" data-testid="question">
      <span className="glyph you">
        <MessageCircleQuestion size={15} aria-hidden />
      </span>
      <span className="grow">
        <Link to={to.item(repo, question.item)} className="q">
          {question.question}
        </Link>
        <span className="sub">
          <span className="mono">{question.item}</span>
          {question.finding ? <> · about finding {question.finding}</> : null}
        </span>
        {answerable ? <AnswerForm repo={repo} item={question.item} finding={question.finding} onAnswered={onAnswered} /> : null}
      </span>
    </div>
  );
}

/** An answer the Desk gave that no commit holds yet. */
export function PendingMark() {
  return (
    <span className="tag you" data-testid="pending">
      Answered from the Desk, not committed yet
    </span>
  );
}
