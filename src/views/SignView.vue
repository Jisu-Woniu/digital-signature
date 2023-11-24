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
import { signFiles } from "@/command";
import FolderOpen from "~icons/ic/twotone-folder-open";

const rules = {
  required: (value: string | string[] | undefined) =>
    !!value?.length || "Required",
};

const items = ref([
  { title: "选择待签名文件", value: 1 },
  { title: "选择私钥文件", value: 2 },
]);

const valid = ref(false);

const step = ref(1);

const data = reactive({
  filePaths: new Array<string>(),
  keyPath: "",
  passwd: "",
});

const fileNames = computed(() =>
  data.filePaths.map((file) => {
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
  console.log("Received submit.");
  if (valid.value) {
    if (step.value === items.value.length) {
      await signFiles(data.filePaths, data.keyPath, data.passwd);
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
          <VCard v-if="step === 1" title="请选择待签名文件" flat>
            <FilesSelector v-slot="{ selectFiles }" v-model="data.filePaths">
              <VTextField
                v-model="data.filePaths"
                :rules="[rules.required]"
                label="待签名文件"
                clearable
                readonly
                class="input-hidden"
                @click:clear="data.filePaths = []"
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
          <VCard v-if="step === 2" title="请选择私钥文件" flat>
            <FileSelector v-slot="{ selectFile }" v-model="data.keyPath">
              <VTextField
                v-model="data.keyPath"
                label="私钥文件"
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
            <VTextField
              v-model="data.passwd"
              title="私钥密码（可选）"
              type="password"
            />
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
