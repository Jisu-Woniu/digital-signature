<script setup lang="ts">
import { ref } from "vue";
import { VBtn, VChip, VContainer, VForm, VTextField } from "vuetify/components";
import { computed } from "vue";
import { mdiCheck } from "@mdi/js";
import FilesSelector from "@/components/FilesSelector.vue";
import FolderOpen from "~icons/ic/twotone-folder-open";

const files = ref<string[]>([]);
const fileNames = computed(() =>
  files.value.map((file) => {
    const pathComponents = file.split(/[\\/]/);
    return pathComponents[pathComponents.length - 1];
  }),
);
const rules = {
  required: (value: string | undefined) => !!value?.trim() || "必填",
};
const valid = ref(false);
const submit = () => {
  if (valid.value) alert("submit");
};
</script>

<template>
  <VContainer fluid>
    <VForm
      v-model="valid"
      validate-on="input"
      fast-fail
      @submit.prevent="submit"
    >
      <FilesSelector v-slot="{ selectFiles }" v-model="files">
        <VTextField
          v-model="files"
          :rules="[rules.required]"
          label="待签名文件"
          clearable
          readonly
          class="input-hidden"
          @click:clear="files = []"
          @click:control="selectFiles"
        >
          <VChip v-for="fileName in fileNames" :key="fileName">
            {{ fileName }}
          </VChip>
          <template #append>
            <VBtn :icon="FolderOpen" @click="selectFiles" />
          </template>
        </VTextField>
      </FilesSelector>
      <VBtn :prepend-icon="mdiCheck" color="#4CAF50" type="submit"> 提交 </VBtn>
    </VForm>
    <ul v-if="files">
      <li v-for="file in files" :key="file">{{ file }}</li>
    </ul>
  </VContainer>
</template>
<style scoped>
.input-hidden :deep(input) {
  display: none;
}
</style>
