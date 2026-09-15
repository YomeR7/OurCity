import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";
import { startWs } from "./lib/lib/websocket";

const app = mount(App, {
  target: document.getElementById("app")!,
});

/* Connect the websocket */
startWs();

export default app;
