<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { message, open } from "@tauri-apps/api/dialog";
import { listen, TauriEvent, type UnlistenFn } from "@tauri-apps/api/event";

import { VBtn, VIcon } from "vuetify/components";
import { mdiFolderOpen } from "@mdi/js";

const props = defineProps<{ modelValue: string | undefined }>();

const emits = defineEmits<{
  (event: "update:modelValue", value: string | undefined): void;
}>();

const files = computed<string | undefined>({
  get: () => props.modelValue,
  set: (value) => {
    emits("update:modelValue", value);
  }
});

const SelectFile = async () => {
  const selected = (await open({ multiple: false })) as string | null;
  files.value = selected ?? undefined;
};

const hover = ref(0);

let unlistenFileDrop: UnlistenFn | undefined;
let unlistenFileDropHover: UnlistenFn | undefined;
let unlistenFileDropCanceled: UnlistenFn | undefined;
let unlistenBlur: UnlistenFn | undefined;

onMounted(async () => {
  unlistenFileDrop = await listen<string[]>(
    TauriEvent.WINDOW_FILE_DROP,
    async (e) => {
      console.log(e.payload);
      if (e.payload.length > 1) {
        await message(
          `You can only drop one file here.\nYou have selected: ${JSON.stringify(e.payload)}`
        );
        return;
      }
      files.value = e.payload[0];
      hover.value = 0;
    }
  );
  unlistenFileDropHover = await listen<string[]>(
    TauriEvent.WINDOW_FILE_DROP_HOVER,
    (e) => {
      console.log(e.payload);
      hover.value = e.payload.length;
    }
  );
  unlistenFileDropCanceled = await listen<void>(
    TauriEvent.WINDOW_FILE_DROP_CANCELLED,
    () => {
      hover.value = 0;
    }
  );
  unlistenBlur = await listen<void>(TauriEvent.WINDOW_BLUR, () => {
    console.log("Blur");
    hover.value = 0;
  });
});

onUnmounted(() => {
  [
    unlistenFileDrop,
    unlistenFileDropHover,
    unlistenFileDropCanceled,
    unlistenBlur
  ].forEach((unlisten) => {
    if (unlisten) {
      unlisten();
    }
  });
});
</script>
<template>
  <VBtn @click="SelectFile">
    <VIcon :icon="mdiFolderOpen" />
    选择文件
  </VBtn>
  <Teleport v-if="hover" to="body">
    <div class="hover-indication">
      <div class="hover-indication-box">{{ hover }} file(s) hovering.</div>
    </div>
  </Teleport>
</template>
<style scoped>
.hover-indication {
  background-color: rgb(255 255 255 / 50%);
  position: fixed;
  z-index: 999;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
}

.hover-indication-box {
  color: red;
  border-radius: 1rem;
  border: 5px red dashed;
  margin: 1rem;
  height: calc(100% - 2rem);
  width: calc(100% - 2rem);
}
</style>
