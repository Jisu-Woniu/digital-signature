<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { message, open } from "@tauri-apps/api/dialog";
import { listen, TauriEvent, type UnlistenFn } from "@tauri-apps/api/event";

import { VBtn, VCard, VDialog, VIcon } from "vuetify/components";

import UploadFile from "~icons/ic/twotone-upload-file";
import FolderOpen from "~icons/ic/twotone-folder-open";
import Reject from "~icons/ic/twotone-highlight-off";

import { useVModel } from "@vueuse/core";
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps<{
  modelValue: string | undefined;
  directory?: boolean;
}>();

const emits = defineEmits<{
  (event: "update:modelValue", value: string | undefined): void;
}>();

const file = useVModel(props, "modelValue", emits);

const SelectFile = async () => {
  const selected = (await open({
    multiple: false,
    directory: props.directory,
  })) as string | null;
  file.value = selected ?? undefined;
};

const hover = ref(false);
const hover_accept = ref(false);

let listeners: UnlistenFn[];

const enum FileType {
  file = 0,
  dir = 1,
  other = 2,
  inexist = 3,
}

const checkFileType = async (path: string) => {
  const fileType = await invoke<FileType>("file_type", { path });
  return fileType === (props.directory ? FileType.dir : FileType.file);
};

onMounted(async () => {
  listeners = await Promise.all([
    listen<string[]>(TauriEvent.WINDOW_FILE_DROP_HOVER, async (e) => {
      hover.value = true;
      hover_accept.value =
        e.payload.length == 1 && (await checkFileType(e.payload[0]));
    }),
    listen<string[]>(TauriEvent.WINDOW_FILE_DROP, async (e) => {
      console.log("DROP!!");
      if (e.payload.length != 1 || !(await checkFileType(e.payload[0]))) {
        await message(
          props.directory ? "请拖放一个文件夹到此处" : "请拖放一个文件到此处",
        );
      } else file.value = e.payload[0];
      hover.value = false;
    }),
    listen<void>(TauriEvent.WINDOW_FILE_DROP_CANCELLED, () => {
      hover.value = false;
    }),
    listen<void>(TauriEvent.WINDOW_BLUR, () => {
      hover.value = false;
    }),
  ]);
});

onUnmounted(() => listeners.forEach((unlisten) => unlisten()));
</script>
<template>
  <VBtn :prepend-icon="FolderOpen" @click="SelectFile"> 选择文件 </VBtn>
  <VDialog v-model="hover" height="100%">
    <VCard v-if="hover_accept" class="hover-indication-box">
      <VIcon :icon="UploadFile" class="hover-indication-icon" />
      <div class="hover-indication-text text-blue-darken-3 mt-3">
        拖拽至此处
      </div>
    </VCard>
    <VCard v-else class="hover-indication-box">
      <VIcon :icon="Reject" class="hover-indication-icon" />
      <div class="hover-indication-text text-blue-darken-3 mt-3">请</div>
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
