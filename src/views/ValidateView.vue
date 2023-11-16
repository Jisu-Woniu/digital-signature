<script setup lang="ts">
import { ref } from "vue";
import FileSelector from "@/components/FileSelector.vue";
import { VContainer } from "vuetify/components";
const file = ref<string>();
</script>

<template>
  <VContainer fluid>
    <h1 class="pa-2">校验</h1>
    <FileSelector v-model="file" />
    <div v-if="file">
      {{ file }}
    </div>
  </VContainer>
</template>
