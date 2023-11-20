import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

import "vuetify/styles";
import { createVuetify } from "vuetify";

import { aliases, mdi } from "vuetify/iconsets/mdi-svg";
import "@fontsource/roboto/latin-300.css";
import "@fontsource/roboto/latin-400.css";
import "@fontsource/roboto/latin-300-italic.css";
import "@fontsource/roboto/latin-400-italic.css";
import "@fontsource/noto-sans-sc/chinese-simplified-300.css";
import "@fontsource/noto-sans-sc/chinese-simplified-400.css";

const vuetify = createVuetify({
  icons: { defaultSet: "mdi", aliases, sets: { mdi } },
});

createApp(App).use(vuetify).mount("#app");
