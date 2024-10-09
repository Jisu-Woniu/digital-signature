<script setup lang="ts">
import { onMounted, ref } from "vue";
import { VBtn, VCard, VDialog, VIcon } from "vuetify/components";
import { TauriEvent } from "@tauri-apps/api/event";
import { useTauriEvent } from "../utils";
import { message, open } from "@tauri-apps/plugin-dialog";
import { FileType, detectFileType } from "@/command";
import FolderOpen from "~icons/ic/twotone-folder-open";
import Reject from "~icons/ic/twotone-highlight-off";
import UploadFile from "~icons/ic/twotone-upload-file";

const props = defineProps<{ directory?: boolean }>();

const files = defineModel<string[]>({ default: [] });

const selectFiles = async () => {
  const selected = (await open({
    multiple: true,
    directory: props.directory,
  })) as string[];
  files.value = selected ?? [];
};

const hover = ref(false);
const hover_accept = ref(false);

const checkFilesType = async (paths: string[]) => {
  const expectedType = props.directory ? FileType.dir : FileType.file;
  return (
    paths.length !== 0 &&
    (await Promise.all(paths.map(detectFileType))).every(
      (fileType) => fileType === expectedType,
    )
  );
};

onMounted(() => {
  Promise.all([
    useTauriEvent<{ paths: string[] }>(TauriEvent.DRAG_ENTER, async (e) => {
      hover.value = true;
      console.log("DRAG_ENTER", e.payload);
      hover_accept.value = await checkFilesType(e.payload.paths);
    }),
    useTauriEvent<{ paths: string[] }>(TauriEvent.DRAG_DROP, async (e) => {
      console.log("DRAG_DROP", e.payload);
      if (!(await checkFilesType(e.payload.paths)))
        await message(props.directory ? "只支持文件夹" : "只支持文件");
      else files.value = e.payload.paths;
      hover_accept.value = hover.value = false;
    }),
    useTauriEvent(TauriEvent.DRAG_LEAVE, async (e) => {
      console.log("DRAG_LEAVE", e.payload);
      hover.value = hover_accept.value = false;
    }),
    useTauriEvent(TauriEvent.WINDOW_BLUR, async (e) => {
      console.log("WINDOW_BLUR", e.payload);
      hover.value = hover_accept.value = false;
    }),
  ]);
});
</script>
<template>
  <slot :select-files="selectFiles">
    <VBtn :prepend-icon="FolderOpen" @click="selectFiles"> 选择多个文件 </VBtn>
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
