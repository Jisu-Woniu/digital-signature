<script setup lang="ts">
import {
  VApp,
  VTab,
  VTabs,
  VToolbar,
  VWindow,
  VWindowItem,
} from "vuetify/components";
import { mdiFileKey, mdiFileCheck, mdiKeyChain } from "@mdi/js";
import SignView from "@/views/SignView.vue";
import ValidateView from "@/views/ValidateView.vue";
import KeygenView from "@/views/KeygenView.vue";
import ColorSwitcher from "@/components/ColorSwitcher.vue";
import { useSessionStorage } from "@vueuse/core";

const enum Tab {
  sign,
  validate,
  keygen,
}

const tab = useSessionStorage("tab", Tab.sign);
</script>

<template>
  <VApp>
    <VToolbar title="签名工具">
      <ColorSwitcher />
      <template #extension>
        <VTabs v-model="tab" color="primary" align-tabs="center">
          <VTab :value="Tab.sign" :prepend-icon="mdiFileKey"> 签名 </VTab>
          <VTab :value="Tab.validate" :prepend-icon="mdiFileCheck"> 校验 </VTab>
          <VTab :value="Tab.keygen" :prepend-icon="mdiKeyChain">
            密钥生成
          </VTab>
        </VTabs>
      </template>
    </VToolbar>

    <VWindow v-model="tab">
      <VWindowItem :value="Tab.sign">
        <SignView v-if="tab === Tab.sign" />
      </VWindowItem>
      <VWindowItem :value="Tab.validate">
        <ValidateView v-if="tab === Tab.validate" />
      </VWindowItem>
      <VWindowItem :value="Tab.keygen">
        <KeygenView v-if="tab === Tab.keygen" />
      </VWindowItem>
    </VWindow>
  </VApp>
</template>
