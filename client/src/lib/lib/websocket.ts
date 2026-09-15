import { get } from "svelte/store";
import { renderer } from "./renderer";

/** Delay before reconnecting, doubling per attempt up to a cap. */
const RECONNECT_MIN_MS = 500;
const RECONNECT_MAX_MS = 10_000;

let socket: WebSocket | null = null;
let reconnectDelay = RECONNECT_MIN_MS;

/* Session identity: regenerated per connection. Everything sent on this
 * socket is attributed to this id; the server excludes same-id ops from
 * transformation, which is what makes bursting safe. */
let clientId: string = crypto.randomUUID();
export function getClientId(): string {
  return clientId;
}

/** Open the websocket and keep it alive for the lifetime of the app.
 * Call once at startup. */
export function startWs() {
  connect();
}

function connect() {
  const proto = location.protocol === "https:" ? "wss:" : "ws:";
  socket = new WebSocket(`${proto}//${location.host}/ws`);

  socket.onopen = async () => {
    clientId = crypto.randomUUID();
    reconnectDelay = RECONNECT_MIN_MS;
  };

  socket.onmessage = (message: MessageEvent) => {
    const engine = get(renderer);
    if (engine.status != "running") {
      console.warn("Unable to handle websocket message: engine not running!");
      return;
    }

    const handle = engine.renderer.handle;
    const event = JSON.parse(message.data);
    handle.server_event(event);
  };

  socket.onclose = () => {
    socket = null;
    console.warn(`ws closed, reconnecting in ${reconnectDelay}ms`);
    setTimeout(connect, reconnectDelay);
    reconnectDelay = Math.min(reconnectDelay * 2, RECONNECT_MAX_MS);
  };

  socket.onerror = (err) => {
    console.warn("Web socket error: ", err);
  };
}
