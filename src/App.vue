<script setup lang="ts">
import { ref } from "vue";
import FileSelector from "@/components/FileSelector.vue";

const onUpdate = (e: Event) => {
  console.log((e.target as HTMLInputElement).value);
};

const file = ref<string>();
</script>

<template>
  <FileSelector v-model="file" />
  <div v-if="file">
    {{ file }}
  </div>
</template>
