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
  modelValue: string[];
  directory?: boolean;
}>();

const emits = defineEmits<{
  (event: "update:modelValue", value: string[]): void;
}>();

const files = useVModel(props, "modelValue", emits);

const selectFiles = async () => {
  const selected = (await open({
    multiple: true,
    directory: props.directory,
  })) as string[];
  files.value = selected ?? [];
};

const hover = ref(false);
const hover_accept = ref(false);

let listeners: UnlistenFn[];

const checkFilesType = async (paths: string[]) => {
  const expectedType = props.directory ? FileType.dir : FileType.file;
  return (await Promise.all(paths.map(detectFileType))).every(
    (fileType) => fileType === expectedType,
  );
};

onMounted(async () => {
  listeners = await Promise.all([
    listen<string[]>(TauriEvent.WINDOW_FILE_DROP_HOVER, async (e) => {
      hover.value = true;
      const payload = e.payload;
      hover_accept.value =
        payload.length != 0 && (await checkFilesType(payload));
    }),
    listen<string[]>(TauriEvent.WINDOW_FILE_DROP, async (e) => {
      console.info("DROPPED!!!");
      const payload = e.payload;
      if (payload.length == 0 || !(await checkFilesType(payload)))
        await message(props.directory ? "只支持文件夹" : "只支持文件");
      else files.value = payload;
      hover_accept.value = true;
      hover.value = false;
    }),
    listen<void>(TauriEvent.WINDOW_FILE_DROP_CANCELLED, () => {
      hover_accept.value = true;
      hover.value = false;
    }),
    listen<void>(TauriEvent.WINDOW_BLUR, () => {
      hover_accept.value = true;
      hover.value = false;
    }),
  ]);
});

onUnmounted(() => {
  listeners.forEach((unlisten) => unlisten());
  listeners = [];
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
