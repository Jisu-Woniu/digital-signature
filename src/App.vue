<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import FileSelector from "@/components/FileSelector.vue";
import { TauriEvent, listen, type UnlistenFn } from "@tauri-apps/api/event";

const onUpdate = (e: Event) => {
  console.log((e.target as HTMLInputElement).value);
};

const files = ref<string[]>([]);
const hover = ref(false);

const onFilesSelected = (files_selected: string[]) => {
  console.log(files_selected);
  files.value = files_selected;
};

let unlistenFileDrop: UnlistenFn | undefined;
let unlistenFileDropHover: UnlistenFn | undefined;
let unlistenFileDropCanceled: UnlistenFn | undefined;

onMounted(async () => {
  unlistenFileDrop = await listen<string[]>(
    TauriEvent.WINDOW_FILE_DROP,
    (e) => {
      files.value = e.payload;
      hover.value = false;
    },
  );
  unlistenFileDropHover = await listen<string[]>(
    TauriEvent.WINDOW_FILE_DROP_HOVER,
    (e) => {
      console.log(e.payload);
      hover.value = true;
    },
  );
  unlistenFileDropCanceled = await listen<void>(
    TauriEvent.WINDOW_FILE_DROP_CANCELLED,
    (e) => {
      console.log(e.payload);
      hover.value = false;
    },
  );
});

onUnmounted(() => {
  [unlistenFileDrop, unlistenFileDropHover, unlistenFileDropCanceled].forEach(
    (unlisten) => {
      if (unlisten) {
        unlisten();
      }
    },
  );
});
</script>

<template>
  <FileSelector @file-selected="onFilesSelected" />
  <ul>
    <li v-for="file in files" :key="file">
      {{ file }}
    </li>
  </ul>
  <div v-if="hover" class="red">Hovering</div>
</template>

<style scoped>
.red {
  color: red;
}
</style>
