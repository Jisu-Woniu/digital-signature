<script setup lang="ts">
import { ref } from "vue";
import { VBtn, VContainer, VForm, VTextField } from "vuetify/components";
import { mdiCheck } from "@mdi/js";
import { message } from "@tauri-apps/plugin-dialog";
import FileSelector from "@/components/FileSelector.vue";
import { generateKeyPair } from "@/command";
import FolderOpen from "~icons/ic/twotone-folder-open";
const name = ref("");
const email = ref("");
const password = ref("");
const valid = ref<boolean | null>(null);
const file = ref<string>();
const rules = {
  required: (value: string | undefined) => !!value?.trim() || "必填",
  email: (value: string) =>
    /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/.test(
      value,
    ) || "非法邮件地址",
};

const generate = async () => {
  try {
    if (valid.value) {
      const paths = await generateKeyPair(
        name.value,
        email.value,
        password.value,
        file.value!,
      );
      await message(
        "生成成功\n" +
          `您的私钥路径为：${paths.secretKeyPath}\n` +
          `公钥路径为：${paths.publicKeyPath}`,
      );
    }
  } catch (x) {
    await message("生成失败，发生了以下错误：\n" + x);
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
      <VTextField v-model="password" label="密码（可选）" type="password" />
      <FileSelector v-slot="{ selectFile }" v-model="file" directory>
        <VTextField
          v-model="file"
          label="保存位置"
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

      <VBtn :prepend-icon="mdiCheck" color="#4CAF50" type="submit"> 提交 </VBtn>
    </VForm>
  </VContainer>
</template>
