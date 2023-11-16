<script setup lang="ts">
import { ref } from "vue";
import {
  VApp,
  VBtn,
  VTab,
  VTabs,
  VToolbar,
  VWindow,
  VWindowItem,
} from "vuetify/components";
import { mdiFileKey, mdiFileCheck, mdiKeyChain } from "@mdi/js";
import SignView from "./views/SignView.vue";
import ValidateView from "./views/ValidateView.vue";
import KeygenView from "./views/KeygenView.vue";
import LightMode from "~icons/ic/twotone-light-mode";
import DarkMode from "~icons/ic/twotone-nightlight";
import { useTheme } from "vuetify";
import { useDark, useToggle } from "@vueuse/core";

const enum Tab {
  sign,
  validate,
  keygen,
}

const tab = ref(Tab.sign);

const theme = useTheme();

const isDark = useDark({
  onChanged: (dark) => {
    theme.global.name.value = dark ? "dark" : "light";
  },
});

const toggleDark = useToggle(isDark);
</script>

<template>
  <VApp>
    <VToolbar title="签名工具">
      <VBtn :icon="isDark ? DarkMode : LightMode" @click="toggleDark()" />
      <template #extension>
        <VTabs v-model="tab" color="primary" align-tabs="center">
          <VTab :value="Tab.sign" :prepend-icon="mdiFileKey"> 签名 </VTab>
          <VTab :value="Tab.validate" :prepend-icon="mdiFileCheck"> 校验 </VTab>
          <VTab :value="Tab.keygen" :prepend-icon="mdiKeyChain">
            密钥管理
          </VTab>
        </VTabs>
      </template>
    </VToolbar>

    <VWindow v-model="tab">
      <VWindowItem :value="Tab.sign">
        <SignView />
      </VWindowItem>
      <VWindowItem :value="Tab.validate">
        <ValidateView />
      </VWindowItem>
      <VWindowItem :value="Tab.keygen">
        <KeygenView />
      </VWindowItem>
    </VWindow>
  </VApp>
</template>
