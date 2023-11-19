<script setup lang="ts">
import { ref } from "vue";
import FileSelector from "@/components/FileSelector.vue";
import { VContainer } from "vuetify/components";

defineProps<{ activated: boolean }>();

const file = ref<string>();
</script>

<template>
  <VContainer fluid>
    <h1 class="pa-2">签名</h1>
    <FileSelector v-if="activated" v-model="file" />
    <div v-if="file">
      {{ file }}
    </div>
  </VContainer>
</template>
