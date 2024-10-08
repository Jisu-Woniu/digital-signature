<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { VBtn, VCard, VDialog, VIcon } from "vuetify/components";
import { TauriEvent, type UnlistenFn, listen } from "@tauri-apps/api/event";
import { message, open } from "@tauri-apps/plugin-dialog";
import { FileType, detectFileType } from "@/command";
import FolderOpen from "~icons/ic/twotone-folder-open";
import Reject from "~icons/ic/twotone-highlight-off";
import UploadFile from "~icons/ic/twotone-upload-file";

const props = defineProps<{ directory?: boolean }>();

const file = defineModel<string>();

const selectFile = async () => {
  const selected = (await open({
    multiple: false,
    directory: props.directory,
  })) as string | null;
  file.value = selected ?? undefined;
};

const hover = ref(false);
const hover_accept = ref(false);

let listeners: UnlistenFn[];

const checkFileType = async (paths: string[]) => {
  return (
    paths.length === 1 &&
    (await detectFileType(paths[0])) ===
      (props.directory ? FileType.dir : FileType.file)
  );
};

onMounted(async () => {
  listeners = await Promise.all([
    listen<{ paths: string[] }>(TauriEvent.DRAG_ENTER, async (e) => {
      hover.value = true;
      console.log("DRAG_ENTER", e.payload);
      hover_accept.value = await checkFileType(e.payload.paths);
    }),
    listen<{ paths: string[] }>(TauriEvent.DRAG_DROP, async (e) => {
      console.log("DRAG_DROP", e.payload);
      if (!(await checkFileType(e.payload.paths)))
        await message(props.directory ? "请选择一个文件夹" : "请选择一个文件");
      else file.value = e.payload.paths[0];
      hover.value = hover_accept.value = false;
    }),
    listen<unknown>(TauriEvent.DRAG_LEAVE, async (e) => {
      console.log("DRAG_LEAVE", e.payload);
      hover.value = hover_accept.value = false;
    }),
    listen<void>(TauriEvent.WINDOW_BLUR, async (e) => {
      console.log("WINDOW_BLUR", e.payload);
      hover.value = hover_accept.value = false;
    }),
  ]);
});

onUnmounted(() => {
  for (const unlistenFn of listeners) unlistenFn();

  listeners = [];
});
</script>
<template>
  <slot :select-file="selectFile">
    <VBtn :prepend-icon="FolderOpen" @click="selectFile"> 选择文件 </VBtn>
  </slot>
  <VDialog v-model="hover" height="100%">
    <VCard v-if="hover_accept" class="hover-indication-box">
      <VIcon :icon="UploadFile" class="hover-indication-icon" />
      <div class="hover-indication-text text-blue-darken-3 mt-3">
        拖拽至此处
      </div>
    </VCard>
    <VCard v-else class="hover-indication-box">
      <VIcon :icon="Reject" class="hover-indication-icon" />
    </VCard>
  </VDialog>
</template>
<style scoped>
.hover-indication-box {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;
}

.hover-indication-icon {
  font-size: 5em;
}

.hover-indication-text {
  font-size: 2em;
}
</style>
