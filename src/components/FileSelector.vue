<script setup lang="ts">
import { computed } from "vue";
import { open } from "@tauri-apps/api/dialog";

const emits = defineEmits<{
  (event: "file-selected", value: string[]): void;
}>();

const SelectFile = async () => {
  const files = (await open({ multiple: true })) as string[];
  console.log(files);
  emits("file-selected", files);
};
</script>
<template>
  <button @click="SelectFile">选择文件</button>
</template>
