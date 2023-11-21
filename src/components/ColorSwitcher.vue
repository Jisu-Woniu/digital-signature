<script setup lang="ts">
import { useTheme } from "vuetify";
import { watchEffect } from "vue";
import { VBtn } from "vuetify/components";
import { useColorMode, useCycleList } from "@vueuse/core";
import AutoLightMode from "~icons/ic/twotone-brightness-auto";
import DarkMode from "~icons/ic/twotone-nightlight";
import LightMode from "~icons/ic/twotone-light-mode";

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
</template>
