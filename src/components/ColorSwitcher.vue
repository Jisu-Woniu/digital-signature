<script setup lang="ts">
import { useColorMode, useCycleList } from "@vueuse/core";
import { watchEffect } from "vue";
import { useTheme } from "vuetify";
import { VBtn } from "vuetify/components";
import AutoLightMode from "~icons/ic/twotone-brightness-auto";
import LightMode from "~icons/ic/twotone-light-mode";
import DarkMode from "~icons/ic/twotone-nightlight";

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
