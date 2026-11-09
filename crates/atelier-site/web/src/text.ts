// Small text helpers shared by the pages, matching how the CLI words the
// same views.

/** A commit's first 8 characters, as the CLI shows it. */
export function shortCommit(commit: string): string {
  return commit.slice(0, 8);
}

function pad(n: number): string {
  return String(n).padStart(2, "0");
}

/** `YYYY-MM-DD HH:MM` in the reader's time zone. */
export function localTime(time: string): string {
  const d = new Date(time);
  if (Number.isNaN(d.getTime())) return time;
  return `${localDate(time)} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

/** `YYYY-MM-DD` in the reader's time zone. */
export function localDate(time: string): string {
  const d = new Date(time);
  if (Number.isNaN(d.getTime())) return time;
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

/** Text on one line, runs of whitespace as one space. */
export function flat(text: string): string {
  return text.split(/\s+/).filter(Boolean).join(" ");
}

/** The first sentence, at most 240 characters. */
export function firstSentence(text: string): string {
  const one = flat(text);
  const at = one.indexOf(". ");
  const sentence = at < 0 ? one : one.slice(0, at + 1);
  const chars = Array.from(sentence);
  return chars.length > 240 ? `${chars.slice(0, 239).join("")}…` : sentence;
}

/** `1 item`, `2 items`. */
export function countNoun(count: number, noun: string): string {
  return `${count} ${noun}${count === 1 ? "" : "s"}`;
}
