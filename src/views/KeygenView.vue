<script setup lang="ts">
import { ref } from "vue";
import FileSelector from "@/components/FileSelector.vue";
import { VBtn, VContainer, VTextField } from "vuetify/components";
import { mdiCheck } from "@mdi/js";
const name = ref("");
const email = ref("");

const file = ref<string>();
const rules = {
  required: (value: string) => !!value.trim() || "必填",
  email: (value: string) => {
    const regex =
      /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/;

    console.log(`Email: ${value}`);
    return regex.test(value) || "非法邮件地址";
  },
};
</script>

<template>
  <VContainer fluid>
    <h1 class="pa-2">密钥管理</h1>
    <VTextField v-model="name" label="姓名" :rules="[rules.required]" />
    <VTextField
      v-model="email"
      label="邮箱"
      :rules="[rules.required, rules.email]"
    />
    <FileSelector v-model="file" directory />
    <div v-if="file">
      {{ file }}
    </div>
    <VBtn :prepend-icon="mdiCheck" color="#4CAF50">提交</VBtn>
  </VContainer>
</template>
