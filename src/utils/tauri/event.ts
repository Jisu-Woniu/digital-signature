import { listen } from "@tauri-apps/api/event";
import { onScopeDispose } from "vue";

export const useTauriEvent = async <T>(...args: Parameters<typeof listen<T>>) =>
  onScopeDispose(await listen<T>(...args));
