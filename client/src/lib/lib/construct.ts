import { get, writable, type Writable } from "svelte/store";
import { renderer } from "./renderer";
import { pushNotification } from "./notifications";
import { constructRequest } from "./api";

/** Current status of the client building a new construct. */
export type ConstructStatus =
  { status: "idle" } | { status: "hover"; building: string };

/** Store for the construct current status */
export const construct: Writable<ConstructStatus> = writable({
  status: "idle",
});

construct.subscribe((val) => {
  const renderer_status = get(renderer);
  if (renderer_status.status != "running") {
    console.warn("Can't change construct status: engine not running!");
    return;
  }

  const engine_handle = renderer_status.renderer.handle;

  if (val.status == "idle") {
    engine_handle.stop_construct_placement();
  } else if (val.status == "hover") {
    engine_handle.start_construct_placement(val.building);
  }
});

export async function sendConstructRequest() {
  const renderer_status = get(renderer);
  if (renderer_status.status != "running") {
    console.warn("Can't ask for construct: engine not running!");
    return {
      ok: false,
      error: "Can't ask for construct: engine not running!",
    };
  }

  const construct_status = get(construct);
  if (construct_status.status != "hover") {
    console.warn("Can't ask for construct: invalid state!");
    return {
      ok: false,
      error: "Can't ask for construct: invalid state!",
    };
  }

  const engine_handle = renderer_status.renderer.handle;
  const placement = await engine_handle.ask_construct_position();

  if (!placement) {
    console.warn("Can't ask for construct: engine is not in placement mode!");
    return {
      ok: false,
      error: "Can't ask for construct: engine is not in placement mode!",
    };
  }

  const response = await constructRequest(
    construct_status.building,
    placement.x,
    placement.y,
    placement.width,
    placement.height,
  );
  if (response.ok) {
    pushNotification("You asked to start a new building!");
    return { ok: true };
  } else {
    return { ok: false, error: await response.text() };
  }
}
