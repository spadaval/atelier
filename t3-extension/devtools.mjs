export async function connectPage(port = 9222, prefix = 't3code://app/') {
  const targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
  const target = targets.find(item => item.type === 'page' && item.url.startsWith(prefix));
  if (!target) throw new Error(`No page matching ${prefix} at debugging port ${port}`);
  const socket = new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((resolve, reject) => { socket.addEventListener('open', resolve, { once: true }); socket.addEventListener('error', reject, { once: true }); });
  const pending = new Map(); let id = 0;
  socket.addEventListener('message', event => {
    const message = JSON.parse(event.data); const entry = pending.get(message.id);
    if (!entry) return;
    clearTimeout(entry.timer); pending.delete(message.id);
    message.error ? entry.reject(new Error(message.error.message)) : entry.resolve(message.result);
  });
  function call(method, params = {}, sessionId, timeout = 15000) {
    const requestId = ++id;
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => { pending.delete(requestId); reject(new Error(`${method} timed out after ${timeout} ms`)); }, timeout);
      pending.set(requestId, { resolve, reject, timer });
      socket.send(JSON.stringify({ id: requestId, method, params, sessionId }));
    });
  }
  return { target, call, async evaluate(expression, timeout) {
    const result = await call('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true }, undefined, timeout);
    if (result.exceptionDetails) throw new Error(result.exceptionDetails.exception?.description ?? result.exceptionDetails.text);
    return result.result.value;
  }, close() { socket.close(); } };
}
