import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

import "vuetify/styles";
import { createVuetify } from "vuetify";

import { aliases, mdi } from "vuetify/iconsets/mdi-svg";
import "@fontsource/roboto/latin.css";
import "@fontsource/noto-sans-sc";

const vuetify = createVuetify({
  icons: { defaultSet: "mdi", aliases, sets: { mdi } },
});

createApp(App).use(vuetify).mount("#app");
