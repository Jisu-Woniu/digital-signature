<script setup lang="ts">
import { ref } from "vue";
import FileSelector from "@/components/FileSelector.vue";
import {
  VContainer,
  VTab,
  VTabs,
  VToolbar,
  VWindow,
  VWindowItem,
} from "vuetify/components";
import { mdiFileKey, mdiFileCheck, mdiKeyChain } from "@mdi/js";

const enum Tab {
  sign,
  validate,
  keygen,
}

const tab = ref(Tab.sign);

const file = ref<string>();
</script>

<template>
  <VToolbar title="签名工具">
    <template #extension>
      <VTabs v-model="tab" color="primary" align-tabs="center">
        <VTab :value="Tab.sign" :prepend-icon="mdiFileKey"> 签名 </VTab>
        <VTab :value="Tab.validate" :prepend-icon="mdiFileCheck"> 校验 </VTab>
        <VTab :value="Tab.keygen" :prepend-icon="mdiKeyChain"> 密钥管理 </VTab>
      </VTabs>
    </template>
  </VToolbar>

  <VWindow v-model="tab">
    <VWindowItem :value="Tab.sign">
      <VContainer fluid>
        <div class="text-h1 pa-2">Sign</div>
        <FileSelector v-model="file" />
        <div v-if="file">
          {{ file }}
        </div>
      </VContainer>
    </VWindowItem>
    <VWindowItem :value="Tab.validate">
      <VContainer>
        <h1>Validate</h1>
        <FileSelector v-model="file" />
        <div v-if="file">
          {{ file }}
        </div>
      </VContainer>
    </VWindowItem>
    <VWindowItem :value="Tab.keygen">
      <VContainer>
        <h1>Keygen</h1>
        <FileSelector v-model="file" />
        <div v-if="file">
          {{ file }}
        </div>
      </VContainer>
    </VWindowItem>
  </VWindow>
</template>
