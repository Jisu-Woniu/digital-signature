import {
  listen,
  type EventCallback,
  type EventName,
  type Options,
} from "@tauri-apps/api/event";
import { tryOnScopeDispose } from "@vueuse/core";

export async function useTauriEvent<T>(
  e: EventName,
  handler: EventCallback<T>,
  options?: Options,
) {
  const unlisten = await listen(e, handler, options);

  tryOnScopeDispose(unlisten);
}
