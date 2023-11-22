<script setup lang="ts">
import { ref } from "vue";
import { VBtn, VContainer, VForm, VTextField } from "vuetify/components";
import { mdiCheck } from "@mdi/js";
import { message } from "@tauri-apps/api/dialog";
import FileSelector from "@/components/FileSelector.vue";
import { generateKeyPair } from "@/command";
import FolderOpen from "~icons/ic/twotone-folder-open";

const name = ref("");
const email = ref("");
const valid = ref<boolean | null>(null);
const file = ref<string>();
const rules = {
  required: (value: string | undefined) => !!value?.trim() || "必填",
  email: (value: string) => {
    const regex =
      /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/;

    console.log(`Email: ${value}`);
    return regex.test(value) || "非法邮件地址";
  },
};

const generate = async () => {
  try {
    if (valid.value) {
      await generateKeyPair(name.value, email.value, file.value!);
      await message("生成成功");
    }
  } catch (x) {
    await message("生成失败\n发生如下错误：\n" + x);
  }
};
</script>

<template>
  <VContainer fluid>
    <VForm
      v-model="valid"
      validate-on="blur"
      fast-fail
      @submit.prevent="generate"
    >
      <VTextField v-model="name" label="姓名" :rules="[rules.required]" />
      <VTextField
        v-model="email"
        label="邮箱"
        :rules="[rules.required, rules.email]"
      />

      <VTextField
        v-model="file"
        label="保存位置"
        clearable
        readonly
        :rules="[rules.required]"
      >
        <template #append>
          <FileSelector v-slot="{ selectFile }" v-model="file" directory>
            <VBtn :icon="FolderOpen" @click="selectFile" />
          </FileSelector>
        </template>
      </VTextField>
      <VBtn :prepend-icon="mdiCheck" color="#4CAF50" type="submit"> 提交 </VBtn>
    </VForm>
  </VContainer>
</template>
