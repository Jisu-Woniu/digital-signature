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

import AutoLightMode from "~icons/ic/twotone-brightness-auto";
import LightMode from "~icons/ic/twotone-light-mode";
import DarkMode from "~icons/ic/twotone-nightlight";
import { useTheme } from "vuetify";
import { useColorMode, useCycleList } from "@vueuse/core";
import { watchEffect } from "vue";

const enum Tab {
  sign,
  validate,
  keygen,
}

const tab = ref(Tab.sign);

const theme = useTheme();

const { store: colorStore } = useColorMode({
  onChanged: (mode) => {
    theme.global.name.value = mode;
  },
});

const { state: colorState, next: nextColor } = useCycleList<
  "auto" | "light" | "dark"
>(["auto", "light", "dark"], {
  initialValue: colorStore.value,
});

watchEffect(() => {
  colorStore.value = colorState.value;
});
</script>

<template>
  <VApp>
    <VToolbar title="签名工具">
      <VBtn
        :icon="
          colorState === 'auto'
            ? AutoLightMode
            : colorState === 'light'
              ? LightMode
              : DarkMode
        "
        @click="nextColor()"
      />
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
