<script setup lang="ts">
import { ref } from "vue";
import { VContainer } from "vuetify/components";
import FilesSelector from "@/components/FilesSelector.vue";

const files = ref<string[]>([]);
</script>

<template>
  <VContainer fluid>
    <FilesSelector v-model="files" />
    <ul v-if="files">
      <li v-for="file in files" :key="file">{{ file }}</li>
    </ul>
  </VContainer>
</template>
