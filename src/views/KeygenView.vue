<script setup lang="ts">
import { ref } from "vue";
import FileSelector from "@/components/FileSelector.vue";
import { VContainer } from "vuetify/components";
const file = ref<string>();
</script>

<template>
  <VContainer fluid>
    <h1 class="h1 pa-2">密钥管理</h1>
    <FileSelector v-model="file" />
    <div v-if="file">
      {{ file }}
    </div>
  </VContainer>
</template>
<style scoped>
.h1 {
  font-size: 6rem;
  font-weight: 300;
  line-height: 6rem;
  letter-spacing: -0.0156em;
  text-transform: none;
}
</style>
