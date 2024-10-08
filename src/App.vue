<script setup lang="ts">
import {
  VApp,
  VAppBar,
  VMain,
  VTab,
  VTabs,
  VWindow,
  VWindowItem,
} from "vuetify/components";
import KeygenView from "@/views/KeygenView.vue";
import SignView from "@/views/SignView.vue";
import ValidateView from "@/views/ValidateView.vue";
import { mdiFileCheck, mdiFileKey, mdiKeyChain } from "@mdi/js";
import { useSessionStorage } from "@vueuse/core";
import ColorSwitcher from "@/components/ColorSwitcher.vue";

enum Tab {
  sign = 0,
  validate = 1,
  keygen = 2,
}

const tab = useSessionStorage("tab", Tab.sign);
</script>

<template>
  <VApp>
    <VAppBar title="签名工具">
      <template #append>
        <ColorSwitcher />
      </template>
      <template #extension>
        <VTabs v-model="tab" color="primary" align-tabs="center">
          <VTab :value="Tab.sign" :prepend-icon="mdiFileKey"> 签名 </VTab>
          <VTab :value="Tab.validate" :prepend-icon="mdiFileCheck"> 校验 </VTab>
          <VTab :value="Tab.keygen" :prepend-icon="mdiKeyChain">
            密钥生成
          </VTab>
        </VTabs>
      </template>
    </VAppBar>
    <VMain>
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
    </VMain>
  </VApp>
</template>
