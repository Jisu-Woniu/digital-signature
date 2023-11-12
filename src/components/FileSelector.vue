<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { message, open } from "@tauri-apps/api/dialog";
import { listen, TauriEvent, type UnlistenFn } from "@tauri-apps/api/event";

import { VBtn, VCard, VDialog, VImg } from "vuetify/components";
import { mdiFolderOpen } from "@mdi/js";
import UploadFile from "@material-design-icons/svg/two-tone/upload_file.svg?url";

const props = defineProps<{ modelValue: string | undefined }>();

const emits = defineEmits<{
  (event: "update:modelValue", value: string | undefined): void;
}>();

const files = computed<string | undefined>({
  get: () => props.modelValue,
  set: (value) => {
    emits("update:modelValue", value);
  },
});

const SelectFile = async () => {
  const selected = (await open({ multiple: false })) as string | null;
  files.value = selected ?? undefined;
};

const hover = ref(false);

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
          `You can only drop one file here.\nYou have selected: ${JSON.stringify(
            e.payload,
          )}`,
        );
        return;
      }
      files.value = e.payload[0];
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
    () => {
      hover.value = false;
    },
  );
  unlistenBlur = await listen<void>(TauriEvent.WINDOW_BLUR, () => {
    console.log("Blur");
    hover.value = false;
  });
});

onUnmounted(() => {
  [
    unlistenFileDrop,
    unlistenFileDropHover,
    unlistenFileDropCanceled,
    unlistenBlur,
  ].forEach((unlisten) => {
    if (unlisten) {
      unlisten();
    }
  });
});
</script>
<template>
  <VBtn :prepend-icon="mdiFolderOpen" @click="SelectFile"> 选择文件 </VBtn>
  <VDialog v-model="hover" height="100%">
    <VCard class="hover-indication-box">
      <VImg :src="UploadFile" class="hover-indication-icon" />
      <div class="hover-indication-text text-blue-darken-3 mt-3">
        拖拽至此处
      </div>
    </VCard>
  </VDialog>
  <!-- </Teleport> -->
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
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;
}

.hover-indication-icon {
  height: 5em;
  width: 5em;
  flex-grow: initial;
}

.hover-indication-text {
  font-size: 2em;
}
</style>
