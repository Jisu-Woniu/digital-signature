<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { VBtn, VCard, VDialog, VIcon } from "vuetify/components";
import { TauriEvent, type UnlistenFn, listen } from "@tauri-apps/api/event";
import { message, open } from "@tauri-apps/plugin-dialog";
import { useVModel } from "@vueuse/core";
import { FileType, detectFileType } from "@/command";
import FolderOpen from "~icons/ic/twotone-folder-open";
import Reject from "~icons/ic/twotone-highlight-off";
import UploadFile from "~icons/ic/twotone-upload-file";

const props = defineProps<{
  modelValue: string | undefined;
  directory?: boolean;
}>();

const emits = defineEmits<{
  (event: "update:modelValue", value: string | undefined): void;
}>();

const file = useVModel(props, "modelValue", emits);

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

const checkFileType = async (path: string) =>
  (await detectFileType(path)) ===
  (props.directory ? FileType.dir : FileType.file);

onMounted(async () => {
  listeners = await Promise.all([
    listen<string[]>(TauriEvent.WINDOW_FILE_DROP_HOVER, async (e) => {
      hover.value = true;
      hover_accept.value =
        e.payload.length == 1 && (await checkFileType(e.payload[0]));
    }),
    listen<string[]>(TauriEvent.WINDOW_FILE_DROP, async (e) => {
      if (e.payload.length != 1 || !(await checkFileType(e.payload[0])))
        await message(props.directory ? "只支持文件夹" : "只支持文件");
      else file.value = e.payload[0];
      hover.value = hover_accept.value = false;
    }),
    listen<void>(TauriEvent.WINDOW_FILE_DROP_CANCELLED, () => {
      hover.value = hover_accept.value = false;
    }),
    listen<void>(TauriEvent.WINDOW_BLUR, () => {
      hover.value = hover_accept.value = false;
    }),
  ]);
});

onUnmounted(() => {
  listeners.forEach((unlisten) => unlisten());
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
