import { writable, get, type Writable } from "svelte/store";
import type { EngineHandle } from "../renderer/renderer.js";
import init, { init_engine } from "../renderer/renderer.js";
import wasmUrl from "../renderer/renderer_bg.wasm?url";

/** Interface to the renderer wasm module */
export interface Renderer {
  handle: EngineHandle;
}

/** Current status of the wasm module. */
export type RendererStatus =
  | { status: "uninit" }
  | { status: "loading"; loaded: number; total: number }
  | { status: "running"; renderer: Renderer }
  | { status: "failed"; error: string };

/** Store for the wasm current status */
export const wasm_status: Writable<RendererStatus> = writable({
  status: "uninit",
});

/** Ask for the wasm module and update the wasm status store accordingly */
export async function loadRenderer(canvas_id: string) {
  // A winit event loop can only be created once per page, and HMR will
  // happily call this twice.
  const status: RendererStatus = get(wasm_status);
  if (status.status != "uninit") return;

  console.log("Loading renderer...");

  try {
    const response = await fetchWithProgress(wasmUrl);
    await init({ module_or_path: response });
    console.log("Loaded renderer, starting engine");
    const handle = init_engine(canvas_id);
    wasm_status.set({ status: "running", renderer: { handle } });
  } catch (e) {
    console.error("Failed to load renderer:", e);
    wasm_status.set({
      status: "failed",
      error: e instanceof Error ? e.message : String(e),
    });
  }
}

/** Fetches the module while reporting progress into the store. */
async function fetchWithProgress(url: string): Promise<Response> {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`server returned ${response.status}`);

  const total = Number(response.headers.get("content-length"));
  if (!total || !response.body) return response;

  const reader = response.body.getReader();
  let loaded = 0;

  /* Create a stream that updates the wasm status when loading */
  const stream = new ReadableStream({
    async pull(controller) {
      const { done, value } = await reader.read();
      if (done) return controller.close();
      loaded += value.byteLength;
      wasm_status.set({ status: "loading", loaded, total });
      controller.enqueue(value);
    },
    cancel: (reason) => reader.cancel(reason),
  });

  /* Return the stream as if it was the web stream being loaded */
  return new Response(stream, {
    headers: { "content-type": "application/wasm" },
  });
}
