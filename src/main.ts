import App from "@/App.vue";
import "@fontsource/noto-sans-sc/chinese-simplified-300.css";
import "@fontsource/noto-sans-sc/chinese-simplified-400.css";
import "@fontsource/roboto/latin-300.css";
import "@fontsource/roboto/latin-300-italic.css";
import "@fontsource/roboto/latin-400.css";
import "@fontsource/roboto/latin-400-italic.css";
import { createApp } from "vue";
import { createVuetify } from "vuetify";
import { aliases, mdi } from "vuetify/iconsets/mdi-svg";
import "vuetify/styles";

import "./styles.css";

const vuetify = createVuetify({
  icons: { aliases, defaultSet: "mdi", sets: { mdi } },
});

createApp(App).use(vuetify).mount("#app");
