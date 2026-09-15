import type { EngineHandle } from "../renderer/renderer";
import { pushNotification } from "./notifications";

/** Ask the server for the city to load and load it into the engine */
export async function loadCity(engine: EngineHandle) {
  const url = "/api/get_our_city";
  const response = await fetch(url);

  if (response.ok) {
    /* Fixme: would be sick to load buildings as they come */
    const city = await response.json();
    engine.load_city(city);
    pushNotification("Welcome to Our City!", "cyan");
  } else {
    console.error("Failed to get city: ", response.status);
    pushNotification(
      `Failed to load city: Server returned status code ${response.status}`,
      "red",
    );
  }
}
