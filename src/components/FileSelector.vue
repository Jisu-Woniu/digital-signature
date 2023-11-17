<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { message, open } from "@tauri-apps/api/dialog";
import { listen, TauriEvent, type UnlistenFn } from "@tauri-apps/api/event";

import { VBtn, VCard, VDialog, VIcon } from "vuetify/components";

import UploadFile from "~icons/ic/twotone-upload-file";
import FolderOpen from "~icons/ic/twotone-folder-open";

import { useVModel } from "@vueuse/core";

const props = defineProps<{ modelValue: string | undefined }>();

const emits = defineEmits<{
  (event: "update:modelValue", value: string | undefined): void;
}>();

const files = useVModel(props, "modelValue", emits);

const SelectFile = async () => {
  const selected = (await open({ multiple: false })) as string | null;
  files.value = selected ?? undefined;
};

const hover = ref(false);

let listeners: UnlistenFn[];

onMounted(async () => {
  listeners = await Promise.all([
    listen<string[]>(TauriEvent.WINDOW_FILE_DROP_HOVER, () => {
      hover.value = true;
    }),
    listen<string[]>(TauriEvent.WINDOW_FILE_DROP, async (e) => {
      console.log(e.payload);
      if (e.payload.length != 1) {
        await message(
          "You can only drop one file here.\n" +
            "You have selected:\n" +
            e.payload.join("\n"),
        );
      } else files.value = e.payload[0];
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

onUnmounted(() => {
  listeners.forEach((unlisten) => unlisten());
});
</script>
<template>
  <VBtn :prepend-icon="FolderOpen" @click="SelectFile"> 选择文件 </VBtn>
  <VDialog v-model="hover" height="100%">
    <VCard class="hover-indication-box">
      <VIcon :icon="UploadFile" class="hover-indication-icon" />
      <div class="hover-indication-text text-blue-darken-3 mt-3">
        拖拽至此处
      </div>
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
