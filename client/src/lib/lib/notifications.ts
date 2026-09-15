import { writable } from "svelte/store";

export interface Notification {
  id: number;
  text: string;
  color: string;
}

export const notifications = writable<Notification[]>([]);

let nextId = 0;

export function pushNotification(
  text: string,
  color = "white",
  duration = 10_000,
) {
  const id = nextId++;
  notifications.update((entries) => [...entries, { id, text, color }]);
  setTimeout(() => {
    notifications.update((entries) => entries.filter((e) => e.id !== id));
  }, duration);
}
