// Spike: `GET /api/thread?id=<identity>`.
export type ThreadInfo = { id: string, title: string, status: string, updated_at: string, parent: string | null, model: string | null, live: boolean, };
