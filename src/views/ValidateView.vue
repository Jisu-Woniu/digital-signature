<script setup lang="ts">
import {
  VBtn,
  VCard,
  VChip,
  VContainer,
  VForm,
  VStepper,
  VStepperActions,
  VTextField,
} from "vuetify/components";
import { computed, reactive, ref } from "vue";
import FileSelector from "@/components/FileSelector.vue";
import FilesSelector from "@/components/FilesSelector.vue";
import { verifySignatures } from "@/command";
import FolderOpen from "~icons/ic/twotone-folder-open";
import { message } from "@tauri-apps/plugin-dialog";

const rules = {
  required: (value: string | string[] | undefined) => !!value?.length || "必填",
};

const items = ref([
  { title: "选择待校验签名", value: 1 },
  { title: "选择公钥文件", value: 2 },
]);

const valid = ref(false);

const step = ref(1);

const data = reactive({
  signaturePaths: new Array<string>(),
  keyPath: "",
});

const fileNames = computed(() =>
  data.signaturePaths.map((file) => {
    const pathComponents = file.split(/[\\/]/);
    return pathComponents[pathComponents.length - 1];
  }),
);

const back = () => {
  if (step.value > 1) {
    step.value -= 1;
  }
};

const submit = async () => {
  if (valid.value) {
    if (step.value === items.value.length) {
      try {
        const results = await verifySignatures(
          data.signaturePaths,
          data.keyPath,
        );
        await message(
          "校验成功\n" +
            Object.keys(results)
              .map((key) => `${key}: ${results[key] ? "✔️" : "❌"}`)
              .join("\n"),
        );
      } catch (error) {
        await message("校验失败，发生了以下错误：\n" + JSON.stringify(error));
      }
    } else {
      step.value += 1;
    }
  }
};
</script>
<template>
  <!-- eslint-disable vue/valid-v-slot -->

  <VContainer fluid>
    <VForm v-model="valid" validate-on="input" @submit.prevent="submit">
      <VStepper v-model="step" :items="items">
        <template #item.1>
          <VCard v-if="step === 1" title="请选择待校验签名" flat>
            <FilesSelector
              v-slot="{ selectFiles }"
              v-model="data.signaturePaths"
            >
              <VTextField
                v-model="data.signaturePaths"
                :rules="[rules.required]"
                label="待校验签名"
                clearable
                readonly
                class="input-hidden"
                @click:clear="data.signaturePaths = []"
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
          </VCard>
        </template>
        <template #item.2>
          <VCard v-if="step === 2" title="请选择公钥文件" flat>
            <FileSelector v-slot="{ selectFile }" v-model="data.keyPath">
              <VTextField
                v-model="data.keyPath"
                label="公钥文件"
                clearable
                readonly
                :rules="[rules.required]"
                @click:control="selectFile"
              >
                <template #append>
                  <VBtn :icon="FolderOpen" @click="selectFile" />
                </template>
              </VTextField>
            </FileSelector>
          </VCard>
        </template>

        <template #actions>
          <VStepperActions>
            <template #prev>
              <VBtn :disabled="step === 1" @click="back">Previous</VBtn>
            </template>
            <template #next>
              <VBtn type="submit" :disabled="false">
                {{ step === items.length ? "Submit" : "Next" }}
              </VBtn>
            </template>
          </VStepperActions>
        </template>
      </VStepper>
    </VForm>
  </VContainer>
</template>
<style scoped>
.input-hidden :deep(input) {
  display: none;
}
</style>
